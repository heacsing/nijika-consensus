use std::{sync::Mutex, collections::HashMap};

use crate::primitives::*;

use super::blockchain::M;

pub struct NijikaTestControlBlock {
    block_type: NijikaBlockType,
    round_num: u64,
    pre_hash: HashValue,
    seed: u64,
    seed_proof: Vec<u8>,
    proposer_id: HashValue,
    signature: Signature,
    data_block_pointers: [HashValue; 32876],
}

impl NijikaBlockT for NijikaTestControlBlock {
    fn get_type(&self) -> &NijikaBlockType {
        &self.block_type
    }
    fn get_round(&self) -> u64 {
        self.round_num
    }
}

impl NijikaControlBlockT for NijikaTestControlBlock {
    fn get_seed(&self) -> u64 {
        self.seed
    }
    fn get_proposer(&self) -> &HashValue {
        &self.proposer_id
    }
}
impl NijikaPBFTMessageAttachment for NijikaTestControlBlock {}

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
            data_block_pointers: [HashValue::default(); 32876],
        }
    }
}

pub struct NijikaTestDataBlock {
    block_type: NijikaBlockType,
    round_num: u64,
    packer_id: HashValue,
    signature: Signature,
    merkle_route: [HashValue; 2*M - 1],
    transaction: [Transaction; M],
}

impl NijikaBlockT for NijikaTestDataBlock {
    fn get_type(&self) -> &NijikaBlockType {
        &self.block_type
    }
    fn get_round(&self) -> u64 {
        self.round_num
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
