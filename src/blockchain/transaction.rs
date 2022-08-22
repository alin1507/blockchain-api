use serde::{Deserialize, Serialize};
use super::{wallet::Wallet, block_chain_errors::BlockChainError};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transaction {
    pub from_wallet: Wallet,
    pub to_wallet: Wallet,
    pub amount: u32,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct TransactionInfo {
    pub from_address: String,
    pub from_password: String,
    pub to_address: String,
    pub amount: u32,
}

impl Transaction {
    pub fn new(from_wallet: Wallet, to_wallet: Wallet, amount: u32) -> Self {
        Transaction {
            amount,
            from_wallet,
            to_wallet,
        }
    }
}

impl TransactionInfo {
    /**
     * Check if the transaction is valid
     */
    pub fn check_transaction_info(&self) -> Result<(), BlockChainError> {
        if self.from_address.is_empty() {
            return Err(BlockChainError::EmptyFromAddress);
        }

        if self.to_address.is_empty() {
            return Err(BlockChainError::EmptyToAddress);
        }

        if self.amount <= 0 {
            return Err(BlockChainError::InvalidAmount);
        }

        Ok(())
    }
}