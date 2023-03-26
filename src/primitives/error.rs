use super::{NijikaNodeRole, NijikaPBFTStage, HashValue};

pub type NijikaResult<T> = Result<T, NijikaError>;
#[derive(Debug)]
pub enum NijikaError {
    InitializeFailed,
    InnerChannelFailed,
    LockConflict(String),
    NetworkFail(String),
    HashCollision(HashValue),
    InsufficientDataBlock,
    TooLessVote,
    InvalidControlBlock(String),
    InvalidPBFTMessage(String),
    VRFError(String),
    ParseError(String),
    IncorrectStage(NijikaPBFTStage),
    MismatchedRole(NijikaNodeRole, NijikaNodeRole),
    MismatchedStage(NijikaPBFTStage, NijikaPBFTStage),
}