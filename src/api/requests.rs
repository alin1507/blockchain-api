use crate::blockchain::{
    block::Block,
    block_chain::{BlockchainError, BLOCKCHAIN},
    transaction::{TransactionInfo, TransactionError},
    wallet::{WalletError, WalletInfo, MineRewardAddress},
};
use actix_web::{get, post, web::Json, web::Path};
use serde::{Deserialize, Serialize};

#[post("/transaction/new")]
pub async fn new_transaction(
    transaction: Json<TransactionInfo>,
) -> Result<String, TransactionError> {
    match BLOCKCHAIN.lock().unwrap().create_transaction(transaction.0) {
        Ok(response) => Ok(response),
        Err(err) => Err(err),
    }
}

#[post("/transaction/mine")]
pub async fn mine_pending_transactions(
    reward_address: Json<MineRewardAddress>,
) -> Result<String, BlockchainError> {
    match BLOCKCHAIN
        .lock()
        .unwrap()
        .mine_pending_transactions(&reward_address.mining_reward_address)
    {
        Ok(()) => Ok("Transactions successfully mined".to_string()),
        Err(err) => Err(err),
    }
}

#[get("/blockchain/get")]
pub async fn show_blockchain() -> Result<Json<Vec<Block>>, BlockchainError> {
    let chain = BLOCKCHAIN.lock().unwrap().chain.clone();

    if chain.is_empty() {
        Err(BlockchainError::ChainIsEmpty)
    } else {
        Ok(Json(chain))
    }
}

#[derive(Deserialize, Serialize)]
pub struct AddressIdentifier {
    address: String,
}

#[get("/balance/{address}")]
pub async fn get_balance_of_address(
    address_identifier: Path<AddressIdentifier>,
) -> Result<String, BlockchainError> {
    let address = address_identifier.into_inner().address;
    let balance = BLOCKCHAIN
        .lock()
        .unwrap()
        .get_balance_of_wallet(address.to_string());

    match balance {
        Ok(balance) => Ok(format!("Balance for wallet {} is: {}", address, balance)),
        Err(err) => Err(err),
    }
}

#[post("/wallet/new")]
pub async fn create_wallet(wallet: Json<WalletInfo>) -> Result<String, WalletError> {
    let mut blockchain = BLOCKCHAIN.lock().unwrap();
    let wallet = blockchain.create_wallet(wallet.0);

    match wallet {
        Ok(msg) => Ok(msg),
        Err(err) => Err(err),
    }
}
