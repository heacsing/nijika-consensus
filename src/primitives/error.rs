use super::{NijikaNodeType, NijikaPBFTStage};

pub type NijikaResult<T> = Result<T, NijikaError>;

pub enum NijikaError {
    PARSE_ERROR(String),
    MISMATCHED_ROLE(NijikaNodeType, NijikaNodeType),
    MISMATCHED_STAGE(NijikaPBFTStage, NijikaPBFTStage),
}