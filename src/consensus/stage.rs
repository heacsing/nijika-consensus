use std::rc::Rc;

use crate::primitives::{
    NijikaNodeT,
    NijikaNodeRole,
    NijikaResult,
    NijikaPBFTMessage,
    NijikaPBFTMessageType,
    NijikaPBFTStage,
    NijikaControlBlockT, NijikaError, NijikaVote
};

pub trait NijikaPBFTStageApi: NijikaNodeT {
    fn check(&self, role: NijikaNodeRole, stage: NijikaPBFTStage) -> NijikaResult<()> {
        let current_round = self.get_round();
        let current_role = current_round.get_role();
        let current_stage = current_round.get_stage();
        if role != current_role {
            return Err(NijikaError::MismatchedRole(current_role, role));
        }
        if stage != current_stage {
            return Err(NijikaError::MismatchedStage(current_stage, stage));
        }
        Ok(())
    }
    fn pre_prepare(&mut self) -> NijikaResult<()> {
        self.check(NijikaNodeRole::PROPOSER, NijikaPBFTStage::PrePrepare)?;
        let control_block = self.new_control_block();
        self.set_round_control_block(Rc::clone(&control_block))?;
        let control_block_hash = control_block.hash()?;
        let pbft_msg = NijikaPBFTMessage::new_control_block_message(
            self.get_id(),
            self.get_round_num(),
            NijikaPBFTMessageType::PrePrepare,
            control_block_hash,
            control_block
        );
        let pbft_msg_hash = pbft_msg.hash()?;
        self.append_pbft_message_queue(pbft_msg_hash)?;
        self.insert_pbft_message_pool(pbft_msg_hash, pbft_msg)?;
        self.set_stage(NijikaPBFTStage::Prepare)?;
        println!("[Complete PrePrepare]");
        Ok(())
    }
    fn handle_pre_prepare(&mut self, control_block: Rc<dyn NijikaControlBlockT>) -> NijikaResult<()> {
        println!("[Handle PrePrepare]");
        self.set_vrf_seed(control_block.get_seed())?;
        self.set_round_control_block(control_block)?;
        self.set_stage(NijikaPBFTStage::Prepare)?;
        self.prepare()
    }




    fn prepare(&mut self) -> NijikaResult<()> {
        self.check(NijikaNodeRole::VALIDATOR, NijikaPBFTStage::Prepare)?;
        let control_block = self.get_round_control_block();
        let control_block_hash = control_block.hash()?;
        let pbft_msg = NijikaPBFTMessage::new_vote_message(
            self.get_id(),
            self.get_round_num(),
            NijikaPBFTMessageType::Prepare,
            control_block_hash,
            NijikaVote::new_true(self.get_id())
        );
        let pbft_msg_hash = pbft_msg.hash()?;
        self.append_pbft_message_queue(pbft_msg_hash)?;
        self.insert_pbft_message_pool(pbft_msg_hash, pbft_msg)?;
        self.broadcast_hash_message(pbft_msg_hash, None)?;
        self.try_set_stage(NijikaPBFTStage::Commit)?;
        Ok(())
    }
    fn handle_prepare(&mut self, vote_result: bool) -> NijikaResult<()> {
        println!("[Handle Prepare]");
        if vote_result {
            let current_round = self.get_round_mut();
            current_round.vote_inc(NijikaPBFTStage::Prepare)?;
        }
        self.try_set_stage(NijikaPBFTStage::Commit)
    }




    fn commit(&mut self) -> NijikaResult<()> {
        self.check(NijikaNodeRole::PROPOSER, NijikaPBFTStage::Commit)?;
        let control_block = self.get_round_control_block();
        let control_block_hash = control_block.hash()?;
        let pbft_msg = NijikaPBFTMessage::new_vote_message(
            self.get_id(),
            self.get_round_num(),
            NijikaPBFTMessageType::Commit,
            control_block_hash,
            NijikaVote::new_true(self.get_id())
        );
        let pbft_msg_hash = pbft_msg.hash()?;
        self.append_pbft_message_queue(pbft_msg_hash)?;
        self.insert_pbft_message_pool(pbft_msg_hash, pbft_msg)?;
        self.broadcast_hash_message(pbft_msg_hash, None)?;
        self.try_set_stage(NijikaPBFTStage::Reply)?;
        Ok(())
    }
    fn handle_commit(&mut self, vote_result: bool) -> NijikaResult<()> {
        println!("[Handle Commit]");
        if vote_result {
            let current_round = self.get_round_mut();
            current_round.vote_inc(NijikaPBFTStage::Commit)?;
        }
        self.try_set_stage(NijikaPBFTStage::Reply)
    }




    fn reply(&mut self) -> NijikaResult<()> {
        self.check(NijikaNodeRole::PROPOSER, NijikaPBFTStage::Reply)?;
        let control_block = self.get_round_control_block();
        let control_block_hash = control_block.hash()?;
        let pbft_msg = NijikaPBFTMessage::new_control_block_message(
            self.get_id().clone(),
            self.get_round_num(),
            NijikaPBFTMessageType::Prepare,
            control_block_hash,
            control_block
        );
        let pbft_msg_hash = pbft_msg.hash()?;
        self.append_pbft_message_queue(pbft_msg_hash)?;
        self.insert_pbft_message_pool(pbft_msg_hash, pbft_msg)?;
        self.broadcast_hash_message(pbft_msg_hash, None)?;
        println!("[Round Completed]");
        self.end_round()?;
        Ok(())
    }
    fn handle_reply(&mut self, control_block: Rc<dyn NijikaControlBlockT>) -> NijikaResult<()> {
        println!("[Handle Reply]");
        if control_block.hash()? == self.get_round_control_block().hash()? {
            let current_round = self.get_round_mut();
            current_round.vote_inc(NijikaPBFTStage::Reply)?;
        }
        self.try_end_round()
    }




    fn pack(&mut self) -> NijikaResult<()> {
        self.check(NijikaNodeRole::PACKER, NijikaPBFTStage::Packing)?;
        let data_block = self.new_data_block();
        let data_block_hash = data_block.hash()?;
        self.append_data_block_hash_queue(data_block_hash)?;
        self.insert_data_block_pool(data_block_hash, data_block)?;
        self.broadcast_hash_message(data_block_hash, None)?;
        self.set_stage(NijikaPBFTStage::WaitReply)?;
        println!("[Complete Pack]");
        Ok(())
    }
}
