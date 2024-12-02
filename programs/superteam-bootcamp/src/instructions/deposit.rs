use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

// Create a Deposit Context
// #[derive(Accounts)]
// pub struct Deposit<'info> {
//     ...
// }

//Implement the Deposit instruction
// impl<'info> Deposit<'info> {
//     pub fn deposit(&mut self, amount: u64) -> Result<()> {
//         ...
//     }
// }