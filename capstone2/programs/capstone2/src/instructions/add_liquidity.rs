use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Transfer, transfer},
    token_interface::{TokenAccount, TokenInterface}
};

use crate::state::Market;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub provider: Signer<'info>,

    /// CHECK: This is just used for the PDA seed
    pub authority: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"market", authority.key().as_ref()],
        bump = market.bump
    )]
    pub market: Account<'info, Market>,

    #[account(
        mut,
        constraint = user_token_account.owner == provider.key(),
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

impl<'info> AddLiquidity<'info> {
    pub fn add_liquidity(&mut self, amount: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);

        let market = &mut self.market;
        let current_time = Clock::get()?.unix_timestamp;

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

        // Transfer tokens from user to vault
        let cpi_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.user_token_account.to_account_info(),
                to: self.borrow_vault.to_account_info(),
                authority: self.provider.to_account_info(),
            },
        );

        transfer(cpi_ctx, amount)?;

        // Emit event for liquidity addition
        emit!(AddLiquidityEvent {
            provider: self.provider.key(),
            amount,
            timestamp: current_time,
        });

        msg!(
            "Liquidity of {} tokens added by {}",
            amount,
            self.provider.key()
        );

        Ok(())
    }
}

#[event]
pub struct AddLiquidityEvent {
    pub provider: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}