use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient token balance in maker's account")]
    InsufficientMakerBalance,

    #[msg("Insufficient token balance in taker's account")]
    InsufficientTakerBalance,

    #[msg("Invalid token mint - must be different from offered token")]
    InvalidTokenMint,

    #[msg("Amount must be greater than zero")]
    InvalidAmount,
}
