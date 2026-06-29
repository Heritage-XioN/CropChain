//! Creates a new PDA BatchAccount
//! for storing crop metadata.

use crate::state::MintBatchCtx;
use anchor_lang::prelude::*;

/// Mints a new crop batch with the given name.
pub fn handler(ctx: Context<MintBatchCtx>, name: String) -> Result<()> {
    let farmer = &mut ctx.accounts.farmer;
    farmer.authority = ctx.accounts.signer.key();

    let batch = &mut ctx.accounts.batch_account;
    batch.authority = ctx.accounts.signer.key();
    batch.bump = ctx.bumps.batch_account;
    batch.checkpoint_count = 0;
    batch.status = crate::state::BatchStatus::Active;
    batch.name = name;

    msg!("Crop batch {} minted by {}", batch.name, batch.authority);
    Ok(())
}
