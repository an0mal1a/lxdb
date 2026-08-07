use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::Instant,
};

use lxdb_compiler::{builder::Builder, compiler::Compiler};
use lxdb_storage::DatasetReader;

use crate::{
    cache::{atomic_write, cache_language_directory},
    config::BuildOptions,
    error::DictionaryError,
    language::find_language,
    manifest,
    model::{LexicalEntry, LexicalRelationKind, SourceKind},
    report::BuildReport,
    source,
};

pub fn build(options: &BuildOptions) -> Result<BuildReport, DictionaryError> {
    let language = find_language(&options.language)
        .ok_or_else(|| DictionaryError::UnsupportedLanguage(options.language.clone()))?;
    validate_options(options)?;
    let mut effective_options = options.clone();
    let mut report = BuildReport::default();
    let mut entries = Vec::new();
    let started = Instant::now();
    let resolved = resolve_sources(&effective_options, language.iso_639_1)?;
    effective_options.sources.kaikki = resolved.kaikki.is_some();
    effective_options.sources.hunspell = resolved.hunspell.is_some();
    effective_options.sources.wordnet = resolved.wordnet.is_some();
    effective_options.sources.frequency = resolved.frequency.is_some();
    // There is no configured local embedding provider in this release.
    effective_options.sources.embeddings = false;
    let mut invalid = 0;
    if let Some(path) = &resolved.kaikki {
        let timer = Instant::now();
        report.entries_read += source::parse_kaikki(
            path,
            language.iso_639_1,
            resolved.snapshot,
            &mut entries,
            &mut invalid,
            effective_options.effective_limit(),
        )?;
        report.phases.insert("extract_kaikki".to_owned(), timer.elapsed());
    }
    if let Some(path) = &resolved.hunspell {
        let timer = Instant::now();
        report.entries_read +=
            source::parse_hunspell(path, language.iso_639_1, resolved.snapshot, &mut entries)?;
        report.phases.insert("extract_hunspell".to_owned(), timer.elapsed());
    }
    if let Some(path) = &resolved.wordnet {
        let timer = Instant::now();
        report.entries_read +=
            source::parse_wordnet_lmf(path, language.iso_639_1, resolved.snapshot, &mut entries)?;
        report.phases.insert("extract_wordnet".to_owned(), timer.elapsed());
    }
    if let Some(path) = &resolved.frequency {
        let timer = Instant::now();
        source::parse_frequency(path, &mut entries)?;
        report.phases.insert("extract_frequency".to_owned(), timer.elapsed());
    }
    report.entries_invalid = invalid;
    let timer = Instant::now();
    let mut rejected = Vec::new();
    entries.retain_mut(|entry| match normalize_and_accept(entry, &effective_options) {
        Ok(()) => true,
        Err(reason) => {
            rejected.push((entry.canonical.clone(), reason));
            false
        }
    });
    report.entries_rejected = rejected.len() as u64;
    report.phases.insert("normalize_filter".to_owned(), timer.elapsed());
    let timer = Instant::now();
    let mut entries = merge(entries, &mut report);
    if let Some(limit) = effective_options.effective_limit() {
        entries.truncate(limit);
    }
    compute_quality(&mut entries, &mut report);
    report.phases.insert("merge_quality".to_owned(), timer.elapsed());
    report.entries_accepted = entries.len() as u64;
    report.unique_lemmas = entries.len() as u64;
    report.surface_forms = entries.iter().map(|entry| entry.forms.len() as u64).sum();
    report.senses = entries.iter().map(|entry| entry.senses.len() as u64).sum();
    let timer = Instant::now();
    let source_text =
        render_lx(&entries, effective_options.profile.max_relations_per_token(), &mut report);
    report.phases.insert("graph".to_owned(), timer.elapsed());
    fs::create_dir_all(&effective_options.output_dir)?;
    let lx_path = effective_options.output_dir.join("dictionary.lx");
    atomic_write(&lx_path, source_text.as_bytes())?;
    let output = effective_options.output_dir.join("dictionary.lxdb");
    let temporary = effective_options.output_dir.join("dictionary.lxdb.tmp");
    let timer = Instant::now();
    Compiler::new()
        .compile(
            Builder::new()
                .language(language.iso_639_1)
                .dataset_name(language.display_name)
                .input(lx_path.to_string_lossy().into_owned())
                .output(temporary.to_string_lossy().into_owned())
                .build(),
        )
        .map_err(DictionaryError::Compile)?;
    if output.exists() {
        fs::remove_file(&output)?;
    }
    fs::rename(&temporary, &output)?;
    report.phases.insert("compile".to_owned(), timer.elapsed());
    let timer = Instant::now();
    let dataset = DatasetReader::new().open(&output).map_err(DictionaryError::Validate)?;
    if dataset.token_count() != count_lx_tokens(&source_text) {
        return Err(DictionaryError::Manifest(output));
    }
    report.phases.insert("validate".to_owned(), timer.elapsed());
    let dataset_hash = fnv1a64(&fs::read(&output)?);
    let timer = Instant::now();
    atomic_write(
        &effective_options.output_dir.join("manifest.json"),
        manifest::render_manifest(language, &effective_options, &report, dataset_hash).as_bytes(),
    )?;
    atomic_write(
        &effective_options.output_dir.join("build-report.json"),
        manifest::render_report(&report).as_bytes(),
    )?;
    atomic_write(
        &effective_options.output_dir.join("ATTRIBUTION.md"),
        manifest::render_attribution(&effective_options).as_bytes(),
    )?;
    if effective_options.emit_rejected {
        atomic_write(
            &effective_options.output_dir.join("rejected-entries.jsonl.zst"),
            &zstd_raw_frame(render_rejected(&rejected).as_bytes())?,
        )?;
    }
    report.phases.insert("artifacts".to_owned(), timer.elapsed());
    report.phases.insert("total".to_owned(), started.elapsed());
    update_cache_manifest(language.iso_639_1, &effective_options, &resolved.root)?;
    if effective_options.emit_source.is_some() || effective_options.keep_intermediate {
        let destination = effective_options.emit_source.as_ref().unwrap_or(&lx_path);
        if destination != &lx_path {
            atomic_write(destination, source_text.as_bytes())?;
        }
    } else {
        fs::remove_file(&lx_path)?;
    }
    Ok(report)
}

