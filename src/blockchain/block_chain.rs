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
use std::sync::Mutex;
use std::vec;

lazy_static! {
    pub static ref BLOCKCHAIN: Mutex<BlockChain> = Mutex::new(BlockChain::default());
}

pub const MINING_ADDRESS: &str = "MINING";

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
    ChainIsEmpty,
}

impl ResponseError for BlockchainError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            BlockchainError::ChainIsEmpty => StatusCode::NOT_FOUND,
        }
    }
}

impl BlockChain {
    pub fn mine_pending_transactions(
        &mut self,
        mining_reward_address: &String,
    ) -> Result<String, TransactionError> {
        if self.pending_transactions.is_empty() {
            return Err(TransactionError::NoPendingTransactions);
        }

        for transaction in self.pending_transactions.clone() {
            let mut to_wallet = transaction.to_wallet;
            let mut from_wallet = transaction.from_wallet;

            to_wallet.balance += transaction.amount;
            to_wallet.transactions.push(TransactionInfo {
                from_address: from_wallet.address.clone(),
                from_password: from_wallet.password.clone(),
                to_address: to_wallet.address.clone(),
                amount: transaction.amount,
            });
            self.update_wallet(to_wallet.clone())?;

            if from_wallet.address != MINING_ADDRESS.to_string() {
                from_wallet.balance -= transaction.amount;
                from_wallet.transactions.push(TransactionInfo {
                    from_address: from_wallet.address.clone(),
                    from_password: from_wallet.password.clone(),
                    to_address: to_wallet.address.clone(),
                    amount: transaction.amount,
                });
                self.update_wallet(from_wallet.clone())?;
            }
        }

        let mining_reward_wallet = match self.get_wallet(mining_reward_address) {
            Some(wallet) => wallet,
            None => return Err(TransactionError::InvalidRewardAddress),
        };

        let mut block = Block::new(self.chain.len(), &self.pending_transactions);

        match self.chain.last() {
            Some(latest_block) => {
                block.set_previous_hash(&latest_block.hash);
            }
            None => {}
        };

        block.mine_block(self.difficulty);

        self.chain.push(block);
        self.pending_transactions = vec![Transaction::new(
            Wallet::new(MINING_ADDRESS.to_string(), 0, "".to_string()),
            mining_reward_wallet,
            self.mining_reward,
        )];

        Ok("Transactions successfully mined".to_string())
    }

    pub fn create_transaction(
        &mut self,
        transaction: TransactionInfo,
    ) -> Result<String, TransactionError> {
        match transaction.check_transaction_info() {
            Ok(_) => (),
            Err(err) => return Err(err),
        }

        let from_wallet = match self.get_wallet(&transaction.from_address) {
            Some(wallet) => wallet,
            None => return Err(TransactionError::InvalidFromAddress),
        };

        if from_wallet.password != transaction.from_password {
            return Err(TransactionError::WrongFromPassword);
        }

        let to_wallet = match self.get_wallet(&transaction.to_address) {
            Some(wallet) => wallet,
            None => return Err(TransactionError::InvalidToAddress),
        };

        if from_wallet.balance < transaction.amount {
            return Err(TransactionError::NotEnoughMoney);
        };

        let new_transaction = Transaction::new(from_wallet, to_wallet, transaction.amount);
        self.pending_transactions.push(new_transaction);

        Ok("Transaction successfully made".to_string())
    }

    pub fn create_wallet(&mut self, wallet: WalletInfo) -> Result<String, WalletError> {
        match wallet.check_wallet_info() {
            Ok(_) => (),
            Err(err) => return Err(err),
        }

        match self.get_wallet(&wallet.address) {
            Some(_) => return Err(WalletError::WalletAlreadyExists),
            None => (),
        }

        let new_wallet = Wallet::new(wallet.address, wallet.balance as u32, wallet.password);
        self.wallets.push(new_wallet);

        Ok("Wallet created!".to_string())
    }

    pub fn get_balance_of_wallet(
        &self,
        address: &String,
        password: &String,
    ) -> Result<u32, WalletError> {
        let wallet = match self.get_wallet(address) {
            Some(wallet) => wallet,
            None => return Err(WalletError::WalletNotFound),
        };

        if wallet.password != password.to_string() {
            return Err(WalletError::WrongPassword);
        }

        Ok(wallet.balance)
    }

    pub fn get_transactions_of_wallet(
        &self,
        address: &String,
        password: &String,
    ) -> Result<Vec<TransactionInfo>, WalletError> {
        let wallet = match self.get_wallet(address) {
            Some(wallet) => wallet,
            None => return Err(WalletError::WalletNotFound),
        };

        if wallet.password != password.to_string() {
            return Err(WalletError::WrongPassword);
        }

        Ok(wallet.transactions)
    }

    pub fn get_wallet(&self, address: &String) -> Option<Wallet> {
        for wallet in self.wallets.clone() {
            if wallet.address == address.to_string() {
                return Some(wallet);
            }
        }
        None
    }

    pub fn update_wallet(&mut self, wallet: Wallet) -> Result<(), TransactionError> {
        match self
            .wallets
            .iter()
            .position(|w| *w.address == wallet.address)
        {
            Some(index) => self.wallets[index] = wallet,
            None => return Err(TransactionError::InvalidWallet),
        }

        Ok(())
    }
}
