use crate::{builder::Builder, error::CompilerError};

/// Compiles external data sources into LXDB datasets.
pub struct Compiler;

impl Compiler {
    pub fn new() -> Self {
        Self
    }

    pub fn compile(self, builder: Builder) -> Result<(), CompilerError> {
        let _ = builder;

        todo!()
    }
}