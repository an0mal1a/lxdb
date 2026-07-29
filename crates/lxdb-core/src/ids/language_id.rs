#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct LanguageId(pub u16);

impl LanguageId {
    pub const fn new(id: u16) -> Self {
        Self(id)
    }

    pub const fn value(self) -> u16 {
        self.0
    }
}