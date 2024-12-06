use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub address: Pubkey,
    pub bump: u8,
}

impl Vault {
    pub const SPACE: usize = 8 + // discriminator
        32 + // pubkey
        1;  // bump
}

#[account]
pub struct FeeVault {
    pub authority: Pubkey,
    pub balance: u64,
    pub withdrawal_fee: u64,
    pub bump: u8,
}

impl FeeVault {
    pub const SPACE: usize = 8 + // discriminator
        32 + // pubkey
        8 + // balance
        8 + // withdrawal_fee
        1;  // bump
}