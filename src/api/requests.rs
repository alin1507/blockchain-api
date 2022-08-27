use crate::blockchain::{
    block_chain::BLOCKCHAIN,
    block_chain_errors::BlockChainError,
    transaction::TransactionInfo,
    wallet::{MineRewardAddress, WalletCoins, WalletInfo},
};
use actix_web::{get, post, web::Json, web::Path, HttpResponse};
use serde::{Deserialize, Serialize};

//CREATE NEW TRANSACTION WITH 'FROM' ADDRESS, 'FROM' PASSWORD, 'TO' ADDRESS AND THE AMOUNT. ALL THE TRANSACTIONS THAT ARE CREATED ARE GOING TO PENDING TRANSACTIONS
#[post("/transaction/new")]
pub async fn create_transaction(
    transaction: Json<TransactionInfo>,
) -> Result<String, BlockChainError> {
    Ok(BLOCKCHAIN
        .lock()
        .unwrap()
        .create_transaction(transaction.0)?)
}

//ALL THE TRANSACTIONS FROM PENDING TRANSACTIONS ARE MOVED INTO THE BLOCK CHAIN AND THE MINER IS REWARDED WITH AN AMOUNT OF COINS
#[post("/transaction/mine")]
pub async fn mine_pending_transactions(
    reward_address: Json<MineRewardAddress>,
) -> Result<String, BlockChainError> {
    Ok(BLOCKCHAIN
        .lock()
        .unwrap()
        .mine_pending_transactions(&reward_address.mining_reward_address)?)
}

//CREATE A NEW WALLET WITH AN ADDRESS, A PASSWORD AND AN AMOUNT OF COINS
#[post("/wallet/new")]
pub async fn create_wallet(wallet: Json<WalletInfo>) -> Result<String, BlockChainError> {
    Ok(BLOCKCHAIN.lock().unwrap().create_wallet(wallet.0)?)
}

//ADD COINS TO AN EXISTING WALLET, THE ADDRESS AND THE PASSWORD ARE NEEDED
#[post("/wallet/addCoins")]
pub async fn add_coins(wallet: Json<WalletCoins>) -> Result<String, BlockChainError> {
    Ok(BLOCKCHAIN.lock().unwrap().add_coins(wallet.0)?)
}

//PARSE THE DATA AND SHOW THE BLOCKCHAIN
#[get("/blockchain/get")]
pub async fn show_blockchain() -> Result<HttpResponse, BlockChainError> {
    let chain = BLOCKCHAIN.lock().unwrap().chain.clone();
    let mut chain_string = String::new();

    for block in chain {
        let mut transactions_string = String::new();

        for transaction in block.transactions {
            transactions_string = format!(
                "{}\n{}",
                transactions_string,
                format!(
                    "   From: {}\n   To: {}\n   Amount: {}\n",
                    transaction.from_wallet.address,
                    transaction.to_wallet.address,
                    transaction.amount,
                )
            );
        }

        chain_string = format!(
            "{}{}",
            chain_string,
            format!(
                "Index: {}\nTimestamp: {}\nTransactions: \n{}\nHash: {}\nPrevious hash: {}\n\n",
                block.index, block.timestamp, transactions_string, block.hash, block.previous_hash
            )
        )
    }

    Ok(HttpResponse::Ok().body(chain_string))
}

//CONTAINS THE ADDRESS AND THE PASSWORD OF AN WALLET IN ORDER TO SEE IT BALANCE
#[derive(Deserialize, Serialize)]
pub struct AddressIdentifier {
    address: String,
    password: String,
}

//SHOW THE BALLANCE OF AN WALLET BASED ON THE ADDRESS AND PASSWORD
#[get("/wallet/balance/{address}/{password}")]
pub async fn get_wallet_balance(
    address_identifier: Path<AddressIdentifier>,
) -> Result<HttpResponse, BlockChainError> {
    let address_iden = address_identifier.into_inner();
    let (address, password) = (address_iden.address, address_iden.password);

    match BLOCKCHAIN
        .lock()
        .unwrap()
        .get_balance_of_wallet(&address, &password)
    {
        Ok(balance) => Ok(HttpResponse::Ok().body(format!("Your balance is: {}", balance))),
        Err(err) => Err(err),
    }
}

//SHOW THE TRANSACTIONS THAT HAD BEEN MADE IN AN WALLET BASED ON THE ADDRESS AND PASSWORD
#[get("wallet/transactions/{address}/{password}")]
pub async fn get_wallet_transactions(
    address_identifier: Path<AddressIdentifier>,
) -> Result<HttpResponse, BlockChainError> {
    let address_identifier = address_identifier.into_inner();
    let (address, password) = (address_identifier.address, address_identifier.password);

    let transactions = BLOCKCHAIN
        .lock()
        .unwrap()
        .get_transactions_of_wallet(&address, &password)?;

    if transactions.len() == 0 {
        return Ok(HttpResponse::NotFound().body("No transactions found for this wallet!"));
    }

    let mut transactions_string = String::new();

    for transaction in transactions {
        transactions_string = format!(
            "{}{}",
            transactions_string,
            format!(
                "From: {}\nTo: {}\nAmount: {}\n\n",
                transaction.from_address, transaction.to_address, transaction.amount,
            )
        );
    }

    Ok(HttpResponse::Ok().body(transactions_string))
}
