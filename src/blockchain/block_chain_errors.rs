use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

//POSSIBLE ERRORS
#[derive(Debug, Display)]
pub enum BlockChainError {
    #[display(fmt = "'From' address is empty!")]
    EmptyFromAddress,
    #[display(fmt = "'To' address is empty!")]
    EmptyToAddress,
    #[display(fmt = "Amount can't be 0 or less!")]
    InvalidAmount,
    #[display(fmt = "Address is empty!")]
    EmptyAddress,
    #[display(fmt = "Address cannot be 'MINING'!")]
    MiningAddress,
    #[display(fmt = "Balance is less than 0!")]
    NegativeBalance,
    #[display(fmt = "Password is empty!")]
    EmptyPassword,
    #[display(fmt = "There are no pending transactions!")]
    NoPendingTransactions,
    #[display(fmt = "Reward address doesn't exists!")]
    InvalidRewardAddress,
    #[display(fmt = "Wrong password!")]
    WrongPassword,
    #[display(fmt = "'To' address doesn't exists!")]
    InvalidToAddress,
    #[display(fmt = "'From' address doesn't exists!")]
    InvalidFromAddress,
    #[display(fmt = "Not enough coins!")]
    NotEnoughCoins,
    #[display(fmt = "Amount is less than 0!")]
    NegativeAmount,
    #[display(fmt = "A wallet with this address already exists!")]
    WalletAlreadyExists,
    #[display(fmt = "Wallet not found!")]
    WalletNotFound,
    #[display(fmt = "Chain is empty!")]
    ChainIsEmpty,
}

impl ResponseError for BlockChainError {
    //DISPLAY THE ERROR MESSAGE IN AN 'HttpResponse'
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    //MATCH EVERY ERROR THAT MIGHT APPEAR WITH AND STATUS CODE
    fn status_code(&self) -> StatusCode {
        match *self {
            BlockChainError::EmptyFromAddress => StatusCode::NOT_FOUND,
            BlockChainError::EmptyToAddress => StatusCode::NOT_FOUND,
            BlockChainError::InvalidAmount => StatusCode::FAILED_DEPENDENCY,
            BlockChainError::EmptyAddress => StatusCode::NOT_FOUND,
            BlockChainError::MiningAddress => StatusCode::FAILED_DEPENDENCY,
            BlockChainError::NegativeBalance => StatusCode::FAILED_DEPENDENCY,
            BlockChainError::EmptyPassword => StatusCode::FAILED_DEPENDENCY,
            BlockChainError::NoPendingTransactions => StatusCode::FAILED_DEPENDENCY,
            BlockChainError::InvalidRewardAddress => StatusCode::FAILED_DEPENDENCY,
            BlockChainError::WrongPassword => StatusCode::FAILED_DEPENDENCY,
            BlockChainError::InvalidToAddress => StatusCode::FAILED_DEPENDENCY,
            BlockChainError::InvalidFromAddress => StatusCode::FAILED_DEPENDENCY,
            BlockChainError::NotEnoughCoins => StatusCode::FAILED_DEPENDENCY,
            BlockChainError::NegativeAmount => StatusCode::FAILED_DEPENDENCY,
            BlockChainError::WalletAlreadyExists => StatusCode::FAILED_DEPENDENCY,
            BlockChainError::WalletNotFound => StatusCode::NOT_FOUND,
            BlockChainError::ChainIsEmpty => StatusCode::NOT_FOUND,
        }
    }
}
