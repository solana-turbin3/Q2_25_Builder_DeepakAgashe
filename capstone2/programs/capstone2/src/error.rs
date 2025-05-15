use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Operation exceeds maximum LTV")]
    ExceedsMaximumLtv,
    
    #[msg("Protocol is paused")]
    ProtocolPaused,
    
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,
    
    #[msg("Position is being unstaked")]
    PositionBeingUnstaked,
    
    #[msg("Position is not being unstaked")]
    NotBeingUnstaked,
    
    #[msg("Unstaking period not complete")]
    UnstakingNotComplete,
    
    #[msg("Invalid amount")]
    InvalidAmount,
    
    #[msg("Math overflow")]
    MathOverflow,
    
    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("Invalid Oracle Data!")]
    InvalidOracleData
}
