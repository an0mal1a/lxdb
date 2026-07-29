use std::{fs, path::Path};

use lxdb_compiler::{builder::Builder, compiler::Compiler};

use crate::error::CliError;

pub fn execute_compile(source_path: &Path, output_path: &Path) -> Result<(), CliError> {
    let input = fs::read_to_string(source_path).map_err(|source| CliError::CompileDataset {
        source_path: source_path.to_path_buf(),
        output_path: output_path.to_path_buf(),
        source: Box::new(source),
    })?;

    let builder =
        Builder::new().input(input).output(output_path.to_string_lossy().into_owned()).build();

    Compiler::new().compile(builder).map_err(|source| CliError::CompileDataset {
        source_path: source_path.to_path_buf(),
        output_path: output_path.to_path_buf(),
        source: Box::new(source),
    })?;

    println!("Compiled {} → {}", source_path.display(), output_path.display(),);

    Ok(())
}