pub fn update(language: &str, cache_dir: &Path) -> Result<PathBuf, DictionaryError> {
    find_language(language)
        .ok_or_else(|| DictionaryError::UnsupportedLanguage(language.to_owned()))?;
    let mut options = BuildOptions::new(language, PathBuf::from("."));
    options.profile = crate::DictionaryProfile::Game;
    options.cache_dir = cache_dir.to_path_buf();
    options.refresh = true;
    let resolved = resolve_sources(&options, language)?;
    update_cache_manifest(language, &options, &resolved.root)?;
    Ok(resolved.root.join("cache-manifest.json"))
}

pub fn inspect_manifest(path: &Path) -> Result<String, DictionaryError> {
    let language = manifest::manifest_language(path)
        .ok_or_else(|| DictionaryError::Manifest(path.to_path_buf()))?;
    let contents = fs::read_to_string(path)?;
    let profile = field(&contents, "profile").unwrap_or_else(|| "unknown".to_owned());
    let hash = field(&contents, "dataset_hash_fnv1a64").unwrap_or_else(|| "unknown".to_owned());
    Ok(format!(
        "Dictionary manifest: {}\nLanguage: {language}\nProfile: {profile}\nDataset hash (FNV-1a 64): {hash}\n",
        path.display()
    ))
}

struct ResolvedSources {
    root: PathBuf,
    snapshot: &'static str,
    kaikki: Option<PathBuf>,
    hunspell: Option<PathBuf>,
    wordnet: Option<PathBuf>,
    frequency: Option<PathBuf>,
}

