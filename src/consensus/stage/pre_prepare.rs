use std::rc::Rc;
use super::prepare;

use crate::primitives::{
    NijikaNodeT,
    NijikaNodeType,
    NijikaPBFTStage,
    NijikaError,
    NijikaResult,
    NijikaRound,
    NijikaMessageType,
    NijikaPBFTMessage, NijikaControlBlockT
};

pub fn pre_prepare(mut node: Box<dyn NijikaNodeT>) -> NijikaResult<()> {
    node.check(NijikaNodeType::PROPOSER, NijikaPBFTStage::PrePrepare)?;
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
    node.set_stage(NijikaPBFTStage::Prepare)?;
    println!("[Complete PrePrepare]");
    Ok(())
}

pub fn handle_pre_prepare(mut node: Box<dyn NijikaNodeT>, control_block: Rc<dyn NijikaControlBlockT>) -> NijikaResult<()> {
    println!("[Handle PrePrepare]");
    node.set_vrf_seed(control_block.get_seed())?;
    node.set_round_control_block(control_block)?;
    node.set_stage(NijikaPBFTStage::Prepare)?;
    prepare(node)
}