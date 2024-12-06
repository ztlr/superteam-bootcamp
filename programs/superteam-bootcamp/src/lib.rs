use anchor_lang::prelude::*;

declare_id!("2r68F8AvCe4jVQQKua6BxESNjJS2oiHKmtntmC6A63ms");

mod state;
mod instructions;

use instructions::*;

#[program]
pub mod superteam_bootcamp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        ctx.accounts.withdraw()?;

        Ok(())
    }

    pub fn initialize_fee_vault(ctx: Context<InitializeFeeVault>, withdrawal_fee: u64) -> Result<()> {
        instructions::init_fee_vault(ctx, withdrawal_fee)
    }
}
