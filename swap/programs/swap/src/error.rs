use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Transaction has already edited once")]
    TxAlreadyEditedOnce,
    #[msg("Invalid Owner")]
    InvalidOwner,
    #[msg("Invalid swap amount")]
    InvalidSwapAmount
}
