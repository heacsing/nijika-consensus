use std::fmt::Display;

use serde::{Serialize, Serializer};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ByteArray<const L: usize>([u8; L]);

impl<const L: usize> Serialize for ByteArray<L> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer
    {
        serializer.serialize_bytes(&self.0)
    }
}

impl<const L: usize> Display for ByteArray<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("0x")?;
        self.0.iter().try_for_each(|v| v.fmt(f))
    }
}

impl<const L: usize> ByteArray<L> {
    pub fn new(data: [u8;L]) -> Self {
        Self(data)
    }

    pub fn default() -> Self {
        Self([0; L])
    }
}
pub type HashValue = ByteArray<64>;
pub type Signature = ByteArray<256>;
pub type Transaction = ByteArray<512>;