use serde::{Deserialize, Serialize};
use chrono::Local;

use crate::modules::block::BlockNum;
use crate::modules::transaction::Action;

#[derive(Deserialize, Serialize, Debug)]
pub struct BlockChain {
    pub block_lists: Vec<BlockNum>,
    pub pof_hardness: usize,
}

impl BlockChain {
    
    pub fn load(path: &str, hardness: usize) -> std::io::Result<Self> {
        Ok(Self {
            block_lists: crate::modules::block::load_or_init_chain(path, hardness)?,
            pof_hardness: hardness,
        })
    }

    
    pub fn add_block(&mut self, tr_list: Vec<Action>) -> std::io::Result<()> {
        let previous_hash = self.block_lists.last().map(|b| b.current_hash.clone()).unwrap_or_else(|| "0".into());
        let mut new_block = BlockNum {
            num: self.block_lists.len() as u32,
            timestamp: Local::now(),
            tr_list,
            previous_hash,
            current_hash: String::new(),
            nonce: 0,
        };
        new_block.mine_block(self.pof_hardness)?;
        self.block_lists.push(new_block);
        Ok(())
    }


    pub fn is_valid_chain(&self) -> bool {
        if self.block_lists.is_empty() {
            return true;
        }
        let target = "0".repeat(self.pof_hardness);
        for i in 1..self.block_lists.len() {
            let current = &self.block_lists[i];
            let previous = &self.block_lists[i - 1];
            if current.current_hash != current.calculate_hash() {
                return false;
            }
            if current.previous_hash != previous.current_hash {
                return false;
            }
            if !current.current_hash.starts_with(&target) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chain_should_be_valid_after_adding_block() {
        let mut chain = BlockChain::load("test_blocks.json", 3).unwrap();
        chain.add_block(vec![Action {
            from: "alice".into(),
            to: "bob".into(),
            time: Local::now(),
            amount: 1.0,
            digital_signature: "sig".into(),
        }]).unwrap();
        assert!(chain.is_valid_chain());
    }
}
