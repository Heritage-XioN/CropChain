use crate::constants::{ESCROW_VAULT_SEED, TRADE_ACCOUNT_SEED};
use crate::constants::TREASURY_PUBKEY;
use crop_batch::state::BatchState;
use anchor_lang::prelude::*;

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
            TRADE_ACCOUNT_SEED,
            batch_account.key().as_ref(),
        ],
        bump = trade_account.bump,
        constraint = batch_account.authority == farmer.key() @ crate::error::ErrorCode::Unauthorized,
        constraint = trade_account.status == TradeStatus::Pending @ crate::error::ErrorCode::InvalidTradeStatus,
    )]
    pub trade_account: Account<'info, TradeAccount>,
}

#[derive(Accounts)]
pub struct ConfirmDeliveryCtx<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// The crop batch being traded
    pub batch_account: Account<'info, BatchState>,
    /// The trade account PDA to update
    #[account(
        mut,
        seeds = [
            TRADE_ACCOUNT_SEED,
            batch_account.key().as_ref(),
        ],
        bump = trade_account.bump,
        constraint = trade_account.buyer == authority.key() @ crate::error::ErrorCode::Unauthorized,
        constraint = trade_account.status == TradeStatus::Active @ crate::error::ErrorCode::InvalidTradeStatus,
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
    /// CHECK: The farmer key who receives the funds (verified against batch authority)
    #[account(
        mut,
        constraint = batch_account.authority == farmer.key() @ crate::error::ErrorCode::Unauthorized
    )]
    pub farmer: UncheckedAccount<'info>,
    /// CHECK: Treasury account receiving the fee
    #[account(
        mut,
        constraint = treasury.key() == TREASURY_PUBKEY @ crate::error::ErrorCode::Unauthorized
    )]
    pub treasury: UncheckedAccount<'info>,
    /// CHECK: The credit account PDA to update (owned by credit_score program)
    #[account(mut)]
    pub credit_account: UncheckedAccount<'info>,
    /// CHECK: The credit score program config PDA
    #[account(mut)]
    pub credit_config: UncheckedAccount<'info>,
    /// CHECK: The trade escrow program itself (verified to match this program ID)
    #[account(constraint = trade_escrow_program.key() == crate::id())]
    pub trade_escrow_program: UncheckedAccount<'info>,
    pub credit_score_program: Program<'info, credit_score::program::CreditScore>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RaiseDisputeCtx<'info> {
    pub authority: Signer<'info>,
    /// The crop batch being traded
    pub batch_account: Account<'info, BatchState>,
    /// The trade account PDA to update
    #[account(
        mut,
        seeds = [
            TRADE_ACCOUNT_SEED,
            batch_account.key().as_ref(),
        ],
        bump = trade_account.bump,
        constraint = (trade_account.buyer == authority.key() || batch_account.authority == authority.key()) @ crate::error::ErrorCode::Unauthorized,
        constraint = trade_account.status == TradeStatus::Active @ crate::error::ErrorCode::InvalidTradeStatus,
    )]
    pub trade_account: Account<'info, TradeAccount>,
}

#[derive(Accounts)]
pub struct ResolveDisputeCtx<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    /// The admin state PDA of the resolving admin to verify they are registered
    #[account(
        seeds = [
            b"admin",
            admin.key().as_ref(),
        ],
        seeds::program = admin_registry_program.key(),
        bump = admin_state.bump,
        constraint = admin_state.admin == admin.key() @ crate::error::ErrorCode::Unauthorized,
    )]
    pub admin_state: Account<'info, admin_registry::state::AdminState>,
    /// The crop batch being traded
    pub batch_account: Account<'info, BatchState>,
    /// The trade account PDA to update
    #[account(
        mut,
        seeds = [
            TRADE_ACCOUNT_SEED,
            batch_account.key().as_ref(),
        ],
        bump = trade_account.bump,
        constraint = trade_account.status == TradeStatus::Disputed @ crate::error::ErrorCode::InvalidTradeStatus,
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
    /// CHECK: The farmer key who receives the funds (verified against batch authority)
    #[account(
        mut,
        constraint = batch_account.authority == farmer.key() @ crate::error::ErrorCode::Unauthorized
    )]
    pub farmer: UncheckedAccount<'info>,
    /// CHECK: The buyer who receives refund
    #[account(
        mut,
        constraint = trade_account.buyer == buyer.key() @ crate::error::ErrorCode::Unauthorized
    )]
    pub buyer: UncheckedAccount<'info>,
    pub admin_registry_program: Program<'info, admin_registry::program::AdminRegistry>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CancelTradeCtx<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// The crop batch being traded
    pub batch_account: Account<'info, BatchState>,
    /// The trade account PDA to close
    #[account(
        mut,
        close = buyer,
        seeds = [
            TRADE_ACCOUNT_SEED,
            batch_account.key().as_ref(),
        ],
        bump = trade_account.bump,
        constraint = trade_account.buyer == buyer.key() @ crate::error::ErrorCode::Unauthorized,
        constraint = trade_account.status == TradeStatus::Pending @ crate::error::ErrorCode::InvalidTradeStatus,
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
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum TradeStatus {
    Pending,
    Active,
    Completed,
    Disputed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum DisputeResolution {
    RefundBuyer,
    PayFarmer,
}
