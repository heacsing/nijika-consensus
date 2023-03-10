use super::HashValue;

pub enum NijikaMessageType {
    PRE_PREPARE,
    PREPARE,
    COMMIT,
    REPLY,
}

pub trait NijikaPBFTMessageAttachment {}

pub struct NijikaPBFTMessage {
    source_node: HashValue,
    round_num: u64,
    message_type: NijikaMessageType,
    control_block: HashValue,
    appendix: Box<dyn NijikaPBFTMessageAttachment>
}

pub struct NijikaVote {
    id: HashValue,
    result: bool,
}
impl NijikaPBFTMessageAttachment for NijikaVote {}