//! Appends a supply chain checkpoint that
//! is callable by the farmer or logistics partner.

use crate::state::AddCheckpointCtx;
use anchor_lang::prelude::*;

/// add a new checkpoint account.
pub fn handle_add_checkpoint(ctx: Context<AddCheckpointCtx>, name: String) -> Result<()> {
    let signer_key = ctx.accounts.signer.key();
    let batch = &mut ctx.accounts.batch_account;

    // Enforce authorization
    if signer_key != batch.authority {
        let partner_state = &ctx.accounts.partner_state;

        // Verify derived PDA address matches
        let (expected_pda, _bump) = Pubkey::find_program_address(
            &[
                crate::constants::LOGISTICS_PARTNER_SEED,
                batch.authority.as_ref(),
                signer_key.as_ref(),
            ],
            ctx.program_id,
        );

        if partner_state.key() != expected_pda {
            return err!(crate::error::ErrorCode::Unauthorized);
        }

        // Verify PDA is initialized and owned by program
        if partner_state.owner != ctx.program_id || partner_state.data_is_empty() {
            return err!(crate::error::ErrorCode::Unauthorized);
        }
    }

    let checkpoint = &mut ctx.accounts.checkpoint_account;

    checkpoint.authority = signer_key;
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
