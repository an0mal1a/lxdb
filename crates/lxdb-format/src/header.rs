use crate::{magic::MAGIC, version::Version};

/// Number of bytes occupied by the binary LXDB header.
pub const HEADER_SIZE: usize = 8;

/// Header stored at the beginning of every LXDB dataset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    version: Version,
}

impl Header {
    pub const SIZE: usize = HEADER_SIZE;

    pub const fn new(version: Version) -> Self {
        Self { version }
    }

    pub const fn current() -> Self {
        Self::new(Version::CURRENT)
    }

    pub const fn version(self) -> Version {
        self.version
    }

    pub fn encode(self) -> [u8; HEADER_SIZE] {
        let mut bytes = [0_u8; HEADER_SIZE];

        bytes[0..4].copy_from_slice(&MAGIC);
        bytes[4..6].copy_from_slice(&self.version.major().to_le_bytes());
        bytes[6..8].copy_from_slice(&self.version.minor().to_le_bytes());

        bytes
    }
}

impl Default for Header {
    fn default() -> Self {
        Self::current()
    }
}

#[cfg(test)]
mod tests {
    use super::{HEADER_SIZE, Header};
    use crate::magic::MAGIC;

    #[test]
    fn encodes_current_header() {
        let bytes = Header::current().encode();

        assert_eq!(bytes.len(), HEADER_SIZE);
        assert_eq!(&bytes[0..4], &MAGIC);
        assert_eq!(u16::from_le_bytes([bytes[4], bytes[5]]), 0);
        assert_eq!(u16::from_le_bytes([bytes[6], bytes[7]]), 1);
    }

    #[test]
    fn encodes_custom_version() {
        let bytes = Header::new(crate::version::Version::new(2, 7)).encode();

        assert_eq!(u16::from_le_bytes([bytes[4], bytes[5]]), 2);
        assert_eq!(u16::from_le_bytes([bytes[6], bytes[7]]), 7);
    }
}
