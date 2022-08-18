use crate::{block::Block, Transaction};

pub struct BlockChain {
    pub chain: Vec<Block>,
    difficulty: usize,
    pending_transactions: Vec<Transaction>,
    mining_reward: u32,
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain {
            chain: vec![],
            difficulty: 3,
            pending_transactions: vec![],
            mining_reward: 100,
        }
    }

    pub fn mine_pending_transactions(&mut self, mining_reward_address: String) {
        let latest_block = self.chain.last();

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

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }

        true
    }
}
