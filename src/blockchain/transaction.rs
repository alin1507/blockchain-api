use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use super::wallet::Wallet;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transaction {
    pub from_wallet: Wallet,
    pub to_wallet: Wallet,
    pub amount: u32,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct TransactionInfo {
    pub from_address: String,
    pub from_password: String,
    pub to_address: String,
    pub amount: u32,
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

impl TransactionInfo {
    /**
     * Check if the transaction is valid
     */
    pub fn check_transaction_info(&self) -> Result<(), TransactionError> {
        if self.from_address.is_empty() {
            return Err(TransactionError::EmptyFromAddress);
        }

        if self.to_address.is_empty() {
            return Err(TransactionError::EmptyToAddress);
        }

        if self.amount <= 0 {
            return Err(TransactionError::InvalidAmount);
        }

        Ok(())
    }
}

#[derive(Debug, Display)]
pub enum TransactionError {
    EmptyToAddress,
    EmptyFromAddress,
    InvalidAmount,
    NotEnoughMoney,
    InvalidFromAddress,
    InvalidToAddress,
    WrongFromPassword,
    InvalidWallet,
    NoPendingTransactions,
    InvalidRewardAddress,
    NegativeAmount
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
            TransactionError::WrongFromPassword => StatusCode::FAILED_DEPENDENCY,
            TransactionError::InvalidWallet => StatusCode::FAILED_DEPENDENCY,
            TransactionError::NoPendingTransactions => StatusCode::NOT_FOUND,
            TransactionError::InvalidRewardAddress => StatusCode::NOT_FOUND,
            TransactionError::NegativeAmount => StatusCode::FAILED_DEPENDENCY,
        }
    }
}