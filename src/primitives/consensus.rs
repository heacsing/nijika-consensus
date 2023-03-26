use tokio::sync::mpsc::{self, UnboundedSender, UnboundedReceiver};

use super::{NijikaNodeRole, NijikaControlBlockT, NijikaResult, NijikaError};

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
#[derive(Debug)]
pub struct NijikaRound<CB: NijikaControlBlockT> {
    thresh: u64,
    expected: u64,
    round_num: u64,
    role: NijikaNodeRole,
    stage: NijikaPBFTStage,
    prepare_vote: u64,
    commit_vote: u64,
    reply_vote: u64,
    end: (UnboundedSender<u64>, UnboundedReceiver<u64>),
    control_block: Option<CB>
}

impl<CB: NijikaControlBlockT> NijikaRound<CB> {
    pub fn new(thresh: u64, expected: u64, round_num: u64, role: NijikaNodeRole, stage: NijikaPBFTStage) -> Self {
        Self {
            thresh,
            expected,
            round_num,
            role,
            stage,
            prepare_vote: 0,
            commit_vote: 0,
            reply_vote: 0,
            end: mpsc::unbounded_channel(),
            control_block: None
        }
    }
    pub fn default() -> Self {
        Self {
            thresh: 0,
            expected: 0,
            round_num: 0,
            role: NijikaNodeRole::NORMAL,
            stage: NijikaPBFTStage::WaitPrePrepare,
            prepare_vote: 0,
            commit_vote: 0,
            reply_vote: 0,
            end: mpsc::unbounded_channel(),
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
    pub fn set_stage(&mut self, next: NijikaPBFTStage) {
        self.stage = next;
    }
    pub fn try_set_stage(&mut self, next:NijikaPBFTStage) -> NijikaResult<NijikaPBFTStage> {
        match next {
            NijikaPBFTStage::Commit => {
                if self.prepare_vote >= self.thresh - 1 {
                    if self.stage == NijikaPBFTStage::Prepare {
                        self.set_stage(NijikaPBFTStage::Commit);
                        Ok(NijikaPBFTStage::Commit)
                    } else {
                        Err(NijikaError::IncorrectStage(self.stage))
                    }
                } else {
                    Err(NijikaError::TooLessVote)
                }
            }
            NijikaPBFTStage::Reply => {
                if self.prepare_vote >= self.thresh {
                    if self.stage == NijikaPBFTStage::Commit {
                        self.set_stage(NijikaPBFTStage::Reply);
                        Ok(NijikaPBFTStage::Reply)
                    } else {
                        Err(NijikaError::IncorrectStage(self.stage))
                    }
                } else {
                    Err(NijikaError::TooLessVote)
                }
            }
            _ => Err(NijikaError::IncorrectStage(next))
        }
    }
    pub fn get_expected(&self) -> u64 {
        self.expected
    }
    pub fn set_expected(&mut self, value: u64) -> NijikaResult<()> {
        self.expected = value;
        Ok(())
    }
    pub fn get_control_block(&self) -> Option<&CB> {
        match &self.control_block {
            Some(block) => Some(block),
            None => None
        }
    }
    pub fn set_control_block(&mut self, block: CB) -> () {
        self.control_block = Some(block);
    }
    pub fn end(&mut self) {
        self.end.0.send(self.round_num).expect("cannot dictate round end");
    }
    pub async fn recv(&mut self) -> Option<u64> {
        self.end.1.recv().await
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