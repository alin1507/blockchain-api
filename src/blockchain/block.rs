use crypto_hash::{hex_digest, Algorithm};
use serde::{Deserialize, Serialize};
use std::{time::SystemTime};

use crate::blockchain::transaction::Transaction;

//CONTAINS INFORMATION ABOUT A BLOCK FROM THE BLOCKCHAIN
#[derive(Clone)]
pub struct Block {
    pub index: usize,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub hash: String,
    pub previous_hash: String,
    pub nonce: usize,
}

//CONTAINS INFORMATION THAT CAN BE SEEN BY THE USER IN THE BLOCKCHAIN
#[derive(Deserialize, Serialize, Debug)]
pub struct BlockTransaction {
    pub from: String,
    pub to: String,
    pub amount: u32,
}

impl Block {
    //CREATE A NEW BLOCK
    pub fn new(index: usize, transactions: &Vec<Transaction>) -> Self {
        let mut new_block = Block {
            index,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            transactions: transactions.to_vec(),
            hash: String::new(),
            previous_hash: String::new(),
            nonce: 0,
        };

        new_block.set_hash();
        new_block
    }

    //SET BLOCK PREVIOUS HASH
    pub fn set_previous_hash(&mut self, previous_hash: &str) {
        self.previous_hash = previous_hash.to_string();
    }

    //SET HASH VALUE
    pub fn set_hash(&mut self) {
        self.hash = self.calculate_hash();
    }

    //CALCULATE HASH WITH SHA256 BASED OT THE BLOCK INFO
    pub fn calculate_hash(&self) -> String {
        let mut transactions_string: Vec<String> = vec![];

        //ADD TRANSACTION INFO INTO A VEC AS STRINGS
        for transaction in &self.transactions {
            transactions_string.push(format!(
                "{}{}",
                transaction.from_wallet.address, transaction.from_wallet.address
            ));
            transactions_string.push(format!(
                "{}{}",
                transaction.to_wallet.address, transaction.to_wallet.address
            ));
            transactions_string.push(transaction.amount.to_string());
        }

        //ADD ALL INFOS ABOUT THE TRANSACTION INTO A STRING
        let hash_string = format!(
            "{}{}{}{}{}",
            self.index,
            self.timestamp.to_string(),
            transactions_string.join(""),
            self.previous_hash,
            self.nonce.to_string()
        );

        //CREATE AND RETURN THE HASH
        let hash_bytes = hash_string.as_bytes();
        hex_digest(Algorithm::SHA256, hash_bytes)
    }

    //MINE BLOCK BASED ON THE BLOCKCHAIN DIFFICULTY
    pub fn mine_block(&mut self, difficulty: usize) {
        while &self.hash[0..difficulty] != vec!["0"; difficulty].join("") {
            self.nonce += 1;
            self.set_hash();
        }
    }
}