use serde::{Deserialize, Serialize};
use super::{transaction::{TransactionInfo}, block_chain::MINING_ADDRESS, block_chain_errors::BlockChainError};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Wallet {
    pub address: String,
    pub balance: u32,
    pub password: String,
    pub transactions: Vec<TransactionInfo>,
}

#[derive(Deserialize, Serialize)]
pub struct WalletInfo {
    pub address: String,
    pub balance: i32,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct WalletCoins {
    pub address: String,
    pub password: String,
    pub coins: u32,
}

#[derive(Deserialize, Serialize)]
pub struct MineRewardAddress {
    pub mining_reward_address: String,
}

impl Wallet {
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
    /**
     * Check if the wallet is valid
     */
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