fn resolve_sources(
    options: &BuildOptions,
    language: &str,
) -> Result<ResolvedSources, DictionaryError> {
    if let Some(root) = &options.fixture_dir {
        return Ok(ResolvedSources {
            root: root.clone(),
            snapshot: "fixture",
            kaikki: options
                .sources
                .kaikki
                .then(|| root.join(format!("kaikki-{language}-small.jsonl"))),
            hunspell: options
                .sources
                .hunspell
                .then(|| root.join(format!("hunspell-{language}-small.dic"))),
            wordnet: options
                .sources
                .wordnet
                .then(|| root.join(format!("wordnet-{language}-small.xml"))),
            frequency: options
                .sources
                .frequency
                .then(|| root.join(format!("frequency-{language}-small.tsv"))),
        });
    }

    let root = cache_language_directory(&options.cache_dir, language);
    fs::create_dir_all(&root)?;
    let kaikki = options
        .sources
        .kaikki
        .then(|| ensure_cached_source(&root, language, "kaikki", options))
        .transpose()?;
    let hunspell = options
        .sources
        .hunspell
        .then(|| ensure_cached_source(&root, language, "hunspell", options))
        .transpose()?;

    // WordNet and frequency are optional local enrichments until their snapshot
    // providers are configured. Kaikki remains sufficient for a large graph.
    let wordnet_path = root.join(format!("wordnet-{language}.xml"));
    let frequency_path = root.join(format!("frequency-{language}.tsv"));
    Ok(ResolvedSources {
        root,
        snapshot: "cache",
        kaikki,
        hunspell,
        wordnet: options.sources.wordnet.then_some(wordnet_path).filter(|path| path.exists()),
        frequency: options.sources.frequency.then_some(frequency_path).filter(|path| path.exists()),
    })
}

fn ensure_cached_source(
    root: &Path,
    language: &str,
    source: &'static str,
    options: &BuildOptions,
) -> Result<PathBuf, DictionaryError> {
    let filename = match source {
        "kaikki" => format!("kaikki-{language}.jsonl"),
        "hunspell" => format!("hunspell-{language}.dic"),
        _ => unreachable!("only known source names reach ensure_cached_source"),
    };
    let target = root.join(filename);
    if target.exists() && !options.refresh {
        return Ok(target);
    }
    if options.offline {
        return Err(DictionaryError::OfflineCacheMiss { source, path: target });
    }
    let url = source_url(language, source)
        .ok_or_else(|| DictionaryError::MissingSource { source, path: target.clone() })?;
    download_to_cache(source, url, &target)?;
    Ok(target)
}

fn source_url(language: &str, source: &str) -> Option<&'static str> {
    match (language, source) {
        ("es", "kaikki") => {
            Some("https://kaikki.org/dictionary/Spanish/kaikki.org-dictionary-Spanish.jsonl")
        }
        ("es", "hunspell") => {
            Some("https://cgit.freedesktop.org/libreoffice/dictionaries/plain/es/es_ES.dic")
        }
        _ => None,
    }
}

