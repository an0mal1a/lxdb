use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
    fs,
    path::{Path, PathBuf},
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
    let mut report = BuildReport::default();
    let mut entries = Vec::new();
    let started = Instant::now();
    let source_root = source_root(options, language.iso_639_1)?;
    let mut invalid = 0;
    if options.sources.kaikki {
        let timer = Instant::now();
        let path = source_root.join(format!("kaikki-{}-small.jsonl", language.iso_639_1));
        report.entries_read +=
            source::parse_kaikki(&path, language.iso_639_1, "fixture", &mut entries, &mut invalid)?;
        report.phases.insert("extract_kaikki".to_owned(), timer.elapsed());
    }
    if options.sources.hunspell {
        let timer = Instant::now();
        let path = source_root.join(format!("hunspell-{}-small.dic", language.iso_639_1));
        report.entries_read +=
            source::parse_hunspell(&path, language.iso_639_1, "fixture", &mut entries)?;
        report.phases.insert("extract_hunspell".to_owned(), timer.elapsed());
    }
    if options.sources.wordnet {
        let timer = Instant::now();
        let path = source_root.join(format!("wordnet-{}-small.xml", language.iso_639_1));
        report.entries_read +=
            source::parse_wordnet_lmf(&path, language.iso_639_1, "fixture", &mut entries)?;
        report.phases.insert("extract_wordnet".to_owned(), timer.elapsed());
    }
    if options.sources.frequency {
        let timer = Instant::now();
        let path = source_root.join(format!("frequency-{}-small.tsv", language.iso_639_1));
        source::parse_frequency(&path, &mut entries)?;
        report.phases.insert("extract_frequency".to_owned(), timer.elapsed());
    }
    report.entries_invalid = invalid;
    let timer = Instant::now();
    let mut rejected = Vec::new();
    entries.retain_mut(|entry| match normalize_and_accept(entry, options) {
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
    if let Some(limit) = options.effective_limit() {
        entries.truncate(limit);
    }
    compute_quality(&mut entries, &mut report);
    report.phases.insert("merge_quality".to_owned(), timer.elapsed());
    report.entries_accepted = entries.len() as u64;
    report.unique_lemmas = entries.len() as u64;
    report.surface_forms = entries.iter().map(|entry| entry.forms.len() as u64).sum();
    report.senses = entries.iter().map(|entry| entry.senses.len() as u64).sum();
    let timer = Instant::now();
    let source_text = render_lx(&entries, options.profile.max_relations_per_token(), &mut report);
    report.phases.insert("graph".to_owned(), timer.elapsed());
    fs::create_dir_all(&options.output_dir)?;
    let lx_path = options.output_dir.join("dictionary.lx");
    atomic_write(&lx_path, source_text.as_bytes())?;
    let output = options.output_dir.join("dictionary.lxdb");
    let temporary = options.output_dir.join("dictionary.lxdb.tmp");
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
        &options.output_dir.join("manifest.json"),
        manifest::render_manifest(language, options, &report, dataset_hash).as_bytes(),
    )?;
    atomic_write(
        &options.output_dir.join("build-report.json"),
        manifest::render_report(&report).as_bytes(),
    )?;
    atomic_write(
        &options.output_dir.join("ATTRIBUTION.md"),
        manifest::render_attribution(options).as_bytes(),
    )?;
    if options.emit_rejected {
        atomic_write(
            &options.output_dir.join("rejected-entries.jsonl.zst"),
            &zstd_raw_frame(render_rejected(&rejected).as_bytes())?,
        )?;
    }
    report.phases.insert("artifacts".to_owned(), timer.elapsed());
    report.phases.insert("total".to_owned(), started.elapsed());
    update_cache_manifest(language.iso_639_1, options, &source_root)?;
    if options.emit_source.is_some() || options.keep_intermediate {
        let destination = options.emit_source.as_ref().unwrap_or(&lx_path);
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
    let directory = cache_language_directory(cache_dir, language);
    fs::create_dir_all(&directory)?;
    let path = directory.join("cache-manifest.json");
    atomic_write(&path, format!("{{\n  \"language\": \"{language}\",\n  \"mode\": \"local-cache\",\n  \"note\": \"Place verified source snapshots in this directory or pass --source-fixture. Network download is intentionally not implicit.\"\n}}\n").as_bytes())?;
    Ok(path)
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

fn source_root(options: &BuildOptions, language: &str) -> Result<PathBuf, DictionaryError> {
    if let Some(fixture) = &options.fixture_dir {
        return Ok(fixture.clone());
    }
    let cached = cache_language_directory(&options.cache_dir, language);
    if options.offline && !cached.exists() {
        return Err(DictionaryError::OfflineCacheMiss { source: "dictionary", path: cached });
    }
    if cached.exists() {
        return Ok(cached);
    }
    Err(DictionaryError::MissingSource { source: "dictionary", path: cached })
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
fn normalize(value: &str) -> Result<String, &'static str> {
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
        if !(character.is_alphabetic() || matches!(character, '-' | '\'' | '’' | ' ')) {
            return Err("invalid_characters");
        }
        text.push(character);
    }
    let normalized = compose_spanish_nfc(&text);
    if normalized.is_empty() { Err("empty") } else { Ok(normalized) }
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
    rejected
        .iter()
        .map(|(word, reason)| {
            format!(
                "{{\"source\":\"pipeline\",\"word\":\"{}\",\"reason\":\"{reason}\"}}\n",
                word.replace('"', "\\\"")
            )
        })
        .collect()
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
