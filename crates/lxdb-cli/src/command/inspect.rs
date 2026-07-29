use std::{fmt::Write, path::Path};

use lxdb_storage::BinaryDataset;

use crate::{command::open_dataset, error::CliError};

/// Displays structural metadata exposed by the storage API.
pub fn execute_inspect(dataset_path: &Path) -> Result<(), CliError> {
    let dataset = open_dataset(dataset_path)?;

    print!("{}", format_summary(dataset_path, &dataset));

    Ok(())
}

fn format_summary(dataset_path: &Path, dataset: &BinaryDataset) -> String {
    let name = dataset_path
        .file_name()
        .map(|name| name.to_string_lossy())
        .unwrap_or_else(|| dataset_path.to_string_lossy());

    let version = dataset.version();
    let mut summary = String::new();

    writeln!(summary, "Dataset: {name}").expect("writing to a String cannot fail");
    writeln!(summary, "Version: {}.{}", version.major(), version.minor())
        .expect("writing to a String cannot fail");
    writeln!(summary, "Tokens: {}", dataset.token_count())
        .expect("writing to a String cannot fail");
    writeln!(summary, "Relations: {}", dataset.relation_count())
        .expect("writing to a String cannot fail");
    writeln!(summary, "Adjacency records: {}", dataset.adjacency_count())
        .expect("writing to a String cannot fail");
    writeln!(summary, "Token string table: {} bytes", dataset.token_string_table().len())
        .expect("writing to a String cannot fail");
    writeln!(summary, "File size: {} bytes", dataset.file_size())
        .expect("writing to a String cannot fail");

    summary
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    use lxdb_compiler::{builder::Builder, compiler::Compiler};
    use lxdb_storage::DatasetReader;

    use super::format_summary;

    #[test]
    fn formats_public_dataset_statistics() {
        let directory = std::env::temp_dir().join(format!(
            "lxdb-inspect-test-{}-{}",
            std::process::id(),
            SystemTime::now().duration_since(UNIX_EPOCH).expect("time should be valid").as_nanos(),
        ));
        fs::create_dir(&directory).expect("test directory should be created");

        let source = directory.join("knowledge.lx");
        let output = directory.join("knowledge.lxdb");
        fs::write(&source, "rust -> language : 0.9\n").expect("source should be written");

        Compiler::new()
            .compile(
                Builder::new()
                    .input(source.to_string_lossy().into_owned())
                    .output(output.to_string_lossy().into_owned())
                    .build(),
            )
            .expect("dataset should compile");

        let dataset = DatasetReader::new().open(&output).expect("dataset should open");
        let summary = format_summary(&output, &dataset);

        assert!(summary.contains("Dataset: knowledge.lxdb\n"));
        assert!(summary.contains("Version: 0.1\n"));
        assert!(summary.contains("Tokens: 2\n"));
        assert!(summary.contains("Relations: 1\n"));
        assert!(summary.contains("Adjacency records: 2\n"));
        assert!(summary.contains("Token string table: 12 bytes\n"));
        assert!(summary.contains(&format!("File size: {} bytes\n", dataset.file_size())));

        fs::remove_dir_all(directory).expect("test directory should be removed");
    }
}
