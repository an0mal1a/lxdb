use crate::{
    error::CompilerError,
    model::{RawRelation, RawToken},
};

/// Parses external sources into raw semantic data.
#[derive(Debug, Default)]
pub struct Parser;

pub struct ParseResult {
    pub tokens: Vec<RawToken>,
    pub relations: Vec<RawRelation>,
}

impl Parser {
    pub fn parse(self) -> Result<ParseResult, CompilerError> {
        todo!()
    }
}