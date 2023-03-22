use super::{NijikaNodeRole, NijikaPBFTStage};

pub type NijikaResult<T> = Result<T, NijikaError>;
#[derive(Debug)]
pub enum NijikaError {
    TooLessVote,
    InvalidControlBlock(String),
    InvalidPBFTMessage(String),
    VRFError(String),
    ParseError(String),
    IncorrectStage(NijikaPBFTStage),
    MismatchedRole(NijikaNodeRole, NijikaNodeRole),
    MismatchedStage(NijikaPBFTStage, NijikaPBFTStage),
}