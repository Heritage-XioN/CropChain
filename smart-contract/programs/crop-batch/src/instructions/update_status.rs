//! Transitions the batch status.
//! Callable only by the batch authority (the farmer).

use crate::error::ErrorCode;
use crate::state::{BatchStatus, UpdateStatusCtx};
use anchor_lang::prelude::*;

pub fn handle_update_status(ctx: Context<UpdateStatusCtx>, new_status: BatchStatus) -> Result<()> {
    let batch = &mut ctx.accounts.batch_account;

    // Validate transition
    if !batch.status.can_transition_to(&new_status) {
        return err!(ErrorCode::InvalidStateTransition);
    }

    let old_status = batch.status;
    batch.status = new_status;

    msg!(
        "Batch status updated from {:?} to {:?} by {}",
        old_status,
        batch.status,
        ctx.accounts.authority.key()
    );

    Ok(())
}
