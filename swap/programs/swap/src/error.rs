use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid swap amount")]
    InvalidSwapAmount,
    
    #[msg("Insufficient funds for swap")]
    InsufficientFunds,

    #[msg("Swap not allowed at this time")]
    SwapNotAllowed,

    #[msg("Invalid token pair for swap")]
    InvalidTokenPair,

    #[msg("Unauthorized access to swap operation")]
    UnauthorizedAccess,

    #[msg("Swap operation failed due to network error")]
    NetworkError,

    #[msg("Unexpected error occurred during swap operation")]
    UnexpectedError,
}
