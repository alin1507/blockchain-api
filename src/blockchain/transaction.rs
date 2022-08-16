use crypto_hash::{hex_digest, Algorithm};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub from_adress: String,
    pub to_adress: String,
    pub amount_transfered: u32
}

impl Transaction {
    pub fn new(from_adress: String, to_adress: String, amount_transfered: u32) -> Self {
        Transaction {
            amount_transfered,
            from_adress,
            to_adress
        }
    }

    pub fn calculate_hash(&self) -> String {
        let string_transaction = format!(
            "{}{}{}",
            self.from_adress,
            self.to_adress,
            self.amount_transfered.to_string()
        );

        hex_digest(Algorithm::SHA256, string_transaction.as_bytes())
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}",
            self.amount_transfered, self.from_adress, self.to_adress
        )
    }
}
