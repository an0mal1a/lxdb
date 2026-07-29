use crate::FormatError;

/// A fixed-size record encoded in the LXDB binary format.
pub trait BinaryRecord: Sized {
    const SIZE: usize;

    fn decode(bytes: &[u8]) -> Result<Self, FormatError>;
}
