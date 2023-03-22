use core::fmt::{Debug, Formatter, Result as FmtResult};
use serde::{Serialize, Deserialize};
use erased_serde;


use super::{value::HashValue, NijikaResult};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub enum NijikaBlockType {
    CONTROL,
    DATA,
}

pub trait NijikaBlockT: erased_serde::Serialize {
    fn get_type(&self) -> &NijikaBlockType;
    fn get_round(&self) -> u64;
    fn hash(&self) -> NijikaResult<HashValue>;
    fn as_bytes(&self) -> NijikaResult<Vec<u8>>;
}
erased_serde::serialize_trait_object!(NijikaBlockT);

pub trait NijikaControlBlockT: NijikaBlockT {
    fn get_seed(&self) -> u64;
    fn get_pre_hash(&self) -> &HashValue;
    fn get_proposer(&self) -> &HashValue;
    // fn get_weights_sum(&self) -> u64;
}
erased_serde::serialize_trait_object!(NijikaControlBlockT);

/* impl Debug for dyn NijikaControlBlockT {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Block: proposer: {}, pre_hash: {}", self.get_proposer(), self.get_pre_hash())
    }
} */

pub trait NijikaDataBlockT: NijikaBlockT {
    fn get_packer(&self) -> &HashValue;
}
erased_serde::serialize_trait_object!(NijikaDataBlockT);