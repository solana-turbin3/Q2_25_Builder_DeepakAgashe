use anchor_lang::prelude::*;

use crate::state::{Market, UserPosition};

#[derive(Accounts)]
pub struct Position<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"market", owner.key().as_ref()], 
        bump = market.bump
    )]
    pub market: Account<'info, Market>,

    #[account(
        init,
        payer = owner,
        space = 8 + UserPosition::INIT_SPACE,
        seeds = [b"user_position", owner.key().as_ref()],
        bump
    )]
    pub user_position: Account<'info, UserPosition>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Position<'info> {
    pub fn create_position(&mut self, bumps: &PositionBumps) -> Result<()> {
        let user_position = &mut self.user_position;
        let current_time = Clock::get()?.unix_timestamp;

        user_position.owner = self.owner.key();
        user_position.deposited_jito = 0;
        user_position.deposited_shares = 0;
        user_position.borrowed_sol = 0;
        user_position.borrowed_shares = 0;
        user_position.last_update_timestamp = current_time;
        user_position.bump = bumps.user_position;

        Ok(())
    }
}