fn download_to_cache(
    source: &'static str,
    url: &'static str,
    target: &Path,
) -> Result<(), DictionaryError> {
    let temporary = target.with_extension(format!(
        "{}.download",
        target.extension().and_then(|value| value.to_str()).unwrap_or("tmp")
    ));
    let status = Command::new("curl")
        .args([
            "--fail",
            "--location",
            "--silent",
            "--show-error",
            "--retry",
            "3",
            "--connect-timeout",
            "30",
            "--output",
        ])
        .arg(&temporary)
        .arg(url)
        .status()
        .map_err(|_| DictionaryError::SourceDownloadFailed { source, url, status: None })?;
    if !status.success() {
        let _ = fs::remove_file(&temporary);
        return Err(DictionaryError::SourceDownloadFailed { source, url, status: status.code() });
    }
    if target.exists() {
        fs::remove_file(target)?;
    }
    fs::rename(temporary, target)?;
    Ok(())
}
fn validate_options(options: &BuildOptions) -> Result<(), DictionaryError> {
    if options.limit == Some(0) {
        return Err(DictionaryError::InvalidConfiguration {
            path: PathBuf::from("--limit"),
            message: "must be greater than zero".to_owned(),
        });
    }
    if !options.sources.kaikki && !options.sources.hunspell && !options.sources.wordnet {
        return Err(DictionaryError::InvalidConfiguration {
            path: PathBuf::from("sources"),
            message: "at least one lexical source must be enabled".to_owned(),
        });
    }
    if let Some(path) = &options.config_path {
        let contents = fs::read_to_string(path).map_err(|error| {
            DictionaryError::InvalidConfiguration { path: path.clone(), message: error.to_string() }
        })?;
        let configured_language = toml_string(&contents, "language").ok_or_else(|| {
            DictionaryError::InvalidConfiguration {
                path: path.clone(),
                message: "missing top-level language".to_owned(),
            }
        })?;
        if configured_language != options.language {
            return Err(DictionaryError::InvalidConfiguration {
                path: path.clone(),
                message: format!(
                    "language '{configured_language}' does not match requested '{}'",
                    options.language
                ),
            });
        }
    }
    Ok(())
}
fn normalize_and_accept(
    entry: &mut LexicalEntry,
    options: &BuildOptions,
) -> Result<(), &'static str> {
    entry.canonical = normalize(&entry.canonical)?;
    entry.normalized_key = entry.canonical.to_lowercase();
    entry.forms = std::mem::take(&mut entry.forms)
        .into_iter()
        .filter_map(|form| normalize(&form).ok())
        .collect();
    if entry.canonical.chars().count() < 2 || entry.canonical.chars().count() > 48 {
        return Err("invalid_length");
    }
    if !options.profile.include_multiword_terms() && entry.canonical.contains(' ') {
        return Err("multiword_disabled");
    }
    if !options.profile.include_proper_nouns()
        && entry.canonical.chars().next().is_some_and(char::is_uppercase)
    {
        return Err("proper_noun_disabled");
    }
    Ok(())
}
/// Normalizes a user-facing lexical surface exactly as dictionary builds do.
/// Consumers can use this for lookup without depending on source providers.
pub fn normalize_lookup(value: &str) -> Result<String, &'static str> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err("empty");
    }
    if trimmed.contains("http://") || trimmed.contains("https://") {
        return Err("url");
    }
    if trimmed.contains('<')
        || trimmed.contains('>')
        || trimmed.contains('{')
        || trimmed.contains('}')
    {
        return Err("markup");
    }
    let mut text = String::new();
    let mut previous_space = false;
    for character in trimmed.chars() {
        if character.is_control() {
            continue;
        }
        if character.is_whitespace() {
            if !previous_space {
                text.push(' ');
            }
            previous_space = true;
            continue;
        }
        previous_space = false;
        if !(character.is_alphabetic()
            || matches!(character, '-' | '\'' | '’' | ' ' | '\u{301}' | '\u{303}' | '\u{308}'))
        {
            return Err("invalid_characters");
        }
        text.push(character);
    }
    let normalized = compose_spanish_nfc(&text);
    if normalized.is_empty() { Err("empty") } else { Ok(normalized) }
}

