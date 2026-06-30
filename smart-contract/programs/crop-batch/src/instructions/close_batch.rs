//! Closes the batch account and reclaims the rent.
//! Only callable if status is Sold and authority matches the farmer.

use crate::state::CloseBatchCtx;
use anchor_lang::prelude::*;

pub fn handle_close_batch(ctx: Context<CloseBatchCtx>) -> Result<()> {
    let batch = &ctx.accounts.batch_account;

    msg!(
        "Crop batch {} (authority {}) successfully closed by farmer. Rent reclaimed.",
        batch.name,
        batch.authority
    );

    Ok(())
}
