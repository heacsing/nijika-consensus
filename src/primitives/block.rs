use super::value::HashValue;

pub enum NijikaBlockType {
    CONTROL,
    DATA,
}

pub trait NijikaBlockT {
    fn get_type(&self) -> &NijikaBlockType;
    fn get_round(&self) -> u64;
}

pub trait NijikaControlBlockT {
    fn get_seed(&self) -> u64;
    fn get_proposer(&self) -> &HashValue;
    // fn get_weights_sum(&self) -> u64;
}

pub trait NijikaDataBlockT {
    fn get_packer(&self) -> &HashValue;
}