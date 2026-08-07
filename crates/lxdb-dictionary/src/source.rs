//! Streaming adapters for source formats used by the fixture and cache builds.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::{
    DictionaryError, LexicalEntry, LexicalRelation, LexicalRelationKind, SourceKind,
    SourceReference,
};

pub fn parse_kaikki(
    path: &Path,
    language: &str,
    snapshot: &str,
    entries: &mut Vec<LexicalEntry>,
    invalid: &mut u64,
    max_entries: Option<usize>,
) -> Result<u64, DictionaryError> {
    let file = File::open(path).map_err(|_| DictionaryError::MissingSource {
        source: "Kaikki",
        path: path.to_path_buf(),
    })?;
    let mut read = 0;
    for (index, line) in BufReader::new(file).lines().enumerate() {
        if max_entries.is_some_and(|limit| entries.len() >= limit) {
            break;
        }
        let line_number = index + 1;
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        read += 1;
        let Some(word) = json_string(&line, "word") else {
            *invalid += 1;
            continue;
        };
        let entry_language = json_string(&line, "lang");
        if let Some(entry_language) = entry_language {
            if entry_language != "Spanish" && entry_language != "English" {
                continue;
            }
        }
        let mut entry = LexicalEntry::new(word, language);
        if let Some(pos) = json_string(&line, "pos") {
            entry.part_of_speech.insert(pos);
        }
        entry.forms.extend(
            objects_after_key(&line, "forms")
                .into_iter()
                .filter_map(|object| json_string(object, "form")),
        );
        entry.senses.extend(strings_after_key(&line, "glosses"));
        for (key, kind) in [
            ("synonyms", LexicalRelationKind::Synonym),
            ("antonyms", LexicalRelationKind::Antonym),
            ("hypernyms", LexicalRelationKind::Hypernym),
            ("hyponyms", LexicalRelationKind::Hyponym),
            ("meronyms", LexicalRelationKind::Meronym),
            ("holonyms", LexicalRelationKind::Holonym),
            ("related", LexicalRelationKind::Related),
            ("form_of", LexicalRelationKind::InflectionOf),
        ] {
            for object in objects_after_key(&line, key) {
                if let Some(target) = json_string(object, "word") {
                    entry.relations.push(LexicalRelation {
                        target,
                        kind,
                        source: SourceKind::Kaikki,
                        confidence: 0.90,
                    });
                }
            }
        }
        entry.provenance.push(SourceReference {
            source: SourceKind::Kaikki,
            snapshot: snapshot.to_owned(),
            source_id: json_string(&line, "id"),
            confidence: 0.90,
        });
        entries.push(entry);
        if line_number == usize::MAX {
            return Err(DictionaryError::InvalidSource {
                source: "Kaikki",
                path: path.to_path_buf(),
                line: line_number,
                message: "line counter overflow".to_owned(),
            });
        }
    }
    Ok(read)
}

pub fn parse_hunspell(
    path: &Path,
    language: &str,
    snapshot: &str,
    entries: &mut Vec<LexicalEntry>,
) -> Result<u64, DictionaryError> {
    let file = File::open(path).map_err(|_| DictionaryError::MissingSource {
        source: "Hunspell",
        path: path.to_path_buf(),
    })?;
    let mut count = 0;
    for (index, line) in BufReader::new(file).lines().enumerate() {
        let line = line?;
        if index == 0 && line.trim().parse::<usize>().is_ok() {
            continue;
        }
        let word = line.split('/').next().unwrap_or_default().trim();
        if word.is_empty() || word.starts_with('#') {
            continue;
        }
        let mut entry = LexicalEntry::new(word.to_owned(), language);
        entry.forms.insert(word.to_owned());
        entry.provenance.push(SourceReference {
            source: SourceKind::Hunspell,
            snapshot: snapshot.to_owned(),
            source_id: None,
            confidence: 0.75,
        });
        entries.push(entry);
        count += 1;
    }
    Ok(count)
}

