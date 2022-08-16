use crate::{block::Block, Person, Transaction};
use std::{fmt, time::SystemTime};

pub struct BlockChain {
    pub chain: Vec<Block>,
    dificulty: usize,
    pending_transactions: Vec<Transaction>,
    mining_reward: u32,
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
    pub fn new() -> Self {
        BlockChain {
            chain: vec![],
            dificulty: 2,
            pending_transactions: vec![],
            mining_reward: 100,
        }
    }

    //TODO: maybe remove this function
    fn create_genesis_block(transactions: &Vec<Transaction>) -> Block {
        Block::new(transactions)
    }

    pub fn get_latest_block(&mut self) -> Option<&Block> {
        self.chain.last()
    }

    pub fn get_latest_block_as_mutable(&mut self) -> Option<&mut Block> {
        self.chain.last_mut()
    }

    // pub fn add_block(&mut self, transaction: Transaction) {
    //     let latest_block = self.get_latest_block_as_mutable();

    //     match latest_block {
    //         Some(latest_block) => {
    //             let mut new_block = Block::new(transaction);

    //             new_block.set_previous_hash(&latest_block.hash);
    //             new_block.mine_block(self.dificulty);

    //             self.chain.push(new_block);
    //         }
    //         None => {}
    //     };
    // }

    pub fn mine_pending_transactions(&mut self, mining_reward_address: String) {

        let latest_block = self.get_latest_block_as_mutable();

        let mut block = Block::new(&self.pending_transactions);


        // match latest_block {
        //     Some(latest_block) => {
        //         block.set_previous_hash(&latest_block.hash);
        //     }
        //     None => {}
        // };

        block.mine_block(self.dificulty);

        println!("Block succesfully mined");

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

    pub fn get_balance_of_adress(&self, address: String) -> u32 {
        let mut balance = 0;

        for block in &self.chain {
            for transaction in &block.transactions {
                if transaction.from_adress == address {
                    balance -= transaction.amount_transfered;
                }

                if transaction.to_adress == address {
                    balance += transaction.amount_transfered;
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
