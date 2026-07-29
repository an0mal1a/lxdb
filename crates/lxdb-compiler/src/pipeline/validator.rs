use crate::{
    error::CompilerError,
    pipeline::parser::ParseResult,
};

#[derive(Debug, Default)]
pub struct Validator;

impl Validator {
    pub fn validate(
        &self,
        result: ParseResult,
    ) -> Result<ParseResult, CompilerError> {

        for token in &result.tokens {
            if token.text.trim().is_empty() {
                return Err(CompilerError::EmptyToken);
            }
        }

        for relation in &result.relations {
            if !(0.0..=1.0).contains(&relation.weight) {
                return Err(CompilerError::InvalidWeight);
            }

            if relation.source == relation.target {
                return Err(CompilerError::SelfReference);
            }
        }

        Ok(result)
    }
}