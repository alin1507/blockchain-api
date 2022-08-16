use crate::block::{Block, BlockData};
use std::{fmt, time::SystemTime};

pub struct BlockChain {
    pub chain: Vec<Block>,
}

pub struct GenerirError {
    message: String,
}

impl fmt::Display for GenerirError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl BlockChain {
    pub fn new(data: BlockData) -> Self {
        let gensis_block = BlockChain::create_genesis_block(data);
        BlockChain {
            chain: vec![gensis_block],
        }
    }

    //TODO: maybe remove this function
    fn create_genesis_block(data: BlockData) -> Block {
        Block::new(0, SystemTime::now(), data)
    }

    pub fn get_latest_block(&mut self) -> Option<&Block> {
        self.chain.last()
    }

    pub fn get_latest_block_as_mutable(&mut self) -> Option<&mut Block> {
        self.chain.last_mut()
    }

    pub fn add_block(&mut self, block_data: BlockData) -> Result<(), GenerirError> {
        let chain_length = self.chain.len();
        let latest_block = self.get_latest_block_as_mutable();

        match latest_block {
            Some(latest_block) => {
                let mut new_block = Block::new(chain_length, SystemTime::now(), block_data);

                new_block.set_previous_hash(&latest_block.hash);

                self.chain.push(new_block);
            }
            None => {
                return Err(GenerirError {
                    message: String::from("Genesis block is missing!"),
                });
            }
        };

        Ok(())
    }
}
