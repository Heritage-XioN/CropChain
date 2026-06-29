//! Creates a new PDA BatchAccount
//! for storing crop metadata.

use crate::state::MintBatchCtx;
use anchor_lang::prelude::*;

/// Mints a new crop batch with the given name.
pub fn handle_mint_batch(ctx: Context<MintBatchCtx>, name: String) -> Result<()> {
    let farmer = &mut ctx.accounts.farmer;
    let is_new_farmer = farmer.authority == Pubkey::default();
    farmer.authority = ctx.accounts.signer.key();

    if is_new_farmer {
        let cpi_accounts = credit_score::cpi::accounts::InitializeCredit {
            signer: ctx.accounts.signer.to_account_info(),
            farmer: ctx.accounts.signer.to_account_info(),
            credit_account: ctx.accounts.credit_account.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.credit_score_program.key(), cpi_accounts);
        credit_score::cpi::initialize_credit(cpi_ctx)?;
    }

    let batch = &mut ctx.accounts.batch_account;
    batch.authority = ctx.accounts.signer.key();
    batch.bump = ctx.bumps.batch_account;
    batch.checkpoint_count = 0;
    batch.status = crate::state::BatchStatus::Active;
    batch.name = name;

    msg!("Crop batch {} minted by {}", batch.name, batch.authority);
    Ok(())
}
