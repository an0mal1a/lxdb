/// Errors produced while compiling an LXDB dataset.
#[derive(Debug)]
pub enum CompilerError {
    InvalidWeight,
    EmptyToken,
    SelfReference,
    DuplicateToken(String),
    UnknownToken(String),
}
