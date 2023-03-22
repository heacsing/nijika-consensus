use std::fmt::Debug;
use serde::{Serialize, Deserialize};

use crate::hash::hash;

use super::{HashValue, NijikaControlBlockT, NijikaError, NijikaResult, NijikaNodeT, NijikaPBFTStage};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum NijikaPBFTMessageType {
    PrePrepare,
    Prepare,
    Commit,
    Reply,
}

#[derive(Debug, Serialize, Clone)]
pub struct NijikaPBFTMessage<CB: NijikaControlBlockT> {
    source_node: HashValue,
    round_num: u64,
    message_type: NijikaPBFTMessageType,
    control_block_hash: HashValue,
    vote: Option<NijikaVote>,
    control_block: Option<CB>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub struct NijikaVote {
    id: HashValue,
    result: bool,
}

impl NijikaVote {
    pub fn new_true(id: HashValue) -> Self {
        NijikaVote { id, result: true }
    }
    pub fn get_result(&self) -> bool {
        self.result
    }
}

impl<CB: NijikaControlBlockT + Serialize + Debug> NijikaPBFTMessage<CB> {
    pub fn new_control_block_message(source_node: HashValue, round_num: u64, message_type: NijikaPBFTMessageType, control_block_hash: HashValue, control_block: CB) -> Self {
        NijikaPBFTMessage {
            source_node,
            round_num,
            message_type,
            control_block_hash,
            control_block: Some(control_block),
            vote: None,
        }
    }

    pub fn new_vote_message(source_node: HashValue, round_num: u64, message_type: NijikaPBFTMessageType, control_block_hash: HashValue, vote: NijikaVote) -> Self {
        NijikaPBFTMessage {
            source_node,
            round_num,
            message_type,
            control_block_hash,
            control_block: None,
            vote: Some(vote),
        }
    }

    pub fn hash (&self) -> NijikaResult<HashValue> {
        if let Ok(content) = bincode::serialize(self) {
            Ok(hash::new(&content))
        } else {
            Err(NijikaError::ParseError(format!("parse error: msg {:#?}", self)))
        }
    }

    pub fn get_source(&self) -> HashValue {
        self.source_node
    }
    pub fn get_round_num(&self) -> u64 {
        self.round_num
    }
    pub fn get_type(&self) -> NijikaPBFTMessageType {
        self.message_type
    }
    pub fn get_vote(&self) -> Option<NijikaVote> {
        self.vote
    }

    pub fn get_control_block(&self) -> &Option<CB> {
        &self.control_block
    }
    pub fn get_control_block_hash(&self) -> HashValue {
        self.control_block_hash
    }
}


mod tests {

    use super::*;

    fn test_nijika_vote_with_option(/* hash: HashValue */) {
        let a = Some(NijikaVote::new_true(HashValue::random()));
        let b = bincode::serialize(&a).expect("fail 1");
        let c: Option<NijikaVote> = bincode::deserialize(&b).expect("fail 2");
        assert_eq!(a, c);
    }
    /* fn test_nijika_pbft_message(hash: HashValue) {
        let a
            = NijikaPBFTMessage::new_vote_message(hash, 12, NijikaPBFTMessageType::Prepare, HashValue::default());
        let b = bincode::serialize(&a).expect("fail 1");
        let c: NijikaPBFTMessage = bincode::deserialize(&b).expect("fail 2");
        assert_eq!(a.vote, c.vote);
    }
    #[test]
    fn combine_test() {
        let hash = HashValue::random();
        println!("{:#?}", hash);
        test_nijika_vote_with_option(hash);
        test_nijika_pbft_message(hash);
    } */
}

// untestable trait

/* pub trait NijikaMessageClientT: NijikaNodeT {
    fn from_bytes(&self, bytes: &[u8]) -> NijikaResult<NijikaPBFTMessage>;
    fn handle_message(&self, bytes: &[u8], peer_ip: &str) -> NijikaResult<()> {
        let msg = self.from_bytes(bytes)?;
        let peer_info = self.get_peer_info_mut();
        let mut msg_peer_id = HashValue::default();
        if let Some(pre_ip) = peer_info.get(&msg.source_node) {
            if pre_ip.0 == self.get_ip() {
                let tmp = (String::from(peer_ip), String::from(pre_ip.1))
                peer_info.insert(msg.source_node, tmp);
            }
            // don't use else, for 2 branches may both run one by one
            if pre_ip.0 != self.get_ip() {
                msg_peer_id = self.get_id().clone();
            }
        }
        let msg_content_hash_queue = self.get_hash_queue_mut(Some("msg_content"));
        if msg_content_hash_queue.contains(&msg.control_block_hash)
        todo!()
    }
    fn handle_
} */
