//! Creates a new PDA BatchAccount
//! for storing crop metadata.

use crate::state::MintBatch;
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<MintBatch>, name: String) -> Result<()> {
    msg!("Crop batch {} minted by {}", name, ctx.accounts.signer.key);
    Ok(())
}
