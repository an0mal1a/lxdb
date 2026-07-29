use lxdb_core::graph::SemanticGraph;

use crate::{
    builder::Builder,
    error::CompilerError,
    pipeline::{GraphBuilder, Parser, Validator, Writer},
};

/// Coordinates the complete LXDB compilation pipeline.
#[derive(Debug, Default)]
pub struct Compiler;

impl Compiler {
    pub const fn new() -> Self {
        Self
    }

    pub fn compile(self, builder: Builder) -> Result<SemanticGraph, CompilerError> {
        let parsed = Parser.parse(&builder)?;
        let validated = Validator.validate(parsed)?;
        let graph = GraphBuilder.build(validated)?;
        let bytes = Writer::new().encode(&graph)?;

        let output = builder.output_path().ok_or(CompilerError::MissingOutput)?;

        std::fs::write(output, bytes)?;

        Ok(graph)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    #[test]
    fn writes_binary_dataset() {
        let tmp = std::env::temp_dir();

        let unique = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();

        let input = tmp.join(format!("lxdb_input_{unique}.txt"));
        let output = tmp.join(format!("lxdb_output_{unique}.lxdb"));

        fs::write(&input, "rust -> language : 1.0\n").unwrap();

        let builder =
            Builder::new().input(input.to_str().unwrap()).output(output.to_str().unwrap());

        Compiler::new().compile(builder).unwrap();

        assert!(output.exists());

        let bytes = fs::read(&output).unwrap();

        assert!(!bytes.is_empty());

        let _ = fs::remove_file(&input);
        let _ = fs::remove_file(&output);
    }
}
