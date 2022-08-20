use crate::blockchain::{block::Block, block_chain::BlockChain, transaction::Transaction};
use actix_web::{
    http::{header::ContentType, StatusCode},
    web::Json,
    HttpResponse, ResponseError,
};
use derive_more::Display;
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Debug, Display)]
pub enum BlockchainError {
    CoinAlreadyExists,
    CoinNotFound,
    NoPendingTransactions,
}

impl ResponseError for BlockchainError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            BlockchainError::CoinAlreadyExists => StatusCode::IM_USED,
            BlockchainError::CoinNotFound => StatusCode::NOT_FOUND,
            BlockchainError::NoPendingTransactions => StatusCode::NOT_FOUND,
        }
    }
}

#[derive(Default)]
pub struct Coin {
    coin: Mutex<BlockChain>,
}

thread_local! {
    static SINGLETON_POOL: Arc<Coin>  = Arc::new(Default::default());
}

impl Coin {
    pub fn get_instance() -> Arc<Coin> {
        SINGLETON_POOL.with(|singleton_pool| singleton_pool.clone())
    }

    pub fn create_transaction(transaction: Transaction) {
        let coin = Coin::get_instance();
        let mut coin = coin.coin.try_lock().unwrap();
        coin.create_transaction(transaction);
    }

    pub fn mine_pending_transactions(reward_address: String) {
        let coin = Coin::get_instance();
        let mut coin = coin.coin.try_lock().unwrap();
        coin.mine_pending_transactions(reward_address);
    }

    pub fn get_blockchain() -> Vec<Block> {
        let coin = Coin::get_instance();
        let mut coin = coin.coin.try_lock().unwrap();
        coin.chain.clone()
    }

    pub fn get_balance_of_address(address: &String) -> u32 {
        let coin = Coin::get_instance();
        let mut coin = coin.coin.try_lock().unwrap();
        coin.get_balance_of_address(address.to_string()).clone()
    }

    
}
