use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserPosition {
    pub owner: Pubkey,              // Owner of this position
    pub deposited_jito: u64,        // Amount of jitoSOL deposited
    pub deposited_shares: u64,      // Shares representing deposit
    pub borrowed_sol: u64,          // Amount of SOL borrowed
    pub borrowed_shares: u64,       // Shares representing borrow
    pub last_update_timestamp: i64, // Last time this position was updated
    pub bump: u8
}