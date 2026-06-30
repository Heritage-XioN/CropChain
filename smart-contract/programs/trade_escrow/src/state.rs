use anchor_lang::prelude::*;
use crop_batch::state::BatchState;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradeStatus {
    Pending,
    Active,
    Completed,
}

#[account]
pub struct TradeAccount {
    pub buyer: Pubkey,
    pub batch: Pubkey,
    pub amount: u64,
    pub status: TradeStatus,
    pub accepted_at: i64,
    pub bump: u8,
}

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
        space = 8 + 32 + 32 + 8 + 1 + 8 + 1, // 8 discriminator + 32 buyer + 32 batch + 8 amount + 1 status + 8 accepted_at + 1 bump = 90
        seeds = [
            b"trade-account",
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
            b"escrow-vault",
            batch_account.key().as_ref(),
        ],
        bump
    )]
    pub escrow_vault: UncheckedAccount<'info>,
    /// system program required for init
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AcceptTradeCtx<'info> {
    #[account(mut)]
    pub farmer: Signer<'info>,
    /// The crop batch being traded, used to verify farmer authority
    pub batch_account: Account<'info, BatchState>,
    /// The trade account PDA to update
    #[account(
        mut,
        seeds = [
            b"trade-account",
            batch_account.key().as_ref(),
        ],
        bump = trade_account.bump,
        constraint = trade_account.batch == batch_account.key() @ crate::error::ErrorCode::Unauthorized,
        constraint = batch_account.authority == farmer.key() @ crate::error::ErrorCode::Unauthorized,
        constraint = trade_account.status == TradeStatus::Pending @ crate::error::ErrorCode::InvalidTradeStatus,
    )]
    pub trade_account: Account<'info, TradeAccount>,
}
