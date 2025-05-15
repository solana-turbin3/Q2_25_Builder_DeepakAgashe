use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Transfer, transfer},
    token_interface::{TokenInterface, TokenAccount}
}; 
use crate::state::{Market, UserPosition};
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct Repay<'info>{
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
        constraint = borrow_vault.key() == market.borrow_vault, 
    )]
    pub borrow_vault: InterfaceAccount<'info, TokenAccount>, 

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System> 
}

impl<'info> Repay<'info>{
    pub fn repay(&mut self, amount: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);

        let market = &mut self.market;
        let user_position = &mut self.user_position;
        let current_time = Clock::get()?.unix_timestamp;

       
        let time_elapsed = current_time.checked_sub(market.last_accrual_timestamp)
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
                    market.total_borrows = market.total_borrows
                        .checked_add(interest as u64)
                        .ok_or(ErrorCode::MathOverflow)?;
                }
            }
            
            
            market.last_accrual_timestamp = current_time;
        }

        let user_borrow_amount = if market.total_borrow_shares == 0 {
            0
        } else {
            user_position.borrowed_shares
                .checked_mul(market.total_borrows)
                .ok_or(ErrorCode::MathOverflow)?
                .checked_div(market.total_borrow_shares)
                .ok_or(ErrorCode::MathOverflow)?
        };

        
        let repay_amount = amount.min(user_borrow_amount);

        
        let shares_to_burn = if user_borrow_amount == 0 {
            0
        } else {
            repay_amount
                .checked_mul(user_position.borrowed_shares)
                .ok_or(ErrorCode::MathOverflow)?
                .checked_div(user_borrow_amount)
                .ok_or(ErrorCode::MathOverflow)?
        };

        
        let cpi_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            Transfer{
                from: self.user_token_account.to_account_info(),
                to: self.borrow_vault.to_account_info(),
                authority: self.owner.to_account_info()
            }
        );

        transfer(cpi_ctx, repay_amount)?;

       
        user_position.borrowed_sol = user_position.borrowed_sol
            .checked_sub(repay_amount)
            .ok_or(ErrorCode::MathOverflow)?;
            
        user_position.borrowed_shares = user_position.borrowed_shares
            .checked_sub(shares_to_burn)
            .ok_or(ErrorCode::MathOverflow)?;
        
       
        market.total_borrows = market.total_borrows
            .checked_sub(repay_amount)
            .ok_or(ErrorCode::MathOverflow)?;
            
        market.total_borrow_shares = market.total_borrow_shares
            .checked_sub(shares_to_burn)
            .ok_or(ErrorCode::MathOverflow)?;
        
       
        user_position.last_update_timestamp = current_time;
        
       
        emit!(RepayEvent {
            user: self.owner.key(),
            amount: repay_amount,
            shares: shares_to_burn,
            timestamp: current_time,
        });
        
        Ok(())
    }
}

#[event]
pub struct RepayEvent {
    pub user: Pubkey,
    pub amount: u64,
    pub shares: u64,
    pub timestamp: i64,
}