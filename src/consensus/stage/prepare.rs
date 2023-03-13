use crate::primitives::{
    NijikaResult,
    NijikaNodeT,
    NijikaNodeType,
    NijikaPBFTStage,
    NijikaPBFTMessage,
    NijikaMessageType
};

pub fn prepare (mut node: Box<dyn NijikaNodeT>) -> NijikaResult<()> {
    node.check(NijikaNodeType::VALIDATOR, NijikaPBFTStage::Prepare)?;
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
    node.try_set_stage(NijikaPBFTStage::Commit)?;
    Ok(())
}

pub fn handle_prepare(mut node: Box<dyn NijikaNodeT>, vote: bool) -> NijikaResult<()> {
    println!("[Handle Prepare]");
    if vote {
        let current_round = node.get_round_mut();
        current_round.vote_inc(NijikaPBFTStage::Prepare)?;
    }
    node.try_set_stage(NijikaPBFTStage::Commit)
}