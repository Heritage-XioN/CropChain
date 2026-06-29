use crate::constants::CREDIT_ACCOUNT_SEED;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeCredit<'info> {
    /// signer (the farmer) pays for account creation
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: The farmer key whom the credit account belongs to
    pub farmer: UncheckedAccount<'info>,
    /// Credit account PDA to initialize
    #[account(
        init,
        payer = signer,
        space = 8 + 32 + 8 + 1, // 8 discriminator + 32 farmer + 8 score + 1 bump
        seeds = [
            CREDIT_ACCOUNT_SEED,
            farmer.key().as_ref(),
        ],
        bump
    )]
    pub credit_account: Account<'info, CreditAccount>,
    /// System program required for init
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CreditAccount {
    /// The farmer who owns this credit profile
    pub farmer: Pubkey, // 32 bytes
    /// The farmer's credit score (starts at 0)
    pub score: u64, // 8 bytes
    /// bump seed
    pub bump: u8, // 1 byte
}
