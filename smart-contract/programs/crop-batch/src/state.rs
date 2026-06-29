use crate::constants::{BATCH_SEED, CHECKPOINT_SEED, FARMER_SEED, LOGISTICS_PARTNER_SEED};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct MintBatchCtx<'info> {
    /// signer (the farmer) pays for account creation
    #[account(mut)]
    pub signer: Signer<'info>,
    /// farmer PDA — derived from FARMER_SEED + signer pubkey
    /// init-if-needed: created on first mint_batch call, reused on subsequent calls
    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + 32,
        seeds = [FARMER_SEED, signer.key().as_ref()],
        bump
    )]
    pub farmer: Account<'info, FarmerState>,
    /// batch PDA — derived from BATCH_SEED + signer pubkey + batch name
    /// init-if-needed: created on first mint_batch call,
    /// reused on subsequent calls if no new name is provided
    #[account(
        init,
        payer = signer,
        space = 8 + 32 + 1 + 8 + 9 + (4 + name.len()),
        seeds = [BATCH_SEED, signer.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub batch_account: Account<'info, BatchState>,
    /// system program required for init
    pub system_program: Program<'info, System>,
}

#[account]
pub struct FarmerState {
    /// authority of the farmer PDA
    pub authority: Pubkey,
}

#[account]
pub struct BatchState {
    /// authority of the batch PDA
    pub authority: Pubkey, // 32 bytes
    /// bump seed for the batch PDA
    pub bump: u8, // 1 byte
    /// total checkpoints added to this batch
    pub checkpoint_count: u64, // 8 bytes
    /// current status of the batch
    pub status: BatchStatus, // 9 bytes
    /// name of the batch
    pub name: String, // 4 bytes + len
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct AddCheckpointCtx<'info> {
    /// signer (the farmer or logistics partner)
    #[account(mut)]
    pub signer: Signer<'info>,
    /// batch PDA to which the checkpoint will be added
    #[account(mut)]
    pub batch_account: Account<'info, BatchState>,
    /// checkpoint PDA to be created
    #[account(
        init,
        payer = signer,
        space = 8 + 32 + 32 + 8 + 1 + (4 + name.len()),
        seeds = [
            CHECKPOINT_SEED,
            batch_account.key().as_ref(),
            &batch_account.checkpoint_count.to_le_bytes()
        ],
        bump
    )]
    pub checkpoint_account: Account<'info, CheckpointState>,
    /// CHECK: Checked dynamically in instruction handler
    pub partner_state: UncheckedAccount<'info>,
    /// system program required for init
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CheckpointState {
    /// authority of the checkpoint PDA
    pub authority: Pubkey, // 32 bytes
    /// batch PDA it belongs to
    pub batch: Pubkey, // 32 bytes
    /// the index of this checkpoint
    pub index: u64, // 8 bytes
    /// bump seed for the checkpoint PDA
    pub bump: u8, // 1 byte
    /// name of the checkpoint
    pub name: String, // 4 bytes + len
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum BatchStatus {
    Active,
    InTransit,
    Checkpoint(u64),
    Sold,
}

impl BatchStatus {
    pub fn can_transition_to(&self, next: &BatchStatus) -> bool {
        match (self, next) {
            // Cannot transition out of Sold
            (BatchStatus::Sold, _) => false,
            // Cannot transition to Active
            (_, BatchStatus::Active) => false,
            // Normal transitions
            (
                BatchStatus::Active,
                BatchStatus::InTransit | BatchStatus::Checkpoint(_) | BatchStatus::Sold,
            ) => true,
            (
                BatchStatus::InTransit,
                BatchStatus::InTransit | BatchStatus::Checkpoint(_) | BatchStatus::Sold,
            ) => true,
            (
                BatchStatus::Checkpoint(_),
                BatchStatus::InTransit | BatchStatus::Checkpoint(_) | BatchStatus::Sold,
            ) => true,
        }
    }
}

#[derive(Accounts)]
pub struct UpdateStatusCtx<'info> {
    /// authority (the farmer / batch authority)
    pub authority: Signer<'info>,
    /// batch PDA to update
    #[account(
        mut,
        has_one = authority,
    )]
    pub batch_account: Account<'info, BatchState>,
}

#[derive(Accounts)]
pub struct CloseBatchCtx<'info> {
    /// farmer who receives the reclaimed rent
    #[account(mut)]
    pub farmer: Signer<'info>,
    /// batch PDA to close
    #[account(
        mut,
        close = farmer,
        constraint = batch_account.authority == farmer.key() @ crate::error::ErrorCode::Unauthorized,
        constraint = batch_account.status == BatchStatus::Sold @ crate::error::ErrorCode::BatchNotSold,
    )]
    pub batch_account: Account<'info, BatchState>,
}

#[account]
pub struct LogisticsPartnerState {
    /// The farmer who authorized this partner
    pub farmer: Pubkey, // 32 bytes
    /// The authorized partner
    pub partner: Pubkey, // 32 bytes
    /// bump seed
    pub bump: u8, // 1 byte
}

#[derive(Accounts)]
#[instruction(partner: Pubkey)]
pub struct RegisterLogisticsPartnerCtx<'info> {
    /// The farmer (batch authority) who pays for creation
    #[account(mut)]
    pub farmer: Signer<'info>,
    /// The logistics partner state PDA to be created
    #[account(
        init,
        payer = farmer,
        space = 8 + 32 + 32 + 1,
        seeds = [
            LOGISTICS_PARTNER_SEED,
            farmer.key().as_ref(),
            partner.as_ref()
        ],
        bump
    )]
    pub partner_state: Account<'info, LogisticsPartnerState>,
    /// System program
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeregisterLogisticsPartnerCtx<'info> {
    /// The farmer (batch authority) who reclaims the rent
    #[account(mut)]
    pub farmer: Signer<'info>,
    /// The logistics partner state PDA to be closed
    #[account(
        mut,
        close = farmer,
        seeds = [
            LOGISTICS_PARTNER_SEED,
            farmer.key().as_ref(),
            partner_state.partner.as_ref()
        ],
        bump = partner_state.bump,
    )]
    pub partner_state: Account<'info, LogisticsPartnerState>,
}
