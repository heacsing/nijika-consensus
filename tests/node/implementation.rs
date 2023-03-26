use std::collections::HashMap;
use super::*;
use nijika::{NijikaPBFTStageApi, NijikaPBFTMessageApi};

use crate::{block::{NijikaTestControlBlock, NijikaTestDataBlock}, conf::TotalWeights};


impl<'a> NijikaPBFTStageApi<'a, NijikaTestControlBlock, NijikaTestDataBlock, HashValue> for NijikaTestNode {}
impl<'a> NijikaPBFTMessageApi<'a, NijikaTestControlBlock, NijikaTestDataBlock, HashValue> for NijikaTestNode {}

impl<'a> NijikaNodeT<'a, NijikaTestControlBlock, NijikaTestDataBlock, HashValue> for NijikaTestNode {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_ip(&self) -> &str {
        &self.ip
    }

    fn get_id(&self) -> HashValue {
        self.id
    }

    fn get_role(&self) -> NijikaNodeRole {
        self.nijika_round.get_role()
    }

    fn get_weight(&self) -> u64 {
        let round_num = self.get_round_num();
        let len = self.moneys.len();
        if len as u64 <= round_num {
            0
        } else {
            self.moneys[round_num as usize]
        }
    }

    fn get_total_weight(&self) -> u64 {
        TotalWeights
    }

    fn get_vrf_params(&self) -> (u64, u64) {
        (self.nijika_round.get_expected(), self.total_weight)
    }

    fn get_peer_info_mut(&mut self) -> &mut HashMap<HashValue, (String, String)> {
        &mut self.peer_nodes
    }

    fn get_hash_queue(&self, identifier: Option<&str>) -> NijikaResult<&Vec<HashValue>> {
        match identifier {
            Some("data_block") => Ok(&self.data_block_hash_queue),
            Some("pbft_msg") => Ok(&self.pbft_msg_hash_queue),
            _ => Err(NijikaError::ParseError(format!("unknown identifier")))
        }
    }

    fn get_hash_queue_mut(&mut self, identifier: Option<&str>) -> NijikaResult<&mut Vec<HashValue>> {
        match identifier {
            Some("data_block") => Ok(&mut self.data_block_hash_queue),
            Some("pbft_msg") => Ok(&mut self.pbft_msg_hash_queue),
            _ => Err(NijikaError::ParseError(format!("unknown identifier")))
        }
    }

    fn get_vrf_seed(&self) -> u64 {
        self.vrf_seed
    }

    fn set_vrf_seed(&mut self, seed: u64) -> () {
        self.vrf_seed = seed;
    }

    fn get_secret_key(&self) -> &[u8] {
        &self.vrf_secret_key
    }

    fn get_public_key(&self) -> &[u8] {
        &self.vrf_public_key
    }

    fn set_keys(&mut self, private_key: Vec<u8>, public_key: Vec<u8>) -> () {
        self.vrf_public_key = public_key;
        self.vrf_secret_key = private_key;
    }

    fn update_proof(&mut self, proof: Vec<u8>, hash: Vec<u8>) -> NijikaResult<()> {
        self.vrf_proof = proof;
        self.vrf_hash = hash;
        Ok(())
    }

    fn set_round(&mut self, round: NijikaRound<NijikaTestControlBlock>) -> NijikaResult<()> {
        self.nijika_round = round;
        Ok(())
    }

    fn get_round(&self) -> &NijikaRound<NijikaTestControlBlock> {
        &self.nijika_round
    }

    fn get_round_mut(&mut self) -> &mut NijikaRound<NijikaTestControlBlock> {
        &mut self.nijika_round
    }

    fn get_round_num(&self) -> u64 {
        self.nijika_round.get_round_num()
    }

    fn set_round_control_block(&mut self, block: NijikaTestControlBlock) -> NijikaResult<()> {
        self.nijika_round.set_control_block(block);
        Ok(())
    }

    fn get_round_control_block(&mut self) -> &NijikaTestControlBlock {
        self.nijika_round.get_control_block().expect("empty block in the round")
    }

    fn new_control_block(&self) -> NijikaTestControlBlock {
        let last_block = self.ledger.last().expect("unable to access to latest control block");
        let pre_hash = last_block.hash().unwrap();
        let mut current_block = NijikaTestControlBlock::new(self.id, self.get_round_num(), pre_hash);
        current_block.set_seed(self.get_vrf_seed());
        current_block
    }

    fn load_control_block(&mut self, block: &mut NijikaTestControlBlock) -> NijikaResult<()> {
        let len = self.data_block_hash_queue.len();
        let num = if len > 300 {300} else {len};
        for _i in 0..num {
            match self.data_block_hash_queue.pop() {
                Some(b) => block.push(b),
                _ => ()
            }
        }
        Ok(())
    }

    fn commit_control_block(&mut self, block: NijikaTestControlBlock) -> NijikaResult<()> {
        self.ledger.push(block);
        Ok(())
    }

    fn new_data_block(&self) -> NijikaTestDataBlock {
        NijikaTestDataBlock::new(self.id, self.get_round_num())
    }

    fn append_data_block_hash_queue(&mut self, hash: HashValue) -> NijikaResult<()> {
        self.data_block_hash_queue.push(hash);
        Ok(())
    }

    fn insert_data_block_pool(&mut self, hash: HashValue, block: NijikaTestDataBlock) -> NijikaResult<()> {
        match self.safe_data_block_pool.insert(hash, block) {
            Some(b) => Err(NijikaError::HashCollision(hash)),
            None => Ok(())
        }
    }

    fn append_pbft_message_queue(&mut self, hash: HashValue) -> NijikaResult<()> {
        self.pbft_msg_hash_queue.push(hash);
        Ok(())
    }

    fn insert_pbft_message_pool(&mut self, hash: HashValue, message: NijikaPBFTMessage<NijikaTestControlBlock, HashValue>) -> NijikaResult<()> {
        match self.safe_pbft_message_pool.insert(hash, message) {
            Some(b) => Err(NijikaError::HashCollision(hash)),
            None => Ok(())
        }
    }

    fn broadcast_hash_message(&self, hash: HashValue, source: Option<HashValue>) -> NijikaResult<()> {
        todo!()
    }
}