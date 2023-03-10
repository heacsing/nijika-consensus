use crate::primitives::NijikaControlBlockT;

pub trait NijikaApi {
    fn get_control_block(&mut self, round: u64) -> Box<dyn NijikaControlBlockT>;
}