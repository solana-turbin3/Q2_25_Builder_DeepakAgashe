use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserPosition {
    pub owner: Pubkey,              
    pub deposited_jito: u64,       
    pub deposited_shares: u64,      
    pub borrowed_sol: u64,          
    pub borrowed_shares: u64,       
    pub last_update_timestamp: i64, 
    pub bump: u8
}