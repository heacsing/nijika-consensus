use crate::primitives::{
    NijikaResult,
    NijikaNodeT,
    NijikaNodeType,
    NijikaPBFTStage,
};

pub fn pack(mut node: Box<dyn NijikaNodeT>) -> NijikaResult<()> {
    node.check(NijikaNodeType::PACKER, NijikaPBFTStage::PACKING)?;
    let data_block = node.new_data_block();
    let data_block_hash = data_block.hash()?;
    node.append_data_block_hash_queue(data_block_hash)?;
    node.insert_data_block_pool(data_block_hash, data_block)?;
    node.broadcast_message_hash(data_block_hash)?;
    node.set_stage(NijikaPBFTStage::WAIT_REPLY)?;
    println!("[Complete Pack]");
    Ok(())
}