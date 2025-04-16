use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken, token::{close_account, CloseAccount}, token_2022::transfer_checked, token_interface::{ Mint, TokenAccount, TokenInterface, TransferChecked}
};

use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker : Signer<'info>,

    #[account(mut)]
    pub maker: AccountInfo<'info>,


    #[account(
        mint::token_program = token_program
    )]
   pub mint_a: InterfaceAccount<'info,Mint>,

   #[account(
        mint::token_program = token_program
    )]
   pub mint_b: InterfaceAccount<'info,Mint>,

   #[account(
    mut,
    associated_token::mint = mint_a,
    associated_token::authority = taker,
    associated_token::token_program = token_program
)]
   pub taker_ata_a: InterfaceAccount<'info,TokenAccount>,

   #[account(
    mut,
    associated_token::mint = mint_b,
    associated_token::authority = taker,
    associated_token::token_program = token_program
)]
   pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,

   #[account(
    mut,
    associated_token::mint = mint_b,
    associated_token::authority = maker,
    associated_token::token_program = token_program
)]
   pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,

   #[account(
    mut,
    close = maker,
    has_one = mint_a,
    has_one = mint_b,
    seeds = [b"escrow", taker.key().as_ref(), seed.to_le_bytes().as_ref()],
    bump = escrow.bump
   )]
   pub escrow: Account<'info, Escrow>,

   #[account(
    mut,
    associated_token::mint = mint_a,
    associated_token::authority = escrow,
    associated_token::token_program = token_program

)]
pub vault: InterfaceAccount<'info, TokenAccount>,


pub associated_token_program: Program<'info,AssociatedToken>,
pub token_program: Interface<'info,TokenInterface>,
pub system_program: Program<'info,System>

}

impl <'info> Take<'info> {
    pub fn taker_deposit(&mut self) -> Result<()> {

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: self.taker_ata_b.to_account_info(), //double check
            to: self.maker_ata_b.to_account_info(),
            mint: self.mint_b.to_account_info(),
            authority: self.taker.to_account_info()

        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, self.escrow.receive, self.mint_b.decimals)
    
    }

    pub fn taker_withdraw(&mut self) -> Result<()>{
        
        let signer_seeds: [&[&[u8]]; 1] = [&[
                b"escrow",
                self.maker.to_account_info().key.as_ref(),
                &self.escrow.seed.to_be_bytes()[..],
                &[self.escrow.bump,]
            ]];

            let cpi_program = self.token_program.to_account_info();
            let cpi_accounts = TransferChecked {
                from: self.vault.to_account_info(),
                to: self.taker_ata_a.to_account_info(),
                mint: self.mint_a.to_account_info(),
                authority: self.escrow.to_account_info()
            };

            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);

            transfer_checked(cpi_ctx, self.vault.amount, self.mint_a.decimals)

    }

    pub fn close_vault(&mut self) -> Result<()> {
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump]

        ]];

        let close_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info()
        };

        let cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), close_accounts, &signer_seeds);
        
        close_account(cpi_ctx)
    }
}