use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::Display;
use serde::{Deserialize, Serialize};

use super::{transaction::Transaction};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Wallet {
    pub address: String,
    pub balance: u32,
    pub password: String,
    transactions: Vec<Transaction>,
}

#[derive(Deserialize, Serialize)]
pub struct WalletInfo {
    pub address: String,
    pub balance: i32,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct MineRewardAddress {
    pub mining_reward_address: String
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

    pub fn check_balance(&self) -> &u32 {
        &self.balance
    }

    pub fn see_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    pub fn change_password(&mut self, old_password: String, new_password: String) {
        if self.password == old_password {
            self.password = new_password
        }
    }
}

#[derive(Debug, Display)]
pub enum WalletError {
    NegativeBallance,
    EmptyAddress,
    EmptyPassword,
    WalletAlreadyExists,
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
        }
    }
}
