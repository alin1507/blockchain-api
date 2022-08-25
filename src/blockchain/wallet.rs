use serde::{Deserialize, Serialize};
use super::{transaction::{TransactionInfo}, block_chain::MINING_ADDRESS, block_chain_errors::BlockChainError};

//CONTAINS THE INFORMATION ABOUT A WALLET
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Wallet {
    pub address: String,
    pub balance: u32,
    pub password: String,
    pub transactions: Vec<TransactionInfo>,
}

//CONTAINS THE INFORMATION THAT ARE REQUIRED WHEN CREATING A WALLET
#[derive(Deserialize, Serialize)]
pub struct WalletInfo {
    pub address: String,
    pub balance: i32,
    pub password: String,
}

//CONTAINS THE INFORMATION THAT ARE REQUIRED WHEN ADDING COINS TO A WALLET
#[derive(Deserialize, Serialize)]
pub struct WalletCoins {
    pub address: String,
    pub password: String,
    pub coins: u32,
}

//THE ADDRESS WHERE THE MINING REWARD WILL GO
#[derive(Deserialize, Serialize)]
pub struct MineRewardAddress {
    pub mining_reward_address: String,
}

//TYPES OF ADDRESSES
pub enum AddressType {
    TO,
    FROM,
    REWARD,
    GENERIC
}

impl Wallet {
    //CREATE A NEW WALLET
    pub fn new(address: String, balance: u32, password: String) -> Self {
        Wallet {
            address,
            balance,
            password,
            transactions: vec![],
        }
    }
}

impl WalletInfo {
    //CHECK IF THE WALLET INFORMATION ARE VALID
    pub fn check_wallet_info(&self) -> Result<(), BlockChainError> {
        if self.address.is_empty() {
            return Err(BlockChainError::EmptyAddress);
        }

        if self.address == MINING_ADDRESS.to_string() {
            return Err(BlockChainError::MiningAddress);
        }

        if self.balance < 0 {
            return Err(BlockChainError::NegativeBalance);
        }

        if self.password.is_empty() {
            return Err(BlockChainError::EmptyPassword);
        }

        Ok(())
    }
}