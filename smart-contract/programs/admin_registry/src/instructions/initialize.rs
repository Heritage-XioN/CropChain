use crate::state::InitializeCtx;
use anchor_lang::prelude::*;

pub fn handle_initialize(ctx: Context<InitializeCtx>, master_authority: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.master_authority = master_authority;
    config.bump = ctx.bumps.config;

    msg!(
        "ProgramConfig initialized. Master authority set to: {}",
        config.master_authority
    );
    Ok(())
}
