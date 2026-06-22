use crate::constants::{BATCH_SEED, FARMER_SEED};
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
        init,
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
        space = 8 + 32 + 1 + (4 + name.len()),
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
    pub bump: u8,          // 1 byte
    /// name of the batch
    pub name: String,      // 4 bytes + len
}
