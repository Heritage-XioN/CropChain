use crate::constants::{ESCROW_VAULT_SEED, TRADE_ACCOUNT_SEED};
use anchor_lang::prelude::*;
use crop_batch::state::BatchState;

#[derive(Accounts)]
pub struct CreateTradeCtx<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// The crop batch being traded
    pub batch_account: Account<'info, BatchState>,
    /// The trade account PDA to initialize
    #[account(
        init,
        payer = buyer,
        space = 8 + 32 + 32 + 8 + 1, // 8 discriminator + 32 buyer + 32 batch + 8 amount + 1 bump
        seeds = [
            TRADE_ACCOUNT_SEED,
            batch_account.key().as_ref(),
        ],
        bump
    )]
    pub trade_account: Account<'info, TradeAccount>,
    /// CHECK: Escrow vault PDA which holds the deposited SOL.
    /// It is derived from the batch_account key.
    #[account(
        mut,
        seeds = [
            ESCROW_VAULT_SEED,
            batch_account.key().as_ref(),
        ],
        bump
    )]
    pub escrow_vault: UncheckedAccount<'info>,
    /// system program required for init
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TradeAccount {
    pub buyer: Pubkey,
    pub batch: Pubkey,
    pub amount: u64,
    pub bump: u8,
}
