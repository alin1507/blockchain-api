use crypto_hash::{hex_digest, Algorithm};
use std::fmt;

#[derive(Debug)]
pub struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    pub fn new(first_name: String, last_name: String) -> Self {
        Person {
            first_name,
            last_name,
        }
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.first_name, self.last_name)
    }
}

#[derive(Debug)]
pub struct BlockData {
    amount_transfered: u32,
    sender: Person,
    receiver: Person,
}

impl BlockData {
    pub fn new(amount_transfered: u32, sender: Person, receiver: Person) -> Self {
        BlockData {
            amount_transfered,
            sender,
            receiver,
        }
    }
}

impl fmt::Display for BlockData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}",
            self.amount_transfered, self.sender, self.receiver
        )
    }
}

#[derive(Debug)]
pub struct Block {
    index: usize,
    timestamp: u64,
    data: BlockData,
    pub hash: String,
    pub previous_hash: String,
}

impl Block {
    pub fn new(index: usize, timestamp: u64, data: BlockData) -> Self {
        let mut new_block = Block {
            index,
            timestamp,
            data,
            hash: String::new(),
            previous_hash: String::new(),
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
        let sender_string = format!(
            "{}{}",
            self.data.sender.first_name, self.data.sender.last_name
        );
        let receiver_string = format!(
            "{}{}",
            self.data.receiver.first_name, self.data.receiver.last_name
        );

        let hash_string = format!(
            "{}{}{}{}{}{}",
            self.index.to_string(),
            self.timestamp.to_string(),
            self.data.amount_transfered.to_string(),
            sender_string,
            receiver_string,
            self.previous_hash
        );

        let hash_bytes = hash_string.as_bytes();

        hex_digest(Algorithm::SHA256, hash_bytes)
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, {:?}, {}, {}, {}",
            self.index, self.timestamp, self.data, self.hash, self.previous_hash
        )
    }
}
