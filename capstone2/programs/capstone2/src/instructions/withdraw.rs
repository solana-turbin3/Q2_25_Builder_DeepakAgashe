use anchor_lang::prelude::*;
use anchor_spl::{
    token::{transfer, Transfer},
    token_interface::{TokenAccount, TokenInterface},
};

use crate::error::ErrorCode;
use crate::state::{Market, UserPosition};

#[derive(Accounts)]
pub struct Withdraw<'info> {
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
        constraint = deposit_vault.key() == market.deposit_vault,
    )]
    pub deposit_vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
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

        let user_deposit_amount = if market.total_deposit_shares == 0 {
            0
        } else {
            user_position
                .deposited_shares
                .checked_mul(market.total_deposits)
                .ok_or(ErrorCode::MathOverflow)?
                .checked_div(market.total_deposit_shares)
                .ok_or(ErrorCode::MathOverflow)?
        };

        require!(user_deposit_amount >= amount, ErrorCode::InvalidAmount);

        let shares_to_burn = if user_deposit_amount == 0 {
            0
        } else {
            amount
                .checked_mul(user_position.deposited_shares)
                .ok_or(ErrorCode::MathOverflow)?
                .checked_div(user_deposit_amount)
                .ok_or(ErrorCode::MathOverflow)?
        };

        user_position.deposited_jito = user_position
            .deposited_jito
            .checked_sub(amount)
            .ok_or(ErrorCode::MathOverflow)?;

        user_position.deposited_shares = user_position
            .deposited_shares
            .checked_sub(shares_to_burn)
            .ok_or(ErrorCode::MathOverflow)?;

        if user_position.borrowed_sol > 0 {
            let remaining_collateral = user_position.deposited_jito;
            let borrowed_amount = user_position.borrowed_sol;

            require!(
                borrowed_amount
                    .checked_mul(10000)
                    .ok_or(ErrorCode::MathOverflow)?
                    <= remaining_collateral
                        .checked_mul(market.max_ltv)
                        .ok_or(ErrorCode::MathOverflow)?,
                ErrorCode::ExceedsMaximumLtv
            );
        }

        let market_key = self.owner.key();
        let seeds = &[b"market", market_key.as_ref(), &[market.bump]];
        let signer = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            Transfer {
                from: self.deposit_vault.to_account_info(),
                to: self.user_token_account.to_account_info(),
                authority: market.to_account_info(),
            },
            signer,
        );
        transfer(cpi_ctx, amount)?;

        market.total_deposits = market
            .total_deposits
            .checked_sub(amount)
            .ok_or(ErrorCode::MathOverflow)?;

        market.total_deposit_shares = market
            .total_deposit_shares
            .checked_sub(shares_to_burn)
            .ok_or(ErrorCode::MathOverflow)?;

        user_position.last_update_timestamp = current_time;

        emit!(WithdrawEvent {
            user: self.owner.key(),
            amount,
            shares: shares_to_burn,
            timestamp: current_time,
        });

        msg!(
            "Withdrew {} jitoSOL by burning {} shares",
            amount,
            shares_to_burn
        );
        Ok(())
    }
}

#[event]
pub struct WithdrawEvent {
    pub user: Pubkey,
    pub amount: u64,
    pub shares: u64,
    pub timestamp: i64,
}
