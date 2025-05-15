use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Operation exceeds maximum LTV")]
    ExceedsMaximumLtv,
    
    #[msg("Protocol is paused")]
    ProtocolPaused,
    
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,
    
    #[msg("Invalid amount")]
    InvalidAmount,
    
    #[msg("Math overflow")]
    MathOverflow,
    
    #[msg("Unauthorized")]
    Unauthorized,


}
