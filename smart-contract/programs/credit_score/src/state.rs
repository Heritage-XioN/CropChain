use crate::constants::{CONFIG_SEED, CREDIT_ACCOUNT_SEED};
use anchor_lang::prelude::*;

#[account]
pub struct CreditConfig {
    pub authority: Pubkey,
    pub trusted_trade_escrow: Pubkey,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub deployer: Signer<'info>,

    #[account(
        init,
        payer = deployer,
        space = 8 + 32 + 32 + 1, // 8 discriminator + 32 authority + 32 trusted_trade_escrow + 1 bump = 73 bytes
        seeds = [CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, CreditConfig>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeCredit<'info> {
    /// signer (the farmer) pays for account creation
    #[account(
        mut,
        constraint = signer.key() == farmer.key() @ crate::error::ErrorCode::Unauthorized
    )]
    pub signer: Signer<'info>,
    /// CHECK: The farmer key whom the credit account belongs to
    pub farmer: UncheckedAccount<'info>,
    /// Credit account PDA to initialize
    #[account(
        init_if_needed,
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

#[derive(Accounts)]
pub struct UpdateScoreCtx<'info> {
    /// The authority (e.g., trade escrow program PDA or authorized signer)
    pub authority: Signer<'info>,
    /// config account
    #[account(
        seeds = [CONFIG_SEED],
        bump = config.bump,
    )]
    pub config: Account<'info, CreditConfig>,
    /// CHECK: The trade escrow program account, verified against config
    #[account(
        constraint = trade_escrow_program.key() == config.trusted_trade_escrow @ crate::error::ErrorCode::Unauthorized
    )]
    pub trade_escrow_program: UncheckedAccount<'info>,
    /// CHECK: The farmer key whom the credit account belongs to
    pub farmer: UncheckedAccount<'info>,
    /// CHECK: Crop batch account used to verify PDA authority seeds
    pub batch_account: UncheckedAccount<'info>,
    /// The credit account PDA to update
    #[account(
        mut,
        seeds = [
            CREDIT_ACCOUNT_SEED,
            farmer.key().as_ref(),
        ],
        bump = credit_account.bump,
        constraint = credit_account.farmer == farmer.key() @ crate::error::ErrorCode::Unauthorized,
    )]
    pub credit_account: Account<'info, CreditAccount>,
}

#[derive(Accounts)]
pub struct GetScoreCtx<'info> {
    /// The credit account PDA to read from
    pub credit_account: Account<'info, CreditAccount>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct CreditProfile {
    pub score: u64,
    pub eligibility: EligibilityStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum EligibilityStatus {
    Eligible,
    Ineligible,
}
