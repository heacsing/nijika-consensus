use std::rc::Rc;

use crate::primitives::{
    NijikaNodeT,
    NijikaNodeType,
    NijikaPBFTStage,
    NijikaError,
    NijikaResult,
    NijikaRound,
    NijikaMessageType,
    NijikaPBFTMessage
};

pub fn pre_prepare(mut node: Box<dyn NijikaNodeT>) -> NijikaResult<()> {
    node.check(NijikaNodeType::PROPOSER, NijikaPBFTStage::PRE_PREPARE)?;
    let control_block = node.new_control_block();
    node.set_round_control_block(Rc::clone(&control_block))?;
    let control_block_hash = control_block.hash()?;
    let pbft_msg = NijikaPBFTMessage::new_control_block_message(
        node.get_id().clone(),
        node.get_round_num(),
        NijikaMessageType::PRE_PREPARE,
        control_block_hash,
        control_block
    );
    let pbft_msg_hash = pbft_msg.hash()?;
    node.append_pbft_message_queue(pbft_msg_hash)?;
    node.insert_pbft_message_pool(pbft_msg_hash, pbft_msg)?;
    node.broadcast_message_hash(pbft_msg_hash)?;
    node.set_stage(NijikaPBFTStage::PREPARE)?;
    println!("[Complete PrePrepare]");
    Ok(())
}