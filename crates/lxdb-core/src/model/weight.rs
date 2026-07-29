use crate::error::LxdbError;

/// Semantic similarity.
///
/// Valid values are between 0.0 and 1.0.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Weight(f32);

impl Weight {
    pub fn new(value: f32) -> Result<Self, LxdbError> {
        if (0.0..=1.0).contains(&value) { Ok(Self(value)) } else { Err(LxdbError::InvalidWeight) }
    }

    pub const fn value(self) -> f32 {
        self.0
    }
}
