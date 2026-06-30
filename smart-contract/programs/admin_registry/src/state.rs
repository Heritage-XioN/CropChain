use crate::constants::{ADMIN_SEED, CONFIG_SEED};
use anchor_lang::prelude::*;

#[account]
pub struct ProgramConfig {
    pub master_authority: Pubkey,
    pub bump: u8,
}

#[account]
pub struct AdminState {
    pub admin: Pubkey,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct InitializeCtx<'info> {
    /// deployer key
    #[account(mut)]
    pub deployer: Signer<'info>,
    /// config account for creating master authority
    #[account(
        init,
        payer = deployer,
        space = 8 + 32 + 1, // 8 discriminator + 32 master_authority + 1 bump = 41 bytes
        seeds = [CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, ProgramConfig>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddAdminCtx<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        seeds = [CONFIG_SEED],
        bump = config.bump,
        constraint = config.master_authority == authority.key() @ crate::error::ErrorCode::Unauthorized
    )]
    pub config: Account<'info, ProgramConfig>,
    /// CHECK: The admin being authorized
    pub admin_to_add: UncheckedAccount<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 1, // 8 discriminator + 32 admin pubkey + 1 bump = 41 bytes
        seeds = [
            ADMIN_SEED,
            admin_to_add.key().as_ref(),
        ],
        bump
    )]
    pub admin_state: Account<'info, AdminState>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RevokeAdminCtx<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        seeds = [CONFIG_SEED],
        bump = config.bump,
        constraint = config.master_authority == authority.key() @ crate::error::ErrorCode::Unauthorized
    )]
    pub config: Account<'info, ProgramConfig>,
    /// CHECK: The admin being revoked
    pub admin_to_revoke: UncheckedAccount<'info>,
    #[account(
        mut,
        close = authority,
        seeds = [
            ADMIN_SEED,
            admin_to_revoke.key().as_ref(),
        ],
        bump = admin_state.bump,
    )]
    pub admin_state: Account<'info, AdminState>,
}
