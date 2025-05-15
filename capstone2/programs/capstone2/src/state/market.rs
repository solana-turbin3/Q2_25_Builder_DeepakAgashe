use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Market {
    pub authority: Pubkey,
    pub deposit_vault: Pubkey,     
    pub borrow_vault: Pubkey,       
    pub total_deposits: u64,        
    pub total_borrows: u64,        
    pub total_deposit_shares: u64,  
    pub total_borrow_shares: u64,  
    pub fixed_borrow_rate: u64,     
    pub max_ltv: u64,              
    pub last_accrual_timestamp: i64, 
    pub bump: u8,
}