fn normalize(value: &str) -> Result<String, &'static str> {
    normalize_lookup(value)
}
fn compose_spanish_nfc(value: &str) -> String {
    let mut output = value.to_owned();
    for (from, to) in [
        ("a\u{301}", "á"),
        ("e\u{301}", "é"),
        ("i\u{301}", "í"),
        ("o\u{301}", "ó"),
        ("u\u{301}", "ú"),
        ("u\u{308}", "ü"),
        ("n\u{303}", "ñ"),
        ("A\u{301}", "Á"),
        ("E\u{301}", "É"),
        ("I\u{301}", "Í"),
        ("O\u{301}", "Ó"),
        ("U\u{301}", "Ú"),
        ("U\u{308}", "Ü"),
        ("N\u{303}", "Ñ"),
    ] {
        output = output.replace(from, to);
    }
    output
}
fn merge(entries: Vec<LexicalEntry>, report: &mut BuildReport) -> Vec<LexicalEntry> {
    let mut merged: BTreeMap<String, LexicalEntry> = BTreeMap::new();
    for entry in entries {
        let key = entry.normalized_key.clone();
        if let Some(current) = merged.get_mut(&key) {
            report.duplicate_entries += 1;
            if entry.canonical < current.canonical {
                current.canonical = entry.canonical.clone();
            }
            current.part_of_speech.extend(entry.part_of_speech);
            current.forms.extend(entry.forms);
            current.senses.extend(entry.senses);
            current.relations.extend(entry.relations);
            current.provenance.extend(entry.provenance);
            current.frequency = match (current.frequency, entry.frequency) {
                (Some(left), Some(right)) => Some(left.max(right)),
                (value, None) => value,
                (None, value) => value,
            };
        } else {
            merged.insert(key, entry);
        }
    }
    for entry in merged.values_mut() {
        entry.senses.sort();
        entry.senses.dedup();
        entry.provenance.sort_by_key(|source| {
            (source.source, source.snapshot.clone(), source.source_id.clone())
        });
        entry.provenance.dedup_by(|left, right| {
            left.source == right.source && left.source_id == right.source_id
        });
        entry.relations.sort_by(|left, right| {
            (left.target.as_str(), left.kind, left.source).cmp(&(
                right.target.as_str(),
                right.kind,
                right.source,
            ))
        });
        entry.relations.dedup_by(|left, right| {
            left.target == right.target && left.kind == right.kind && left.source == right.source
        });
    }
    merged.into_values().collect()
}
fn compute_quality(entries: &mut [LexicalEntry], report: &mut BuildReport) {
    let available: BTreeSet<String> =
        entries.iter().map(|entry| entry.normalized_key.clone()).collect();
    for entry in entries {
        let relations = entry
            .relations
            .iter()
            .filter(|relation| available.contains(&relation.target.to_lowercase()))
            .count() as u32;
        entry.quality.relation_count = relations;
        entry.quality.connected = relations > 0;
        entry.quality.frequency = entry.frequency;
        entry.quality.quality_score = (relations.min(8) as f32 / 8.0 * 0.7
            + if entry.frequency.is_some() { 0.15 } else { 0.0 }
            + if !entry.senses.is_empty() { 0.15 } else { 0.0 })
        .min(1.0);
        if entry.quality.connected {
            report.connected_tokens += 1;
        } else {
            report.isolated_tokens += 1;
        }
    }
}
fn render_lx(entries: &[LexicalEntry], maximum: usize, report: &mut BuildReport) -> String {
    let known: BTreeSet<String> =
        entries.iter().map(|entry| entry.normalized_key.clone()).collect();
    let mut tokens = known.clone();
    let mut relations: BTreeMap<(String, String, LexicalRelationKind), (f32, SourceKind)> =
        BTreeMap::new();
    for entry in entries {
        for form in &entry.forms {
            let key = form.to_lowercase();
            if key != entry.normalized_key {
                tokens.insert(key.clone());
                relations.insert(
                    (key, entry.normalized_key.clone(), LexicalRelationKind::InflectionOf),
                    (
                        relation_weight(LexicalRelationKind::InflectionOf, 0.75),
                        SourceKind::Hunspell,
                    ),
                );
            }
        }
        let mut ordered = entry
            .relations
            .iter()
            .filter_map(|relation| {
                let target = relation.target.to_lowercase();
                (known.contains(&target) && target != entry.normalized_key)
                    .then_some((target, relation))
            })
            .collect::<Vec<_>>();
        ordered.sort_by(|left, right| {
            (left.0.as_str(), left.1.kind, left.1.source).cmp(&(
                right.0.as_str(),
                right.1.kind,
                right.1.source,
            ))
        });
        for (target, relation) in ordered.into_iter().take(maximum) {
            let key = (entry.normalized_key.clone(), target, relation.kind);
            let weight = relation_weight(relation.kind, relation.confidence);
            relations
                .entry(key)
                .and_modify(|existing| {
                    if weight > existing.0 {
                        *existing = (weight, relation.source);
                    }
                })
                .or_insert((weight, relation.source));
        }
    }
    let mut text = String::new();
    for token in &tokens {
        writeln!(text, "token {token}").expect("String write cannot fail");
    }
    for ((source, target, kind), (weight, source_kind)) in relations {
        *report.relation_types.entry(kind.name().to_owned()).or_default() += 1;
        *report.relation_sources.entry(source_kind.name().to_owned()).or_default() += 1;
        report.relations += 1;
        writeln!(text, "{source} -> {target} : {weight:.6}").expect("String write cannot fail");
    }
    text
}
fn relation_weight(kind: LexicalRelationKind, confidence: f32) -> f32 {
    let base = match kind {
        LexicalRelationKind::Synonym => 1.0,
        LexicalRelationKind::Antonym => 0.92,
        LexicalRelationKind::Hypernym | LexicalRelationKind::Hyponym => 0.88,
        LexicalRelationKind::Meronym | LexicalRelationKind::Holonym => 0.82,
        LexicalRelationKind::Related => 0.78,
        LexicalRelationKind::DerivedFrom => 0.72,
        LexicalRelationKind::InflectionOf => 0.70,
        LexicalRelationKind::Translation => 0.75,
        LexicalRelationKind::EmbeddingNeighbor => 0.75,
    };
    (base * confidence).clamp(0.0, 1.0)
}
fn render_rejected(rejected: &[(String, &'static str)]) -> String {
    let mut output = String::new();
    for (word, reason) in rejected {
        writeln!(
            output,
            "{{\"source\":\"pipeline\",\"word\":\"{}\",\"reason\":\"{reason}\"}}",
            word.replace('"', "\\\"")
        )
        .expect("String write cannot fail");
    }
    output
}
/// Creates a standards-compliant Zstandard frame containing raw blocks.
fn zstd_raw_frame(contents: &[u8]) -> Result<Vec<u8>, DictionaryError> {
    let length =
        u32::try_from(contents.len()).map_err(|_| DictionaryError::InvalidConfiguration {
            path: PathBuf::from("rejected-entries.jsonl.zst"),
            message: "rejected entry log exceeds 4 GiB".to_owned(),
        })?;
    let mut frame = Vec::with_capacity(contents.len() + 16);
    frame.extend_from_slice(&0xFD2F_B528_u32.to_le_bytes());
    frame.push(0xA0); // Single-segment frame with a four-byte content size.
    frame.extend_from_slice(&length.to_le_bytes());
    let mut offset = 0;
    while offset < contents.len() {
        let chunk_length = (contents.len() - offset).min(128 * 1024);
        let last = offset + chunk_length == contents.len();
        let block_header = (u32::try_from(chunk_length)
            .map_err(|_| DictionaryError::Manifest(PathBuf::from("rejected-entries.jsonl.zst")))?
            << 3)
            | u32::from(last);
        frame.extend_from_slice(&block_header.to_le_bytes()[..3]);
        frame.extend_from_slice(&contents[offset..offset + chunk_length]);
        offset += chunk_length;
    }
    if contents.is_empty() {
        frame.extend_from_slice(&[1, 0, 0]);
    }
    Ok(frame)
}
fn count_lx_tokens(text: &str) -> usize {
    text.lines().filter(|line| line.starts_with("token ")).count()
}
fn fnv1a64(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325_u64, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
    })
}
fn update_cache_manifest(
    language: &str,
    options: &BuildOptions,
    source_root: &Path,
) -> Result<(), DictionaryError> {
    let path = cache_language_directory(&options.cache_dir, language).join("cache-manifest.json");
    atomic_write(
        &path,
        format!(
            "{{\"language\":\"{language}\",\"source_root\":\"{}\",\"mode\":\"{}\"}}\n",
            source_root.display(),
            if options.fixture_dir.is_some() { "fixture" } else { "cache" }
        )
        .as_bytes(),
    )
}
fn field(text: &str, name: &str) -> Option<String> {
    let marker = format!("\"{name}\": \"");
    let rest = text.split_once(&marker)?.1;
    Some(rest.split_once('"')?.0.to_owned())
}
fn toml_string(text: &str, name: &str) -> Option<String> {
    text.lines().find_map(|line| {
        let (key, value) = line.trim().split_once('=')?;
        (key.trim() == name).then(|| value.trim().trim_matches('"').to_owned())
    })
}
