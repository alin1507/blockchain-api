use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use super::{transaction::{TransactionInfo}, block_chain::MINING_ADDRESS};

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
    pub fn check_wallet_info(&self) -> Result<(), WalletError> {
        if self.address.is_empty() {
            return Err(WalletError::EmptyAddress);
        }

        if self.address == MINING_ADDRESS.to_string() {
            return Err(WalletError::AddressNotValid);
        }

        if self.balance < 0 {
            return Err(WalletError::NegativeBallance);
        }

        if self.password.is_empty() {
            return Err(WalletError::EmptyPassword);
        }

        Ok(())
    }
}

#[derive(Debug, Display)]
pub enum WalletError {
    NegativeBallance,
    EmptyAddress,
    EmptyPassword,
    WalletAlreadyExists,
    WalletNotFound,
    WrongPassword,
    AddressNotValid
}

impl ResponseError for WalletError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            WalletError::NegativeBallance => StatusCode::FAILED_DEPENDENCY,
            WalletError::EmptyAddress => StatusCode::FAILED_DEPENDENCY,
            WalletError::EmptyPassword => StatusCode::FAILED_DEPENDENCY,
            WalletError::WalletAlreadyExists => StatusCode::FAILED_DEPENDENCY,
            WalletError::WalletNotFound => StatusCode::NOT_FOUND,
            WalletError::WrongPassword => StatusCode::FAILED_DEPENDENCY,
            WalletError::AddressNotValid => StatusCode::FAILED_DEPENDENCY,
        }
    }
}
