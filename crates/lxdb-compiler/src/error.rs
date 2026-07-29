/// Compiler errors.
#[derive(Debug)]
pub enum CompilerError {
    InvalidWeight,
    EmptyToken,
    SelfReference,
}