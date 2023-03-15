use std::{sync::Mutex, rc::Rc};

use super::{HashValue, NijikaNodeRole, NijikaControlBlockT, NijikaResult, NijikaError};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NijikaPBFTStage {
    PrePrepare,
    Prepare,
    Commit,
    Reply,
    WaitPrePrepare,
    Packing,
    WaitReply
}

pub struct NijikaRound {
    round_num: u64,
    role: NijikaNodeRole,
    stage: NijikaPBFTStage,
    prepare_vote: u64,
    commit_vote: u64,
    reply_vote: u64,
    end: Mutex<bool>,
    control_block: Option<Rc<dyn NijikaControlBlockT>>
}

impl NijikaRound {
    pub fn new(round_num: u64, role: NijikaNodeRole, stage: NijikaPBFTStage) -> Self {
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
    pub fn get_role(&self) -> NijikaNodeRole {
        self.role
    }
    pub fn get_stage(&self) -> NijikaPBFTStage {
        self.stage
    }

    pub fn vote_inc(&mut self, stage: NijikaPBFTStage) -> NijikaResult<()> {
        match stage {
            NijikaPBFTStage::Prepare => {
                self.prepare_vote += 1;
                Ok(())
            },
            NijikaPBFTStage::Commit => {
                self.commit_vote += 1;
                Ok(())
            },
            NijikaPBFTStage::Reply => {
                self.reply_vote += 1;
                Ok(())
            },
            _ => {
                Err(NijikaError::IncorrectStage(stage))
            },
        }
    }
}