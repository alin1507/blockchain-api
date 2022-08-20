use super::coin::BlockchainError;
use crate::{
    api::coin::Coin,
    blockchain::{block::Block, transaction::Transaction},
};
use actix_web::{post, web::Json, web::Path, get};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CoinInfo {
    name: String,
    difficulty: usize,
    mining_reward: u32,
}

#[derive(Deserialize, Serialize)]
pub struct CoinIdentifier {
    coin_name: String,
}

#[post("/transaction/new")]
pub async fn new_transaction(transaction: Json<Transaction>) -> Result<String, BlockchainError> {
    Coin::create_transaction(Transaction::new(
        transaction.from_address.clone(),
        transaction.to_address.clone(),
        transaction.amount,
    ));

    Ok("Transaction successfully made".to_string())
}

#[derive(Deserialize, Serialize)]
pub struct RewardAddress {
    mining_reward_address: String,
}

#[post("/transaction/mine")]
pub async fn mine_pending_transactions(
    reward_address: Json<RewardAddress>,
) -> Result<String, BlockchainError> {
    Coin::mine_pending_transactions(reward_address.mining_reward_address.clone());

    Ok("Transactions successfully mined".to_string())
}

#[get("/blockchain/get")]
pub async fn show_blockchain() -> Result<Json<Vec<Block>>, BlockchainError> {
    let blockchain = Coin::get_blockchain();

    Ok(Json(blockchain))
}

#[derive(Deserialize, Serialize)]
pub struct AddressIdentifier {
    address: String,
}

#[get("/balance/{address}")]
pub async fn get_balance_of_address(address_identifier: Path<AddressIdentifier>) -> Result<String, BlockchainError> {
    let address = address_identifier.into_inner().address;
    let balance = Coin::get_balance_of_address(&address);

    Ok(format!("Balance for wallet {} is: {}", address, balance))
}
