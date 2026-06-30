use crate::state::InitializeConfig;
use anchor_lang::prelude::*;

pub fn handle_initialize_config(
    ctx: Context<InitializeConfig>,
    master_authority: Pubkey,
    trusted_trade_escrow: Pubkey,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.master_authority = master_authority;
    config.trusted_trade_escrow = trusted_trade_escrow;
    config.bump = ctx.bumps.config;

    msg!(
        "CreditConfig initialized. Master Authority: {}, Trusted Escrow Program: {}",
        config.master_authority,
        config.trusted_trade_escrow
    );
    Ok(())
}
