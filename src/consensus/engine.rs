use crate::core::NijikaApi;
use crate::primitives::{NijikaNodeT};

const ConsensusEngineId: &str = "NIJIKA";

pub struct NijikaEngine {
    client: Box<dyn NijikaApi>,
    // TODO: replace the latter u64 with true Transaction
    host_node: Box<dyn NijikaNodeT>,
}

impl NijikaEngine {
    fn default() -> &'static str {
        ConsensusEngineId
    }
}