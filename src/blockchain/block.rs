use crypto_hash::{hex_digest, Algorithm};
use serde::{Deserialize, Serialize};
use std::{fmt, time::SystemTime};

use crate::blockchain::transaction::Transaction;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Block {
    pub index: usize,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub hash: String,
    pub previous_hash: String,
    pub nonce: usize,
}
#[derive( Deserialize, Serialize)]
pub struct BlockInfo {
    pub index: usize,
    pub timestamp: u64,
    pub transactions: Vec<BlockTransaction>,
    pub hash: String,
    pub previous_hash: String,
}

#[derive( Deserialize, Serialize, Debug)]
pub struct BlockTransaction {
    pub from: String,
    pub to: String,
    pub amount: u32,
}

impl Block {
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

    pub fn set_previous_hash(&mut self, previous_hash: &str) {
        self.previous_hash = previous_hash.to_string();
    }

    pub fn set_hash(&mut self) {
        self.hash = self.calculate_hash();
    }

    pub fn calculate_hash(&self) -> String {
        let mut transactions_string: Vec<String> = vec![];

        for transaction in &self.transactions {
            let from_address_string = format!(
                "{}{}",
                transaction.from_wallet.address, transaction.from_wallet.address
            );

            let to_address_string = format!(
                "{}{}",
                transaction.to_wallet.address, transaction.to_wallet.address
            );

            transactions_string.push(from_address_string);
            transactions_string.push(to_address_string);
            transactions_string.push(transaction.amount.to_string());
        }

        let hash_string = format!(
            "{}{}{}{}{}",
            self.index,
            self.timestamp.to_string(),
            transactions_string.join(""),
            self.previous_hash,
            self.nonce.to_string()
        );

        let hash_bytes = hash_string.as_bytes();

        hex_digest(Algorithm::SHA256, hash_bytes)
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        while &self.hash[0..difficulty] != vec!["0"; difficulty].join("") {
            self.nonce += 1;
            self.set_hash();
        }

        println!("Block mined: {}", self.hash);
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, {:?}, {:#?}, {}, {}",
            self.index, self.timestamp, self.transactions, self.hash, self.previous_hash
        )
    }
}
