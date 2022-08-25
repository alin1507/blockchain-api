use super::{block_chain_errors::BlockChainError, wallet::Wallet};
use serde::{Deserialize, Serialize};

//CONTAINS INFORMATION ABOUT A TRANSACTION
#[derive(Clone)]
pub struct Transaction {
    pub from_wallet: Wallet,
    pub to_wallet: Wallet,
    pub amount: u32,
}

//CONTAINS THE INFORMATION THAT ARE REQUIRED WHEN CREATING A NEW TRANSACTION
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TransactionInfo {
    pub from_address: String,
    pub from_password: String,
    pub to_address: String,
    pub amount: u32,
}

impl Transaction {
    //CREATE A NEW TRANSACTION
    pub fn new(from_wallet: Wallet, to_wallet: Wallet, amount: u32) -> Self {
        Transaction {
            amount,
            from_wallet,
            to_wallet,
        }
    }
}

impl TransactionInfo {
    //CHECK IF THE TRANSACTION IS VALID
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
