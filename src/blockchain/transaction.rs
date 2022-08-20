use actix_web::{FromRequest, Error};
use crypto_hash::{hex_digest, Algorithm};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
pub struct Transaction {
    pub from_address: String,
    pub to_address: String,
    pub amount: u32
}

impl Transaction {
    pub fn new(from_address: String, to_address: String, amount: u32) -> Self {
        Transaction {
            amount,
            from_address,
            to_address
        }
    }

    pub fn calculate_hash(&self) -> String {
        let string_transaction = format!(
            "{}{}{}",
            self.from_address,
            self.to_address,
            self.amount.to_string()
        );

        hex_digest(Algorithm::SHA256, string_transaction.as_bytes())
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}",
            self.amount, self.from_address, self.to_address
        )
    }
}
