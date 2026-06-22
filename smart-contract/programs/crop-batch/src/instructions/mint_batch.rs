//! Creates a new PDA BatchAccount
//! for storing crop metadata.

use crate::state::MintBatchCtx;
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<MintBatchCtx>, name: String) -> Result<()> {
    let batch = &mut ctx.accounts.batch_account;
    batch.signer = ctx.accounts.signer.key();
    batch.bump = ctx.bumps.batch_account;
    batch.name = name;

    msg!("Crop batch {} minted by {}", batch.name, batch.signer);
    Ok(())
}
