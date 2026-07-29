use crate::{
    builder::Builder,
    error::CompilerError,
    model::{RawRelation, RawToken},
};

/// Raw semantic data produced by the parser.
#[derive(Debug)]
pub struct ParseResult {
    pub tokens: Vec<RawToken>,
    pub relations: Vec<RawRelation>,
}

/// Parses an external source into raw semantic data.
#[derive(Debug, Default)]
pub struct Parser;

impl Parser {
    pub fn parse(&self, builder: &Builder) -> Result<ParseResult, CompilerError> {
        let _ = builder;

        todo!("source parsing will be implemented later")
    }
}
