use std::rc::Rc;

use crate::primitives::{
    NijikaResult,
    NijikaNodeT,
    NijikaNodeType,
    NijikaPBFTStage,
    NijikaPBFTMessage,
    NijikaMessageType,
    NijikaControlBlockT
};

pub fn reply(mut node: Box<dyn NijikaNodeT>) -> NijikaResult<()> {
    node.check(NijikaNodeType::PROPOSER, NijikaPBFTStage::Reply)?;
    let control_block = node.get_round_control_block();
    let control_block_hash = control_block.hash()?;
    let pbft_msg = NijikaPBFTMessage::new_vote_message(
        node.get_id().clone(),
        node.get_round_num(),
        NijikaMessageType::PREPARE,
        control_block_hash
    );
    let pbft_msg_hash = pbft_msg.hash()?;
    node.append_pbft_message_queue(pbft_msg_hash)?;
    node.insert_pbft_message_pool(pbft_msg_hash, pbft_msg)?;
    node.broadcast_message_hash(pbft_msg_hash)?;
    println!("[Round Completed]");
    node.end_round()?;
    Ok(())
}

pub fn handle_reply(mut node: Box<dyn NijikaNodeT>, control_block: Rc<dyn NijikaControlBlockT>) -> NijikaResult<()> {
    println!("[Handle Reply]");
    if control_block.hash()? == node.get_round_control_block().hash()? {
        let current_round = node.get_round_mut();
        current_round.vote_inc(NijikaPBFTStage::Reply)?;
    }
    node.try_end_round()
}