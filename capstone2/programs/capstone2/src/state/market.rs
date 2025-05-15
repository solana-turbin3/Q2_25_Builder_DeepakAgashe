use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Market {
    pub authority: Pubkey,
    pub deposit_vault: Pubkey,      // Vault for jitoSOL deposits
    pub borrow_vault: Pubkey,       // Vault for SOL that can be borrowed
    pub total_deposits: u64,        // Total jitoSOL deposited
    pub total_borrows: u64,         // Total SOL borrowed
    pub total_deposit_shares: u64,  // Total shares for depositors
    pub total_borrow_shares: u64,   // Total shares for borrowers
    pub fixed_borrow_rate: u64,     // Fixed interest rate (in basis points)
    pub max_ltv: u64,               // Maximum loan-to-value ratio (in basis points)
    pub last_accrual_timestamp: i64, // Last time interest was accrued
    pub bump: u8,
}