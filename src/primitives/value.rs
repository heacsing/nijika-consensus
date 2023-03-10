#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ByteArray<const L: usize>([u8; L]);

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