#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;
pub mod instructions;
pub mod state;

declare_id!("5Hg6GpJcwhZQ5CjoaU3qbsPA9NvbK1GJUvKutU3poDDB");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>,seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;

        Ok(())
    }

    pub fn take(ctx: Context<Take>) ->  Result<()> {
        ctx.accounts.taker_deposit()?;

        ctx.accounts.taker_withdraw()?;

        Ok(())
    }
}


