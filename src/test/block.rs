use std::{sync::Mutex, collections::HashMap};

use serde::{Serialize, Deserialize};
use serde_big_array::BigArray;
use bincode;

use crate::primitives::*;
use crate::hash::hash;

use super::blockchain::M;
#[derive(Debug, Serialize,Clone, Deserialize)]
pub struct NijikaTestControlBlock {
    block_type: NijikaBlockType,
    round_num: u64,
    pre_hash: HashValue,
    seed: u64,
    seed_proof: Vec<u8>,
    proposer_id: HashValue,
    signature: Signature,
    data_block_pointers: Vec<HashValue>,
}

impl NijikaBlockT for NijikaTestControlBlock {
    fn get_type(&self) -> &NijikaBlockType {
        &self.block_type
    }
    fn get_round(&self) -> u64 {
        self.round_num
    }
    fn as_bytes(&self) -> NijikaResult<Vec<u8>> {
        if let Ok(content) = bincode::serialize(self) {
            Ok(content)
        } else {
            Err(NijikaError::ParseError(format!("Parse Error: {:#?}", self)))
        }
    }
    fn hash(&self) -> NijikaResult<HashValue> {
        match self.as_bytes() {
            Ok(content) => Ok(hash::new(&content)),
            Err(e) => Err(e)
        }
    }
}


impl NijikaControlBlockT for NijikaTestControlBlock {
    fn get_seed(&self) -> u64 {
        self.seed
    }
    fn get_pre_hash(&self) -> &HashValue {
        &self.pre_hash
    }
    fn get_proposer(&self) -> &HashValue {
        &self.proposer_id
    }
}

impl NijikaTestControlBlock {
    fn new(node_id: HashValue, round: u64, pre_hash: HashValue) -> Self {
        NijikaTestControlBlock {
            block_type: NijikaBlockType::CONTROL,
            round_num: round,
            pre_hash: pre_hash,
            seed: 0,
            seed_proof: vec![0],
            proposer_id: node_id,
            signature: Signature::default(),
            data_block_pointers: vec![],
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NijikaTestDataBlock {
    block_type: NijikaBlockType,
    round_num: u64,
    packer_id: HashValue,
    signature: Signature,
    #[serde(with = "BigArray")]
    merkle_route: [HashValue; 2*M - 1],
    #[serde(with = "BigArray")]
    transaction: [Transaction; M],
}

impl NijikaBlockT for NijikaTestDataBlock {
    fn get_type(&self) -> &NijikaBlockType {
        &self.block_type
    }
    fn get_round(&self) -> u64 {
        self.round_num
    }
    fn as_bytes(&self) -> NijikaResult<Vec<u8>> {
        if let Ok(content) = bincode::serialize(self) {
            Ok(content)
        } else {
            Err(NijikaError::ParseError(format!("Parse Error: {:#?}", self)))
        }
    }
    fn hash(&self) -> NijikaResult<HashValue> {
        match self.as_bytes() {
            Ok(content) => Ok(hash::new(&content)),
            Err(e) => Err(e)
        }
    }
}

impl NijikaDataBlockT for NijikaTestDataBlock {
    fn get_packer(&self) -> &HashValue {
        &self.packer_id
    }
}

impl NijikaTestDataBlock {
    fn new(node_id: HashValue, round_num: u64) -> Self {
        NijikaTestDataBlock {
            block_type: NijikaBlockType::DATA,
            round_num: round_num,
            packer_id: node_id,
            signature: Signature::default(),
            merkle_route: [HashValue::default();2*M-1],
            transaction: [Transaction::default();M],
        }
    }
}

pub type DataBlockPool<'a> = Mutex<HashMap<HashValue, &'a NijikaTestDataBlock>>;
