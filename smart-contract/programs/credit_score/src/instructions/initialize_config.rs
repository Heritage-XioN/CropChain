use crate::state::InitializeConfig;
use anchor_lang::prelude::*;

pub fn handle_initialize_config(
    ctx: Context<InitializeConfig>,
    authority: Pubkey,
    trusted_trade_escrow: Pubkey,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.authority = authority;
    config.trusted_trade_escrow = trusted_trade_escrow;
    config.bump = ctx.bumps.config;

    msg!(
        "CreditConfig initialized. Authority: {}, Trusted Escrow Program: {}",
        config.authority,
        config.trusted_trade_escrow
    );
    Ok(())
}
