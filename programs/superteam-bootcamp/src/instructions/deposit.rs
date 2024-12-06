use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};
use crate::state::Vault;

// Create a Deposit Context
#[derive(Accounts)]
pub struct Deposit<'info> {
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        seeds = [b"vault", user.key.as_ref()],
        bump = vault_data.bump
    )]
    pub vault_data: Account<'info, Vault>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user
    )]
    pub user_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = vault_data
    )]
    pub vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>

}

//Implement the Deposit instruction
impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.user_ata.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.user.to_account_info()
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}