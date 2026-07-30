use std::{fmt::Write, path::Path};

use crate::{SourceKind, config::BuildOptions, language::LanguageProfile, report::BuildReport};

pub fn render_manifest(
    language: &LanguageProfile,
    options: &BuildOptions,
    report: &BuildReport,
    dataset_hash: u64,
) -> String {
    let mut text = String::new();
    writeln!(text, "{{").expect("String write cannot fail");
    writeln!(text, "  \"language\": \"{}\",", language.iso_639_1)
        .expect("String write cannot fail");
    writeln!(text, "  \"language_name\": \"{}\",", language.display_name)
        .expect("String write cannot fail");
    writeln!(text, "  \"generator_version\": \"{}\",", env!("CARGO_PKG_VERSION"))
        .expect("String write cannot fail");
    writeln!(text, "  \"lxdb_format\": \"0.1\",").expect("String write cannot fail");
    writeln!(text, "  \"profile\": \"{}\",", options.profile.name())
        .expect("String write cannot fail");
    writeln!(text, "  \"dataset_hash_fnv1a64\": \"{dataset_hash:016x}\",")
        .expect("String write cannot fail");
    writeln!(text, "  \"normalization\": {{\"unicode\": \"NFC (Spanish combining-mark subset)\", \"case_policy\": \"preserve-and-fold\"}},").expect("String write cannot fail");
    writeln!(text, "  \"relation_strategy\": {{\"version\": \"1\", \"formula\": \"base × source confidence\"}},").expect("String write cannot fail");
    writeln!(text, "  \"sources\": [").expect("String write cannot fail");
    let sources = used_sources(options);
    for (index, source) in sources.iter().enumerate() {
        let trailing = if index + 1 == sources.len() { "" } else { "," };
        writeln!(text, "    {{\"name\": \"{}\", \"snapshot\": \"fixture-or-cache\", \"license\": \"{}\"}}{trailing}", source.name(), source_license(*source)).expect("String write cannot fail");
    }
    writeln!(text, "  ],").expect("String write cannot fail");
    writeln!(text, "  \"counts\": {{\"entries_read\": {}, \"accepted\": {}, \"lemmas\": {}, \"forms\": {}, \"relations\": {}}}", report.entries_read, report.entries_accepted, report.unique_lemmas, report.surface_forms, report.relations).expect("String write cannot fail");
    writeln!(text, "}}").expect("String write cannot fail");
    text
}

pub fn render_report(report: &BuildReport) -> String {
    let phases = report
        .phases
        .iter()
        .map(|(name, duration)| format!("\"{name}\": {}", duration.as_millis()))
        .collect::<Vec<_>>()
        .join(", ");
    let kinds = report
        .relation_types
        .iter()
        .map(|(name, count)| format!("\"{name}\": {count}"))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "{{\n  \"entries_read\": {},\n  \"entries_invalid\": {},\n  \"entries_accepted\": {},\n  \"entries_rejected\": {},\n  \"unique_lemmas\": {},\n  \"surface_forms\": {},\n  \"senses\": {},\n  \"relations\": {},\n  \"duplicates\": {},\n  \"connected_tokens\": {},\n  \"isolated_tokens\": {},\n  \"relations_by_type\": {{{kinds}}},\n  \"phase_durations_ms\": {{{phases}}}\n}}\n",
        report.entries_read,
        report.entries_invalid,
        report.entries_accepted,
        report.entries_rejected,
        report.unique_lemmas,
        report.surface_forms,
        report.senses,
        report.relations,
        report.duplicate_entries,
        report.connected_tokens,
        report.isolated_tokens
    )
}

pub fn render_attribution(options: &BuildOptions) -> String {
    let mut text = String::from(
        "# Dictionary source attribution\n\nThis file is generated for the exact sources enabled in this build. Verify a source snapshot's bundled license before redistributing an expanded dataset.\n\n",
    );
    for source in used_sources(options) {
        let (url, license, note) = match source {
            SourceKind::Kaikki => (
                "https://kaikki.org/dictionary/Spanish/",
                "Wiktionary content: CC BY-SA 4.0 / GFDL",
                "Derived structured Wiktionary data via Wiktextract; attribute Wiktionary contributors and retain share-alike obligations.",
            ),
            SourceKind::Hunspell => (
                "https://github.com/LibreOffice/dictionaries",
                "Spanish dictionary: GPLv3+ / LGPLv3+ / MPL 1.1",
                "Used only for orthographic coverage; retain the selected dictionary license and notices.",
            ),
            SourceKind::WordNet => (
                "https://nlp.lsi.upc.edu/omw/",
                "Source-specific; recorded from the downloaded snapshot",
                "The build preserves the source identity but does not assume a universal WordNet license.",
            ),
            SourceKind::Frequency => (
                "https://github.com/rspeer/wordfreq",
                "Apache-2.0 software; frequency data CC BY-SA 4.0",
                "Used only for ranking and quality; never as a semantic edge weight.",
            ),
            SourceKind::Embedding => (
                "",
                "Model-specific",
                "Optional offline provider; this build does not ship model data.",
            ),
        };
        writeln!(
            text,
            "## {}\n\n- URL: {url}\n- License: {license}\n- Notes: {note}\n",
            source.name()
        )
        .expect("String write cannot fail");
    }
    text
}

pub fn source_license(source: SourceKind) -> &'static str {
    match source {
        SourceKind::Kaikki => "CC BY-SA 4.0 / GFDL",
        SourceKind::Hunspell => "GPLv3+ / LGPLv3+ / MPL 1.1",
        SourceKind::WordNet => "snapshot-specific",
        SourceKind::Frequency => "CC BY-SA 4.0 data",
        SourceKind::Embedding => "model-specific",
    }
}
fn used_sources(options: &BuildOptions) -> Vec<SourceKind> {
    let mut sources = Vec::new();
    if options.sources.kaikki {
        sources.push(SourceKind::Kaikki);
    }
    if options.sources.hunspell {
        sources.push(SourceKind::Hunspell);
    }
    if options.sources.wordnet {
        sources.push(SourceKind::WordNet);
    }
    if options.sources.frequency {
        sources.push(SourceKind::Frequency);
    }
    if options.sources.embeddings {
        sources.push(SourceKind::Embedding);
    }
    sources
}

pub fn manifest_language(path: &Path) -> Option<String> {
    let text = std::fs::read_to_string(path).ok()?;
    let marker = "\"language\": \"";
    let value = text.split_once(marker)?.1;
    Some(value.split_once('"')?.0.to_owned())
}
