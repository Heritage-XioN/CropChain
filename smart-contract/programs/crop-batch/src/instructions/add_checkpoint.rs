//! Appends a supply chain checkpoint that
//! is callable by the farmer or logistics partner.

use crate::state::AddCheckpointCtx;
use anchor_lang::prelude::*;

/// add a new checkpoint account.
pub fn handler(ctx: Context<AddCheckpointCtx>, name: String) -> Result<()> {
    let checkpoint = &mut ctx.accounts.checkpoint_account;
    let batch = &mut ctx.accounts.batch_account;

    checkpoint.authority = ctx.accounts.signer.key();
    checkpoint.batch = batch.key();
    checkpoint.index = batch.checkpoint_count;
    checkpoint.bump = ctx.bumps.checkpoint_account;
    checkpoint.name = name;

    let next_status = crate::state::BatchStatus::Checkpoint(checkpoint.index);
    if !batch.status.can_transition_to(&next_status) {
        return err!(crate::error::ErrorCode::InvalidStateTransition);
    }
    batch.status = next_status;

    batch.checkpoint_count = batch.checkpoint_count.checked_add(1).unwrap();

    msg!(
        "Checkpoint added: {} (index {}) to batch {}",
        checkpoint.name,
        checkpoint.index,
        checkpoint.batch
    );

    Ok(())
}
