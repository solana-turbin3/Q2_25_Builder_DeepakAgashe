use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::state::Market;

#[derive(Accounts)]
pub struct InitializeMarket<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + Market::INIT_SPACE,
        seeds = [b"market", authority.key().as_ref()],
        bump
    )]
    pub market: Account<'info, Market>,

    pub deposit_mint: InterfaceAccount<'info, Mint>,

    pub borrow_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = authority,
        token::mint = deposit_mint,
        token::authority = market
    )]
    pub deposit_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        token::mint = borrow_mint,
        token::authority = market
    )]
    pub borrow_vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> InitializeMarket<'info> {
    pub fn initialize_market(&mut self, bumps: &InitializeMarketBumps) -> Result<()> {
        let market = &mut self.market;
        let current_time = Clock::get()?.unix_timestamp;

        market.authority = self.authority.key();
        market.deposit_vault = self.deposit_vault.key();
        market.borrow_vault = self.borrow_vault.key();
        market.total_deposits = 0;
        market.total_borrows = 0;
        market.total_deposit_shares = 0;
        market.total_borrow_shares = 0;
        market.fixed_borrow_rate = 500;
        market.max_ltv = 7000;
        market.last_accrual_timestamp = current_time;
        market.bump = bumps.market;

        Ok(())
    }
}