/// Reads a WN-LMF XML file without materialising the whole lexical graph.
/// The adapter handles the portable subset used by Global Wordnet exports:
/// `LexicalEntry/Lemma/Sense` and `Synset/SynsetRelation`.
pub fn parse_wordnet_lmf(
    path: &Path,
    language: &str,
    snapshot: &str,
    entries: &mut Vec<LexicalEntry>,
) -> Result<u64, DictionaryError> {
    let text = std::fs::read_to_string(path).map_err(|_| DictionaryError::MissingSource {
        source: "WordNet",
        path: path.to_path_buf(),
    })?;
    let mut synsets: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut relations = Vec::new();
    let mut count = 0;
    for block in xml_blocks(&text, "LexicalEntry") {
        let Some(lemma_tag) = xml_tag(block, "Lemma") else {
            continue;
        };
        let Some(word) = attribute(lemma_tag, "writtenForm") else {
            continue;
        };
        let mut entry = LexicalEntry::new(word.to_owned(), language);
        if let Some(pos) = attribute(lemma_tag, "partOfSpeech") {
            entry.part_of_speech.insert(pos.to_owned());
        }
        for sense in xml_tags(block, "Sense") {
            if let Some(synset) = attribute(sense, "synset") {
                synsets.entry(synset.to_owned()).or_default().insert(word.to_owned());
            }
        }
        entry.provenance.push(SourceReference {
            source: SourceKind::WordNet,
            snapshot: snapshot.to_owned(),
            source_id: None,
            confidence: 1.0,
        });
        entries.push(entry);
        count += 1;
    }
    for block in xml_blocks(&text, "Synset") {
        let Some(opening) = block.split('>').next() else {
            continue;
        };
        let Some(source) = attribute(opening, "id") else {
            continue;
        };
        for relation in xml_tags(block, "SynsetRelation") {
            let Some(target) = attribute(relation, "target") else {
                continue;
            };
            let kind = match attribute(relation, "relType") {
                Some("hypernym") => LexicalRelationKind::Hypernym,
                Some("hyponym") => LexicalRelationKind::Hyponym,
                Some("meronym") => LexicalRelationKind::Meronym,
                Some("holonym") => LexicalRelationKind::Holonym,
                _ => continue,
            };
            relations.push((source.to_owned(), target.to_owned(), kind));
        }
    }
    let mut by_word: BTreeMap<String, usize> = BTreeMap::new();
    for (index, entry) in entries.iter().enumerate() {
        by_word.entry(entry.canonical.clone()).or_insert(index);
    }
    for words in synsets.values() {
        for source in words {
            if let Some(index) = by_word.get(source) {
                for target in words {
                    if source != target {
                        entries[*index].relations.push(LexicalRelation {
                            target: target.clone(),
                            kind: LexicalRelationKind::Synonym,
                            source: SourceKind::WordNet,
                            confidence: 1.0,
                        });
                    }
                }
            }
        }
    }
    for (source_synset, target_synset, kind) in relations {
        if let (Some(sources), Some(targets)) =
            (synsets.get(&source_synset), synsets.get(&target_synset))
        {
            for source in sources {
                if let Some(index) = by_word.get(source) {
                    for target in targets {
                        entries[*index].relations.push(LexicalRelation {
                            target: target.clone(),
                            kind,
                            source: SourceKind::WordNet,
                            confidence: 1.0,
                        });
                    }
                }
            }
        }
    }
    Ok(count)
}

pub fn parse_frequency(path: &Path, entries: &mut [LexicalEntry]) -> Result<u64, DictionaryError> {
    let file = File::open(path).map_err(|_| DictionaryError::MissingSource {
        source: "frequency",
        path: path.to_path_buf(),
    })?;
    let mut values = BTreeMap::new();
    for line in BufReader::new(file).lines() {
        let line = line?;
        let Some((word, value)) = line.split_once('\t') else {
            continue;
        };
        if let Ok(value) = value.trim().parse::<f32>() {
            values.insert(word.trim().to_lowercase(), value);
        }
    }
    let mut matched = 0;
    for entry in entries {
        if let Some(value) = values.get(&entry.normalized_key) {
            entry.frequency = Some(*value);
            matched += 1;
        }
    }
    Ok(matched)
}

fn json_string(text: &str, key: &str) -> Option<String> {
    let marker = format!("\"{key}\"");
    let start = text.find(&marker)? + marker.len();
    let value = text[start..].trim_start().strip_prefix(':')?.trim_start().strip_prefix('"')?;
    let end = value.find('"')?;
    Some(value[..end].replace("\\\"", "\"").replace("\\n", " "))
}
fn objects_after_key<'a>(text: &'a str, key: &str) -> Vec<&'a str> {
    let Some(position) = text.find(&format!("\"{key}\"")) else {
        return Vec::new();
    };
    let rest = &text[position..];
    let Some(start) = rest.find('[') else {
        return Vec::new();
    };
    let mut result = Vec::new();
    let mut depth = 0;
    let mut object_start = None;
    for (offset, character) in rest[start..].char_indices() {
        match character {
            '{' => {
                if depth == 0 {
                    object_start = Some(offset);
                }
                depth += 1;
            }
            '}' => {
                depth -= 1;
                if depth == 0 {
                    if let Some(begin) = object_start {
                        result.push(&rest[start + begin..start + offset + 1]);
                    }
                }
            }
            ']' if depth == 0 => break,
            _ => {}
        }
    }
    result
}
fn strings_after_key(text: &str, key: &str) -> Vec<String> {
    let Some(position) = text.find(&format!("\"{key}\"")) else {
        return Vec::new();
    };
    let rest = &text[position..];
    let Some(start) = rest.find('[') else {
        return Vec::new();
    };
    let Some(end) = rest[start..].find(']') else {
        return Vec::new();
    };
    rest[start + 1..start + end]
        .split('"')
        .skip(1)
        .step_by(2)
        .map(|value| value.to_owned())
        .collect()
}
fn xml_blocks<'a>(text: &'a str, tag: &str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut rest = text;
    let open = format!("<{tag}");
    let close = format!("</{tag}>");
    while let Some(start) = rest.find(&open) {
        let tail = &rest[start..];
        let Some(end) = tail.find(&close) else {
            break;
        };
        let end = end + close.len();
        result.push(&tail[..end]);
        rest = &tail[end..];
    }
    result
}
fn xml_tags<'a>(text: &'a str, tag: &str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut rest = text;
    let open = format!("<{tag}");
    while let Some(start) = rest.find(&open) {
        let tail = &rest[start..];
        let Some(end) = tail.find('>') else {
            break;
        };
        result.push(&tail[..=end]);
        rest = &tail[end + 1..];
    }
    result
}
fn xml_tag<'a>(text: &'a str, tag: &str) -> Option<&'a str> {
    xml_tags(text, tag).into_iter().next()
}
fn attribute<'a>(text: &'a str, name: &str) -> Option<&'a str> {
    let marker = format!("{name}=\"");
    let start = text.find(&marker)? + marker.len();
    let value = &text[start..];
    Some(&value[..value.find('"')?])
}
