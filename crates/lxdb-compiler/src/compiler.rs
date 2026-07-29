use lxdb_core::graph::SemanticGraph;

use crate::{
    builder::Builder,
    error::CompilerError,
    pipeline::{GraphBuilder, Parser, Validator},
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

        Ok(graph)
    }
}
