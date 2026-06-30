#![allow(clippy::diverging_sub_expression)]

pub mod constants;
pub mod error;
pub mod instruction_tests;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("BaJMP4stm9d7srNjxFJBV5DChSBq2shc47E16sosSpB2");

#[program]
pub mod admin_registry {
    use super::*;

    pub fn initialize(ctx: Context<InitializeCtx>, master_authority: Pubkey) -> Result<()> {
        initialize::handle_initialize(ctx, master_authority)
    }

    pub fn add_admin(ctx: Context<AddAdminCtx>) -> Result<()> {
        add_admin::handle_add_admin(ctx)
    }

    pub fn revoke_admin(ctx: Context<RevokeAdminCtx>) -> Result<()> {
        revoke_admin::handle_revoke_admin(ctx)
    }
}
