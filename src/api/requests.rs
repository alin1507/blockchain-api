use crate::blockchain::{
    block::{BlockInfo, BlockTransaction},
    block_chain::{BLOCKCHAIN},
    transaction::{TransactionInfo},
    wallet::{MineRewardAddress, WalletInfo, WalletCoins}, block_chain_errors::BlockChainError,
};
use actix_web::{get, post, web::Json, web::Path};
use serde::{Deserialize, Serialize};

#[post("/transaction/new")]
pub async fn create_transaction(
    transaction: Json<TransactionInfo>,
) -> Result<String, BlockChainError> {
    Ok(BLOCKCHAIN
        .lock()
        .unwrap()
        .create_transaction(transaction.0)?)
}

#[post("/transaction/mine")]
pub async fn mine_pending_transactions(
    reward_address: Json<MineRewardAddress>,
) -> Result<String, BlockChainError> {
    Ok(BLOCKCHAIN
        .lock()
        .unwrap()
        .mine_pending_transactions(&reward_address.mining_reward_address)?)
}

#[post("/wallet/new")]
pub async fn create_wallet(wallet: Json<WalletInfo>) -> Result<String, BlockChainError> {
    Ok(BLOCKCHAIN.lock().unwrap().create_wallet(wallet.0)?)
}

#[post("/wallet/addCoins")]
pub async fn add_coins(wallet: Json<WalletCoins>) -> Result<String, BlockChainError> {
    Ok(BLOCKCHAIN.lock().unwrap().add_coins(wallet.0)?)
}

#[get("/blockchain/get")]
pub async fn show_blockchain() -> Result<Json<Vec<BlockInfo>>, BlockChainError> {
    let chain = BLOCKCHAIN.lock().unwrap().chain.clone();
    let mut show_chain = vec![];

    for block in chain {
        let mut show_transactions = vec![];

        for transaction in block.transactions {
            show_transactions.push(BlockTransaction {
                from: transaction.from_wallet.address,
                to: transaction.to_wallet.address,
                amount: transaction.amount,
            });
        }

        let block_info = BlockInfo {
            index: block.index,
            timestamp: block.timestamp,
            transactions: show_transactions,
            hash: block.hash,
            previous_hash: block.previous_hash,
        };

        show_chain.push(block_info);
    }

    if show_chain.is_empty() {
        Err(BlockChainError::ChainIsEmpty)
    } else {
        Ok(Json(show_chain))
    }
}

#[derive(Deserialize, Serialize)]
pub struct AddressIdentifier {
    address: String,
    password: String,
}

#[get("/wallet/balance/{address}/{password}")]
pub async fn get_wallet_balance(
    address_identifier: Path<AddressIdentifier>,
) -> Result<String, BlockChainError> {
    let address_iden = address_identifier.into_inner();
    let (address, password) = (address_iden.address, address_iden.password);

    match BLOCKCHAIN
        .lock()
        .unwrap()
        .get_balance_of_wallet(&address, &password)
    {
        Ok(balance) => Ok(format!("Balance for wallet {} is: {}", address, balance)),
        Err(err) => Err(err),
    }
}

#[get("wallet/transactions/{address}/{password}")]
pub async fn get_wallet_transactions(
    address_identifier: Path<AddressIdentifier>,
) -> Result<Json<Vec<BlockTransaction>>, BlockChainError> {
    let address_iden = address_identifier.into_inner();
    let (address, password) = (address_iden.address, address_iden.password);

    let transactions = BLOCKCHAIN
        .lock()
        .unwrap()
        .get_transactions_of_wallet(&address, &password)?;

    let block_transactions: Vec<BlockTransaction> = transactions
        .iter()
        .map(|transaction_info| BlockTransaction {
            from: transaction_info.from_address.to_string(),
            to: transaction_info.to_address.to_string(),
            amount: transaction_info.amount,
        })
        .collect();

    Ok(Json(block_transactions))
}
