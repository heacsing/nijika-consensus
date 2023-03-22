use std::{collections::HashMap, sync::Mutex};

use crate::consensus::{NijikaPBFTStageApi, NijikaPBFTMessageApi};
use crate::primitives::{HashValue, NijikaRound, NijikaPBFTMessage, NijikaNodeT, NijikaError, NijikaResult, NijikaPBFTStage, NijikaControlBlockT};
use crate::vrf::NijikaVRFClientS;
use super::block::{DataBlockPool, NijikaTestControlBlock, NijikaTestDataBlock};
use super::conf::{TotalWeights};

type NijikaMessagePool<'a> = Mutex<HashMap<HashValue, &'a NijikaPBFTMessage<NijikaTestControlBlock>>>;
type PeerNodeMap = HashMap<HashValue, (String, String)>;



pub struct NijikaTestNode<'a> {
    name: String,
    ip: String,
    id: HashValue,
    moneys: Vec<u64>,
    total_weight: u64,
    ledger: Vec<NijikaTestControlBlock>,
    peer_nodes: PeerNodeMap,
    data_block_hash_queue: Vec<HashValue>,
    safe_data_block_pool: DataBlockPool<'a>,
    pbft_msg_hash_queue: Vec<HashValue>,
    safe_pbft_message_pool: NijikaMessagePool<'a>,
    nijika_round: NijikaRound<NijikaTestControlBlock>,
    vrf_seed: u64,
    vrf_proof: Vec<u8>,
    vrf_hash: Vec<u8>,
    public_key: Vec<u8>,
    secret_key: Vec<u8>,
}

impl<'a> NijikaTestNode<'a> {
    pub fn new(seed: u64) -> Option<Self> {
        let mut vrf_client = NijikaVRFClientS::new_raw();
        if let Ok((p1, p2)) = vrf_client.gen_keys(seed) {
            let rndm = rand::random::<u64>();
            Some(Self {
                name: format!("nijika-node-{}", rndm),
                ip: String::from("127.0.0.1:13000"),
                id: HashValue::random(),
                moneys: vec![1000],
                total_weight: TotalWeights,
                ledger: vec![],
                peer_nodes: PeerNodeMap::new(),
                data_block_hash_queue: vec![],
                safe_data_block_pool: DataBlockPool::new(HashMap::new()),
                pbft_msg_hash_queue: vec![],
                safe_pbft_message_pool: NijikaMessagePool::new(HashMap::new()),
                nijika_round: NijikaRound::default(),
                vrf_seed: rndm,
                vrf_hash: vec![],
                vrf_proof: vec![],
                public_key: p2,
                secret_key: p1,
            })
        } else {
            None
        }
    }
}

impl<'a> NijikaPBFTStageApi<'a, NijikaTestControlBlock, NijikaTestDataBlock> for NijikaTestNode<'a> {}
impl<'a> NijikaPBFTMessageApi<'a, NijikaTestControlBlock, NijikaTestDataBlock> for NijikaTestNode<'a> {}

impl<'a> NijikaNodeT<'a, NijikaTestControlBlock, NijikaTestDataBlock> for NijikaTestNode<'a> {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_ip(&self) -> &str {
        &self.ip
    }

    fn get_id(&self) -> HashValue {
        self.id
    }

    fn get_role(&self) -> crate::primitives::NijikaNodeRole {
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

    fn get_private_key(&self) -> &[u8] {
        &self.secret_key
    }

    fn get_public_key(&self) -> &[u8] {
        &self.public_key
    }

    fn set_keys(&mut self, private_key: Vec<u8>, public_key: Vec<u8>) -> () {
        self.public_key = public_key;
        self.secret_key = private_key;
    }

    fn update_proof(&mut self, proof: Vec<u8>, hash: Vec<u8>) -> NijikaResult<()> {
        self.vrf_proof = proof;
        self.vrf_hash = hash;
        Ok(())
    }

    fn commit_round(&mut self) -> NijikaResult<()> {
        let block = self.nijika_round.get_control_block().expect("empty block in the round");
        self.ledger.push(block.clone());
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

    fn end_round(&mut self) -> NijikaResult<()> {
        self.nijika_round.end();
        Ok(())
    }

    fn try_end_round(&mut self) -> NijikaResult<()> {
        self.end_round()
    }

    fn set_stage(&mut self, next: NijikaPBFTStage) -> NijikaResult<()> {
        self.nijika_round.set_stage(next);
        Ok(())
    }

    fn try_set_stage(&mut self, next: NijikaPBFTStage) -> NijikaResult<()> {
        let next = self.nijika_round.try_set_stage(next);
        match next {
            Ok(stage) => {
                if stage == NijikaPBFTStage::Commit {
                    self.commit()
                } else if stage == NijikaPBFTStage::Reply {
                    self.commit_round()?;
                    self.reply()
                } else {
                    println!("stage error: cannot enter next stage");
                    Ok(())
                }
            }
            Err(e) => {
                println!("stage error: {:#?}", e);
                Ok(())
            }
        }
    }

    fn new_control_block(&self) -> NijikaTestControlBlock {
        todo!()
    }

    fn new_data_block(&self) -> NijikaTestDataBlock {
        todo!()
    }

    fn append_data_block_hash_queue(&mut self, hash: HashValue) -> NijikaResult<()> {
        todo!()
    }

    fn insert_data_block_pool(&mut self, hash: HashValue, block: NijikaTestDataBlock) -> NijikaResult<()> {
        todo!()
    }

    fn append_pbft_message_queue(&mut self, hash: HashValue) -> NijikaResult<()> {
        todo!()
    }

    fn insert_pbft_message_pool(&mut self, hash: HashValue, message: NijikaPBFTMessage<NijikaTestControlBlock>) -> NijikaResult<()> {
        todo!()
    }

    fn broadcast_hash_message(&self, hash: HashValue, source: Option<HashValue>) -> NijikaResult<()> {
        todo!()
    }
}