use anchor_lang::prelude::*;
use anchor_spl::{
    token::{transfer, Transfer},
    token_interface::{TokenAccount, TokenInterface},
};

use crate::error::ErrorCode;
use crate::state::{Market, UserPosition};

#[derive(Accounts)]
pub struct Deposit<'info> {
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

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);

        let market = &mut self.market;
        let user_position = &mut self.user_position;
        let current_time = Clock::get()?.unix_timestamp;

        // Log pre-deposit state
        msg!(
            "DEPOSIT DEBUG - Pre-deposit: market.total_deposits={}, market.total_deposit_shares={}",
            market.total_deposits,
            market.total_deposit_shares
        );

        // For Phase 1, simple fixed interest accrual
        // Calculate time elapsed since last accrual
        let time_elapsed = current_time.checked_sub(market.last_accrual_timestamp)
            .ok_or(ErrorCode::MathOverflow)?;
        
        if time_elapsed > 0 {
            // Apply simple interest to total borrows
            // Formula: interest = principal * rate * time
            // Where rate is per second (fixed_borrow_rate / 10000 / 365 / 86400)
            if market.total_borrows > 0 {
                // Convert basis points to decimal, then to per-second rate
                // 10000 basis points = 100%, 365 days, 86400 seconds per day
                let interest_factor = (market.fixed_borrow_rate as u128)
                    .checked_mul(time_elapsed as u128)
                    .ok_or(ErrorCode::MathOverflow)?
                    .checked_div(10000 * 365 * 86400)
                    .ok_or(ErrorCode::MathOverflow)?;
                
                let interest = (market.total_borrows as u128)
                    .checked_mul(interest_factor)
                    .ok_or(ErrorCode::MathOverflow)?;
                
                if interest > 0 {
                    market.total_borrows = market.total_borrows
                        .checked_add(interest as u64)
                        .ok_or(ErrorCode::MathOverflow)?;
                }
            }
            
            // Update last accrual timestamp
            market.last_accrual_timestamp = current_time;
        }

        // Log post-accrual state
        msg!(
            "DEPOSIT DEBUG - Post-accrual: market.total_deposits={}, market.total_deposit_shares={}",
            market.total_deposits,
            market.total_deposit_shares
        );

        // Transfer tokens before calculating shares
        let cpi_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.user_token_account.to_account_info(),
                to: self.deposit_vault.to_account_info(),
                authority: self.owner.to_account_info(),
            },
        );

        transfer(cpi_ctx, amount)?;

        // After transfer, update total deposits before calculating shares
        market.total_deposits = market
            .total_deposits
            .checked_add(amount)
            .ok_or(ErrorCode::MathOverflow)?;

        msg!(
            "DEPOSIT DEBUG - After deposit: total_deposits={}",
            market.total_deposits
        );

        // Calculate shares
        let shares = if market.total_deposit_shares == 0 {
            // First deposit, 1:1 ratio
            msg!("DEPOSIT DEBUG - Using 1:1 ratio for first deposit");
            amount
        } else {
            // Calculate shares based on proportion of new deposit to existing deposits
            amount
                .checked_mul(market.total_deposit_shares)
                .ok_or(ErrorCode::MathOverflow)?
                .checked_div(market.total_deposits.checked_sub(amount).ok_or(ErrorCode::MathOverflow)?)
                .ok_or(ErrorCode::MathOverflow)?
        };

        msg!("DEPOSIT DEBUG - Calculated shares: {}", shares);

        // Update user position
        user_position.deposited_jito = user_position
            .deposited_jito
            .checked_add(amount)
            .ok_or(ErrorCode::MathOverflow)?;

        user_position.deposited_shares = user_position
            .deposited_shares
            .checked_add(shares)
            .ok_or(ErrorCode::MathOverflow)?;

        // Update total deposit shares
        market.total_deposit_shares = market
            .total_deposit_shares
            .checked_add(shares)
            .ok_or(ErrorCode::MathOverflow)?;

        // Update timestamp
        user_position.last_update_timestamp = current_time;

        // Emit deposit event
        emit!(DepositEvent {
            user: self.owner.key(),
            amount,
            shares,
            timestamp: current_time,
        });

        Ok(())
    }
}

#[event]
pub struct DepositEvent {
    pub user: Pubkey,
    pub amount: u64,
    pub shares: u64,
    pub timestamp: i64,
}