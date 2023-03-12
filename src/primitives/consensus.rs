use std::{sync::Mutex, rc::Rc};

use super::{HashValue, NijikaNodeType, NijikaControlBlockT};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NijikaPBFTStage {
    PRE_PREPARE,
    PREPARE,
    COMMIT,
    REPLY,
    WAIT_PRE_PREPARE,
    PACKING,
    WAIT_REPLY
}

pub struct NijikaRound {
    round_num: u64,
    role: NijikaNodeType,
    stage: NijikaPBFTStage,
    prepare_vote: u64,
    commit_vote: u64,
    reply_vote: u64,
    end: Mutex<bool>,
    control_block: Option<Rc<dyn NijikaControlBlockT>>
}

impl NijikaRound {
    pub fn new(round_num: u64, role: NijikaNodeType, stage: NijikaPBFTStage) -> Self {
        Self {
            round_num,
            role,
            stage,
            prepare_vote: 0,
            commit_vote: 0,
            reply_vote: 0,
            end: Mutex::new(false),
            control_block: None
        }
    }
    pub fn get_round_num(&self) -> u64 {
        self.round_num
    }
    pub fn get_role(&self) -> NijikaNodeType {
        self.role
    }
    pub fn get_stage(&self) -> NijikaPBFTStage {
        self.stage
    }
}