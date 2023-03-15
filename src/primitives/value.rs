use std::fmt::{Display, Formatter, Result as FmtResult};
use serde::{
    ser::{Serialize, Serializer, SerializeTuple},
    Deserialize, de::Visitor
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ByteArray<const L: usize>([u8; L]);

impl<const L: usize> Serialize for ByteArray<L> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer
    {
        serializer.serialize_bytes(&self.0)
    }
}

impl<'de, const L: usize> Deserialize<'de> for ByteArray<L> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        struct ArrayVisitor<const L: usize>;
        impl<'de, const L: usize> Visitor<'de> for ArrayVisitor<L> {
            type Value = ByteArray<L>;
            fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
                formatter.write_str("a vec<u8>")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::SeqAccess<'de>, {
                let mut res: Vec<u8> = Vec::new();
                while let Ok(Some(elem)) = seq.next_element() {
                    res.push(elem);
                }
                Ok(ByteArray::from(res))
            }
        }
        deserializer.deserialize_seq(ArrayVisitor)
    }
}


impl<const L: usize> Display for ByteArray<L> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("0x")?;
        self.0.iter().try_for_each(|v| write!(f, "{:x}", v))
    }
}

impl<const L: usize> ByteArray<L> {
    pub fn new(data: [u8;L]) -> Self {
        Self(data)
    }

    pub fn from(data: Vec<u8>) -> Self {
        let res: [u8; L] = match data.try_into() {
            Ok(a) => a,
            Err(e) => panic!("unable to convert a vec to arr, {:#?}", e)
        };
        ByteArray::new(res)
    }

    pub fn default() -> Self {
        Self([0; L])
    }

    pub fn random() -> Self {
        let mut a = Self::default();
        for i in a.0.iter_mut() {
            *i = rand::random()
        }
        a
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

pub type HashValue = ByteArray<64>;
pub type Signature = ByteArray<256>;
pub type Transaction = ByteArray<512>;


mod tests {
    use super::*;
    #[test]
    fn work() {
        let a = HashValue::random();
        println!("proto: {}", a);
        let b = bincode::serialize(&a).expect("serialize fail");
        println!("serialize: {:#?}, len: {}", b, b.len());
        let c: HashValue = bincode::deserialize(&b).expect("deserialize fail");
        println!("deserialize: {}", c);
    }
}