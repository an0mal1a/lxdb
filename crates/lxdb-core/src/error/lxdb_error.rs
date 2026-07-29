/// Core error type used across LXDB.
#[derive(Debug)]
pub enum LxdbError {
    InvalidWeight,
    InvalidToken,
    InvalidRelation,
    InvalidDataset,
}
