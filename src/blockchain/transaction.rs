use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::fmt;

use super::wallet::Wallet;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transaction {
    pub from_wallet: Wallet,
    pub to_wallet: Wallet,
    pub amount: u32,
}

#[derive(Deserialize, Serialize)]
pub struct TransactionInfo {
    pub from_address: String,
    pub to_address: String,
    pub amount: u32,
}

#[derive(Debug, Display)]
pub enum TransactionError {
    EmptyToAddress,
    EmptyFromAddress,
    InvalidAmount,
    NotEnoughMoney,
    InvalidFromAddress,
    InvalidToAddress,
}

impl ResponseError for TransactionError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            TransactionError::InvalidFromAddress => StatusCode::FAILED_DEPENDENCY,
            TransactionError::InvalidToAddress => StatusCode::FAILED_DEPENDENCY,
            TransactionError::EmptyToAddress => StatusCode::NOT_FOUND,
            TransactionError::EmptyFromAddress => StatusCode::NOT_FOUND,
            TransactionError::InvalidAmount => StatusCode::FAILED_DEPENDENCY,
            TransactionError::NotEnoughMoney => StatusCode::FAILED_DEPENDENCY,
        }
    }
}

impl Transaction {
    pub fn new(from_wallet: Wallet, to_wallet: Wallet, amount: u32) -> Self {
        Transaction {
            amount,
            from_wallet,
            to_wallet,
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, {:?}, {:?}",
            self.amount, self.from_wallet, self.to_wallet
        )
    }
}
