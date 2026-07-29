/// Section flags.
pub mod flags {
    /// Payload is compressed.
    pub const COMPRESSED: u16 = 1 << 0;

    /// Payload is encrypted.
    pub const ENCRYPTED: u16 = 1 << 1;

    /// Payload is optional.
    pub const OPTIONAL: u16 = 1 << 2;
}
