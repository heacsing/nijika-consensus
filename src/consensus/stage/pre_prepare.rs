use crate::primitives::{NijikaNodeT, NijikaNodeType, NijikaPBFTStage, NijikaError};

pub fn pre_prepare(node: Box<dyn NijikaNodeT>) -> Result<(), NijikaError> {
    let current_round = node.get_round();
    let role = current_round.get_role();
    let stage = current_round.get_stage();
    if role != NijikaNodeType::PROPOSER {
        return Err(NijikaError::MISMATCHED_ROLE(role, NijikaNodeType::PROPOSER));
    }
    if stage != NijikaPBFTStage::PRE_PREPARE {
        return Err(NijikaError::MISMATCHED_STAGE(stage, NijikaPBFTStage::PRE_PREPARE));
    }
    Ok(())
}