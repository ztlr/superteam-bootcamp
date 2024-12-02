use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

// Create an Initialize Context
// #[derive(Accounts)]
// pub struct Initialize<'info> {
//     #[account(mut)]
//     ...
// }

// Implement the Initialize instruction
// impl<'info> Initialize<'info> {
//     pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
//         ...
//     }
// }