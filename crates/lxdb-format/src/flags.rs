/// Section payload is compressed.
pub const COMPRESSED: u8 = 1 << 0;

/// Section payload is encrypted.
pub const ENCRYPTED: u8 = 1 << 1;

/// Section may be ignored by readers that do not support it.
pub const OPTIONAL: u8 = 1 << 2;

/// No section flags are enabled.
pub const NONE: u8 = 0;
