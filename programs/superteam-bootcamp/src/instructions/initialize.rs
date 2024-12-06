use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};
use crate::state::Vault;
use crate::state::FeeVault;

// Create an Initialize Context
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,
    
    #[account(init,
        payer = user,
        seeds = [b"vault", user.key.as_ref()],
        bump,
        space = Vault::SPACE
    )]
    pub vault_data: Account<'info, Vault>,
    #[account(
        init,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = vault_data
    )]
    pub vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

}

// Implement the Initialize instruction
impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.vault_data.set_inner(Vault {
            address: self.user.key(),
            bump: bumps.vault_data
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeFeeVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = FeeVault::SPACE,
        seeds = [b"fee_vault"],
        bump
    )]
    pub fee_vault: Account<'info, FeeVault>,
    
    pub system_program: Program<'info, System>,
}

pub fn init_fee_vault(ctx: Context<InitializeFeeVault>, withdrawal_fee: u64) -> Result<()> {
    let fee_vault = &mut ctx.accounts.fee_vault;
    fee_vault.authority = ctx.accounts.authority.key();
    fee_vault.balance = 0;
    fee_vault.withdrawal_fee = withdrawal_fee;
    fee_vault.bump = ctx.bumps.fee_vault;
    Ok(())
}