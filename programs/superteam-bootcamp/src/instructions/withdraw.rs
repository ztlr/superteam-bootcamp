use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::state::Vault;

// Create a Withdraw Context
// #[derive(Accounts)]
// pub struct Withdraw<'info> {
//     ...
// }

// Implement the Withdraw instruction
// impl<'info> Withdraw<'info> {
//     pub fn withdraw(&mut self) -> Result<()> {
        
//         // Perform a CPI to the token program to transfer the funds from the vault to the user

//         // Create the seeds for the CPI
//         let seeds = &[
//             "vault".as_bytes(),
//             self.user.key.as_ref(),
//             &[self.vault_data.bump]
//         ];
//         let signer_seeds = &[&seeds[..]];

//         // Since we are using a PDA, we need to create a cpi context with the seeds signer
//         let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

//     }
// }