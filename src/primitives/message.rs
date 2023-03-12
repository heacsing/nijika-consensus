use std::rc::Rc;

use serde::Serialize;

use crate::hash::hash;

use super::{HashValue, NijikaControlBlockT, NijikaError, NijikaResult};
#[derive(Debug, Serialize)]
pub enum NijikaMessageType {
    PRE_PREPARE,
    PREPARE,
    COMMIT,
    REPLY,
}

#[derive(Debug, Serialize)]
pub struct NijikaPBFTMessage {
    source_node: HashValue,
    round_num: u64,
    message_type: NijikaMessageType,
    control_block_hash: HashValue,
    control_block: Option<Rc<dyn NijikaControlBlockT>>,
    vote: Option<NijikaVote>,
}
#[derive(Debug, Serialize)]
struct NijikaVote {
    id: HashValue,
    result: bool,
}

impl NijikaVote {
    fn new_true(id: HashValue) -> Self {
        NijikaVote { id, result: true }
    }
}

impl NijikaPBFTMessage {
    pub fn new_control_block_message(source_node: HashValue, round_num: u64, message_type: NijikaMessageType, control_block_hash: HashValue, appendix: Rc<dyn NijikaControlBlockT>) -> Self {
        NijikaPBFTMessage {
            source_node,
            round_num,
            message_type,
            control_block_hash,
            control_block: Some(appendix),
            vote: None,
        }
    }

    pub fn new_vote_message(source_node: HashValue, round_num: u64, message_type: NijikaMessageType, control_block_hash: HashValue) -> Self {
        NijikaPBFTMessage {
            source_node,
            round_num,
            message_type,
            control_block_hash,
            control_block: None,
            vote: Some(NijikaVote::new_true(source_node)),
        }
    }

    pub fn hash (&self) -> NijikaResult<HashValue> {
        if let Ok(content) = bincode::serialize(self) {
            Ok(hash::new(&content))
        } else {
            Err(NijikaError::PARSE_ERROR(format!("parse error: msg {:#?}", self)))
        }
    }
}
