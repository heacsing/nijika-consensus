use super::{NijikaNodeType, NijikaPBFTStage};

pub enum NijikaError {
    MISMATCHED_ROLE(NijikaNodeType, NijikaNodeType),
    MISMATCHED_STAGE(NijikaPBFTStage, NijikaPBFTStage),
}