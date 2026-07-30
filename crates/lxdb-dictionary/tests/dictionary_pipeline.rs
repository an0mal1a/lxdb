use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use lxdb_dictionary::{BuildOptions, DictionaryProfile, build, find_language};
use lxdb_engine::BinaryDatasetExt;
use lxdb_storage::DatasetReader;

fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}
fn output_dir() -> PathBuf {
    std::env::temp_dir().join(format!(
        "lxdb-dictionary-test-{}",
        SystemTime::now().duration_since(UNIX_EPOCH).expect("time").as_nanos()
    ))
}

#[test]
fn resolves_languages_and_rejects_unknown_ones() {
    assert_eq!(find_language("es").expect("Spanish").iso_639_3, "spa");
    assert_eq!(find_language("en").expect("English").hunspell_locale, Some("en_US"));
    assert!(find_language("zz").is_none());
}

#[test]
fn builds_a_deterministic_spanish_fixture_dataset() {
    let directory = output_dir();
    let mut options = BuildOptions::new("es", &directory);
    options.profile = DictionaryProfile::Development;
    options.fixture_dir = Some(fixture_dir());
    options.cache_dir = directory.join("cache");
    let report = build(&options).expect("fixture build should work");
    assert!(report.entries_read >= 20);
    assert!(report.relations > 0);
    for artifact in [
        "dictionary.lxdb",
        "manifest.json",
        "build-report.json",
        "ATTRIBUTION.md",
        "rejected-entries.jsonl.zst",
    ] {
        assert!(directory.join(artifact).exists(), "missing {artifact}");
    }
    let dataset = DatasetReader::new()
        .open(directory.join("dictionary.lxdb"))
        .expect("dataset should validate");
    let query = dataset.query();
    assert!(query.token_by_text("casa").expect("query").is_some());
    assert!(query.token_by_text("árbol").expect("query").is_some());
    assert!(query.token_by_text("casas").expect("query").is_some());
    assert!(query.related_to("perro").expect("query").expect("perro").len() > 0);
    let rejected = fs::read(directory.join("rejected-entries.jsonl.zst")).expect("rejected");
    assert_eq!(&rejected[..4], &0xFD2F_B528_u32.to_le_bytes());
    fs::remove_dir_all(directory).expect("cleanup");
}
