use std::sync::Mutex;

use super::transaction::{TransactionError, TransactionInfo};
use super::wallet::{Wallet, WalletError, WalletInfo};
use crate::blockchain::block::Block;
use crate::blockchain::transaction::Transaction;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref BLOCKCHAIN: Mutex<BlockChain> = Mutex::new(BlockChain::default());
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BlockChain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub pending_transactions: Vec<Transaction>,
    pub mining_reward: u32,
    pub name: String,
    pub wallets: Vec<Wallet>,
}

impl Default for BlockChain {
    fn default() -> Self {
        Self {
            chain: vec![],
            difficulty: 2,
            pending_transactions: vec![],
            mining_reward: 100,
            name: "Blockchain".to_string(),
            wallets: vec![],
        }
    }
}

#[derive(Debug, Display)]
pub enum BlockchainError {
    NoPendingTransactions,
    AddressNotFound,
    ChainIsEmpty,
    InvalidRewardAddress,
}

impl ResponseError for BlockchainError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            BlockchainError::NoPendingTransactions => StatusCode::NOT_FOUND,
            BlockchainError::AddressNotFound => StatusCode::NOT_FOUND,
            BlockchainError::ChainIsEmpty => StatusCode::NOT_FOUND,
            BlockchainError::InvalidRewardAddress => StatusCode::NOT_FOUND,
        }
    }
}

impl BlockChain {
    pub fn mine_pending_transactions(
        &mut self,
        mining_reward_address: &String,
    ) -> Result<(), BlockchainError> {
        if self.pending_transactions.is_empty() {
            return Err(BlockchainError::NoPendingTransactions);
        }

        let mining_reward_address = self.get_wallet(mining_reward_address.to_string());

        if mining_reward_address.is_none() {
            return Err(BlockchainError::InvalidRewardAddress);
        }

        let latest_block = self.chain.last();
        let mut block = Block::new(self.chain.len(), &self.pending_transactions);

        match latest_block {
            Some(latest_block) => {
                block.set_previous_hash(&latest_block.hash);
            }
            None => {}
        };

        block.mine_block(self.difficulty);

        self.chain.push(block);
        self.pending_transactions = vec![Transaction::new(
            Wallet::new("".to_string(), 0, "".to_string()),
            mining_reward_address.unwrap(),
            self.mining_reward,
        )];

        Ok(())
    }

    pub fn create_transaction(
        &mut self,
        transaction: TransactionInfo,
    ) -> Result<String, TransactionError> {
        if transaction.from_address.is_empty() {
            return Err(TransactionError::EmptyFromAddress);
        }

        if transaction.to_address.is_empty() {
            return Err(TransactionError::EmptyToAddress);
        }

        if transaction.amount <= 0 {
            return Err(TransactionError::InvalidAmount);
        }

        let from_wallet = self.get_wallet(transaction.from_address);
        let to_wallet = self.get_wallet(transaction.to_address);

        if from_wallet.is_none() {
            return Err(TransactionError::InvalidFromAddress);
        }

        if to_wallet.is_none() {
            return Err(TransactionError::InvalidToAddress);
        }

        let mut from_wallet = from_wallet.unwrap();
        let mut to_wallet = to_wallet.unwrap();

        if from_wallet.balance < transaction.amount {
            return Err(TransactionError::NotEnoughMoney);
        }

        from_wallet.balance -= transaction.amount;
        to_wallet.balance += transaction.amount;

        let new_transaction = Transaction::new(from_wallet, to_wallet, transaction.amount);
        self.pending_transactions.push(new_transaction);

        Ok("Transaction successfully made".to_string())
    }

    pub fn create_wallet(&mut self, wallet: WalletInfo) -> Result<String, WalletError> {
        if wallet.address.is_empty() {
            return Err(WalletError::EmptyAddress);
        }

        if wallet.balance < 0 {
            return Err(WalletError::NegativeBallance);
        }

        if wallet.password.is_empty() {
            return Err(WalletError::EmptyPassword);
        }

        for blockchain_wallet in &self.wallets {
            if blockchain_wallet.address == wallet.address {
                return Err(WalletError::WalletAlreadyExists);
            }
        }

        let new_wallet = Wallet::new(wallet.address, wallet.balance as u32, wallet.password);

        self.wallets.push(new_wallet);

        Ok("Wallet created!".to_string())
    }

    pub fn get_balance_of_wallet(&self, address: String) -> Result<u32, BlockchainError> {
        let mut balance = 0;
        let mut balance_found = false;

        for blockchain_wallet in &self.wallets {
            if blockchain_wallet.address == address {
                balance = blockchain_wallet.balance;
                balance_found = true;
            }
        }

        match balance_found {
            true => Ok(balance),
            false => Err(BlockchainError::AddressNotFound),
        }
    }

    pub fn get_wallet(&self, address: String) -> Option<Wallet>{
        for wallet in self.wallets.clone() {
            if wallet.address == address {
                return Some(wallet);
            }
        }

        None
    }
}
