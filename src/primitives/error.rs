use super::{NijikaNodeType, NijikaPBFTStage};

pub type NijikaResult<T> = Result<T, NijikaError>;

pub enum NijikaError {
    ParseError(String),
    IncorrectStage(NijikaPBFTStage),
    MismatchedRole(NijikaNodeType, NijikaNodeType),
    MismatchedStage(NijikaPBFTStage, NijikaPBFTStage),
}