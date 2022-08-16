use crypto_hash::{hex_digest, Algorithm};
use std::{fmt, time::SystemTime};

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
    timestamp: SystemTime,
    data: BlockData,
    pub hash: String,
    pub previous_hash: String,
}

impl Block {
    pub fn new(index: usize, timestamp: SystemTime, data: BlockData) -> Self {
        let index_string = index.to_string();
        let index_str = index_string.as_str();

        Block {
            index,
            timestamp,
            data,
            hash: hex_digest(Algorithm::SHA256, index_str.as_bytes()),
            previous_hash: String::new(),
        }
    }

    pub fn set_previous_hash(&mut self, previous_hash: &str) {
        self.previous_hash = previous_hash.to_string();
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
