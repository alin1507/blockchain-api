use serde::{Deserialize, Serialize};

use crate::blockchain::block::Block;
use crate::blockchain::transaction::Transaction;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BlockChain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub pending_transactions: Vec<Transaction>,
    pub mining_reward: u32,
    pub name: String,
}

impl Default for BlockChain {
    fn default() -> Self {
        Self {
            chain: vec![],
            difficulty: 2,
            pending_transactions: vec![],
            mining_reward: 100,
            name: "Blockchain".to_string(),
        }
    }
}

impl BlockChain {
    pub fn mine_pending_transactions(&mut self, mining_reward_address: String) {
        let latest_block = self.chain.last();

        println!("{:?}", &self);

        let mut block = Block::new(self.chain.len(), &self.pending_transactions);

        match latest_block {
            Some(latest_block) => {
                block.set_previous_hash(&latest_block.hash);
            }
            None => {}
        };

        block.mine_block(self.difficulty);

        println!("Block successfully mined");

        self.chain.push(block);
        self.pending_transactions = vec![Transaction::new(
            String::new(),
            mining_reward_address,
            self.mining_reward,
        )];
    }

    pub fn create_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    pub fn get_balance_of_address(&self, address: String) -> u32 {
        let mut balance = 0;

        for block in &self.chain {
            for transaction in &block.transactions {
                if transaction.from_address == address {
                    balance -= transaction.amount;
                }

                if transaction.to_address == address {
                    balance += transaction.amount;
                }
            }
        }

        balance
    }
}
