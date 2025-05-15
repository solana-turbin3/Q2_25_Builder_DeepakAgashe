use anchor_lang::prelude::*;
use anchor_spl::{
    token::{transfer, Transfer},
    token_interface::{TokenAccount, TokenInterface},
};

use crate::error::ErrorCode;
use crate::state::{Market, UserPosition};

#[derive(Accounts)]
pub struct Borrow<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"market", owner.key().as_ref()],
        bump = market.bump
    )]
    pub market: Account<'info, Market>,

    #[account(
        mut,
        seeds = [b"user_position", owner.key().as_ref()],
        bump = user_position.bump,
        constraint = user_position.owner == owner.key(),
    )]
    pub user_position: Account<'info, UserPosition>,

    #[account(
        mut,
        constraint = user_token_account.owner == owner.key(),
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        constraint = borrow_vault.key() == market.borrow_vault
    )]
    pub borrow_vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Borrow<'info> {
    pub fn borrow(&mut self, amount: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);

        let market = &mut self.market;
        let user_position = &mut self.user_position;
        let current_time = Clock::get()?.unix_timestamp;

        let time_elapsed = current_time
            .checked_sub(market.last_accrual_timestamp)
            .ok_or(ErrorCode::MathOverflow)?;

        if time_elapsed > 0 {
            if market.total_borrows > 0 {
                let interest_factor = (market.fixed_borrow_rate as u128)
                    .checked_mul(time_elapsed as u128)
                    .ok_or(ErrorCode::MathOverflow)?
                    .checked_div(10000 * 365 * 86400)
                    .ok_or(ErrorCode::MathOverflow)?;

                let interest = (market.total_borrows as u128)
                    .checked_mul(interest_factor)
                    .ok_or(ErrorCode::MathOverflow)?;

                if interest > 0 {
                    market.total_borrows = market
                        .total_borrows
                        .checked_add(interest as u64)
                        .ok_or(ErrorCode::MathOverflow)?;
                }
            }

            market.last_accrual_timestamp = current_time;
        }

        require!(
            self.borrow_vault.amount >= amount,
            ErrorCode::InsufficientLiquidity
        );

        let collateral_value = user_position.deposited_jito;
        let borrow_value = user_position
            .borrowed_sol
            .checked_add(amount)
            .ok_or(ErrorCode::MathOverflow)?;

        require!(
            borrow_value
                .checked_mul(10000)
                .ok_or(ErrorCode::MathOverflow)?
                <= collateral_value
                    .checked_mul(market.max_ltv)
                    .ok_or(ErrorCode::MathOverflow)?,
            ErrorCode::ExceedsMaximumLtv
        );

        let new_total_borrows = market
            .total_borrows
            .checked_add(amount)
            .ok_or(ErrorCode::MathOverflow)?;

        let shares = if market.total_borrow_shares == 0 {
            amount
        } else {
            amount
                .checked_mul(market.total_borrow_shares)
                .ok_or(ErrorCode::MathOverflow)?
                .checked_div(market.total_borrows)
                .ok_or(ErrorCode::MathOverflow)?
        };

        user_position.borrowed_sol = user_position
            .borrowed_sol
            .checked_add(amount)
            .ok_or(ErrorCode::MathOverflow)?;

        user_position.borrowed_shares = user_position
            .borrowed_shares
            .checked_add(shares)
            .ok_or(ErrorCode::MathOverflow)?;

        let market_key = self.owner.key();
        let seeds = &[b"market", market_key.as_ref(), &[market.bump]];
        let signer = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            Transfer {
                from: self.borrow_vault.to_account_info(),
                to: self.user_token_account.to_account_info(),
                authority: market.to_account_info(),
            },
            signer,
        );

        transfer(cpi_ctx, amount)?;

        market.total_borrows = new_total_borrows;
        market.total_borrow_shares = market
            .total_borrow_shares
            .checked_add(shares)
            .ok_or(ErrorCode::MathOverflow)?;

        user_position.last_update_timestamp = current_time;

        emit!(BorrowEvent {
            user: self.owner.key(),
            amount,
            shares,
            timestamp: current_time
        });

        Ok(())
    }
}

#[event]
pub struct BorrowEvent {
    pub user: Pubkey,
    pub amount: u64,
    pub shares: u64,
    pub timestamp: i64,
}
