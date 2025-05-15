#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;


pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use instructions::*;
pub use state::*;



declare_id!("9Wvj6zZ7f4LegpRaWRFXjFFBU8tBT6hwD5S47D366Ww7");

#[program]
pub mod capstone {
    use super::*;

    pub fn initialize(ctx: Context<InitializeMarket>) -> Result<()> {
        ctx.accounts.initialize_market(&ctx.bumps)?;
        Ok(())
    }
    
    pub fn user_position(ctx: Context<Position>) -> Result<()>{
        ctx.accounts.create_position(&ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()>{
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn borrow(ctx: Context<Borrow>, amount: u64) -> Result<()> {
        ctx.accounts.borrow(amount)?;
        Ok(())
    }

    pub fn repay(ctx: Context<Repay>, amount: u64) -> Result<()> {
        ctx.accounts.repay(amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>, amount: u64) -> Result<()> {
        ctx.accounts.add_liquidity(amount)?;
        Ok(())
    }
}