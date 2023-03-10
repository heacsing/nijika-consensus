use serde::Serialize;

use super::{HashValue, NijikaRound};

#[derive(Debug, Serialize, PartialEq, Eq, Clone, Copy)]
pub enum NijikaNodeType {
    NORMAL,
    PACKER,
    PROPOSER,
    VALIDATOR,
}

pub trait NijikaNodeT {
    fn get_name(&self) -> &str;
    fn get_ip(&self) -> &str;
    fn get_id(&self) -> &HashValue;
    fn get_round(&self) -> &NijikaRound;
}