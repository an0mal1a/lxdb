/// Version of the LXDB binary format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    major: u16,
    minor: u16,
}

impl Version {
    pub const CURRENT: Self = Self::new(0, 1);

    pub const fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }

    pub const fn major(self) -> u16 {
        self.major
    }

    pub const fn minor(self) -> u16 {
        self.minor
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::CURRENT
    }
}
