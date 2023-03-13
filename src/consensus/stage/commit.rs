use crate::primitives::{
    NijikaNodeT,
    NijikaResult,
    NijikaNodeType,
    NijikaPBFTStage,
    NijikaPBFTMessage, NijikaMessageType
};

pub fn commit(mut node: Box<dyn NijikaNodeT>) -> NijikaResult<()> {
    node.check(NijikaNodeType::PROPOSER, NijikaPBFTStage::Commit)?;
    let control_block = node.get_round_control_block();
    let control_block_hash = control_block.hash()?;
    let pbft_msg = NijikaPBFTMessage::new_vote_message(
        node.get_id().clone(),
        node.get_round_num(),
        NijikaMessageType::COMMIT,
        control_block_hash
    );
    let pbft_msg_hash = pbft_msg.hash()?;
    node.append_pbft_message_queue(pbft_msg_hash)?;
    node.insert_pbft_message_pool(pbft_msg_hash, pbft_msg)?;
    node.broadcast_message_hash(pbft_msg_hash)?;
    node.try_set_stage(NijikaPBFTStage::Reply)?;
    Ok(())
}

pub fn handle_commit(mut node: Box<dyn NijikaNodeT>, vote: bool) -> NijikaResult<()> {
    println!("[Handle Commit]");
    if vote {
        let current_round = node.get_round_mut();
        current_round.vote_inc(NijikaPBFTStage::Commit)?;
    }
    node.try_set_stage(NijikaPBFTStage::Reply)
}