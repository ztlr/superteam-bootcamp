use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, close_account, Mint, Token, TokenAccount, Transfer, CloseAccount};
use anchor_spl::associated_token::AssociatedToken;

use crate::state::Vault;
use crate::state::FeeVault;

// Create a Withdraw Context
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        close = user,
        seeds = [b"vault", user.key.as_ref()],
        bump = vault_data.bump
    )]
    pub vault_data: Account<'info, Vault>,
    #[account(
        init_if_needed,
        payer = user,
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
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(
        mut,
        seeds = [b"fee_vault"],
        bump = fee_vault.bump
    )]
    pub fee_vault: Account<'info, FeeVault>,
    /// CHECK: This is safe as we're only receiving SOL
    #[account(mut)]
    pub fee_receiver: AccountInfo<'info>,
}

// Implement the Withdraw instruction
impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self) -> Result<()> {
        // Transfer fee first
        let fee = self.fee_vault.withdrawal_fee;
        
        // Transfer SOL from user to fee receiver
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &self.user.key(),
            &self.fee_receiver.key(),
            fee,
        );
        
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                self.user.to_account_info(),
                self.fee_receiver.to_account_info(),
                self.system_program.to_account_info(),
            ],
        )?;
        
        // Update fee vault balance
        self.fee_vault.balance = self.fee_vault.balance.checked_add(fee).unwrap_or(self.fee_vault.balance);
        
        // Perform a CPI to the token program to transfer the funds from the vault to the user
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user_ata.to_account_info(),
            authority: self.vault_data.to_account_info()
        };

        // Create the seeds for the CPI
        let seeds = &[
            "vault".as_bytes(),
            self.user.key.as_ref(),
            &[self.vault_data.bump]
        ];
        let signer_seeds = &[&seeds[..]];

        // Since we are using a PDA, we need to create a cpi context with the seeds signer
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, self.vault.amount)?;

        // Close the vault account
        let close_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.user.to_account_info(),
            authority: self.vault_data.to_account_info()
        };
        let close_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), close_accounts, signer_seeds);

        close_account(close_ctx)?;

        Ok(())
    }
}