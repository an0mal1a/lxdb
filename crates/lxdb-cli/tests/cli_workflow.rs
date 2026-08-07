use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn cli() -> &'static str {
    env!("CARGO_BIN_EXE_lxdb-cli")
}

fn test_directory() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be valid")
        .as_nanos();
    let directory =
        std::env::temp_dir().join(format!("lxdb-cli-test-{}-{timestamp}", std::process::id()));

    fs::create_dir(&directory).expect("test directory should be created");

    directory
}

fn output(command: &mut Command) -> (bool, String, String) {
    let output = command.output().expect("CLI process should start");

    (
        output.status.success(),
        String::from_utf8(output.stdout).expect("stdout should be UTF-8"),
        String::from_utf8(output.stderr).expect("stderr should be UTF-8"),
    )
}

fn normalize_newlines(value: String) -> String {
    value.replace("\r\n", "\n")
}

#[test]
fn compiles_queries_and_inspects_the_example_dataset() {
    let directory = test_directory();
    let source = directory.join("knowledge.lx");
    let dataset = directory.join("knowledge.lxdb");

    fs::write(&source, include_str!("../../../examples/knowledge.lx"))
        .expect("example source should be copied");

    let (success, stdout, stderr) =
        output(Command::new(cli()).arg("compile").arg(&source).arg("--output").arg(&dataset));
    assert!(success, "compile failed: {stderr}");
    assert_eq!(
        normalize_newlines(stdout),
        format!("Compiled {} → {}\n", source.display(), dataset.display()),
    );

    let (success, stdout, stderr) =
        output(Command::new(cli()).arg("query").arg(&dataset).arg("rust"));
    assert!(success, "query failed: {stderr}");
    assert_eq!(
        normalize_newlines(stdout),
        "rust\n├─ 0.950 → language\n├─ 0.700 → compiler\n└─ 0.880 → memory\n",
    );

    let (success, stdout, stderr) =
        output(Command::new(cli()).arg("query").arg(&dataset).arg("compiler"));
    assert!(success, "empty query failed: {stderr}");
    assert_eq!(normalize_newlines(stdout), "compiler\n└─ no outgoing relations\n");

    let (success, stdout, stderr) =
        output(Command::new(cli()).arg("query").arg(&dataset).arg("missing"));
    assert!(success, "missing token query failed: {stderr}");
    assert_eq!(normalize_newlines(stdout), "Token not found: missing\n");

    let (success, stdout, stderr) = output(Command::new(cli()).arg("inspect").arg(&dataset));
    assert!(success, "inspect failed: {stderr}");
    assert_eq!(
        normalize_newlines(stdout),
        "Dataset: knowledge.lxdb\nVersion: 0.1\nTokens: 7\nRelations: 6\nAdjacency records: 7\nToken string table: 51 bytes\nFile size: 483 bytes\n",
    );

    fs::remove_dir_all(directory).expect("test directory should be removed");
}

#[test]
fn reports_invalid_sources_and_corrupt_datasets_as_failures() {
    let directory = test_directory();
    let invalid_source = directory.join("invalid.lx");
    let missing_source = directory.join("missing.lx");
    let output_dataset = directory.join("invalid.lxdb");
    let corrupt_dataset = directory.join("corrupt.lxdb");
    let missing_dataset = directory.join("missing.lxdb");

    // A standalone `token <text>` declaration is valid source syntax. Use a
    // relation missing its weight to exercise the invalid-source path.
    fs::write(&invalid_source, "rust -> language\n").expect("invalid source should be written");
    fs::write(&corrupt_dataset, [0_u8; 8]).expect("corrupt dataset should be written");

    let (success, _, stderr) = output(
        Command::new(cli())
            .arg("compile")
            .arg(&invalid_source)
            .arg("--output")
            .arg(&output_dataset),
    );
    assert!(!success);
    assert!(stderr.contains("invalid syntax at line 1: rust -> language"));

    let (success, _, stderr) = output(
        Command::new(cli())
            .arg("compile")
            .arg(&missing_source)
            .arg("--output")
            .arg(&output_dataset),
    );
    assert!(!success);
    assert!(stderr.contains("failed to compile"));
    assert!(stderr.contains("input/output error"));

    let (success, _, stderr) =
        output(Command::new(cli()).arg("query").arg(&corrupt_dataset).arg("rust"));
    assert!(!success);
    assert!(stderr.contains("failed to open dataset"));
    assert!(stderr.contains("invalid or unsupported LXDB header"));

    let (success, _, stderr) = output(Command::new(cli()).arg("inspect").arg(&corrupt_dataset));
    assert!(!success);
    assert!(stderr.contains("failed to open dataset"));

    let (success, _, stderr) =
        output(Command::new(cli()).arg("query").arg(&missing_dataset).arg("rust"));
    assert!(!success);
    assert!(stderr.contains("failed to open dataset"));

    assert!(!Path::new(&output_dataset).exists());
    fs::remove_dir_all(directory).expect("test directory should be removed");
}
