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

declare_id!("7M66PRBrpGP2mT1tTUmaENEGaQ3M9VFCtjEAMyemxKMh");

#[program]
pub mod crop_batch {
    use super::*;

    pub fn mint_batch(ctx: Context<MintBatchCtx>, name: String) -> Result<()> {
        mint_batch::handler(ctx, name)
    }

    pub fn add_checkpoint(ctx: Context<AddCheckpointCtx>, name: String) -> Result<()> {
        add_checkpoint::handler(ctx, name)
    }

    pub fn update_status(ctx: Context<UpdateStatusCtx>, new_status: BatchStatus) -> Result<()> {
        update_status::handler(ctx, new_status)
    }

    pub fn register_logistics_partner(
        ctx: Context<RegisterLogisticsPartnerCtx>,
        partner: Pubkey,
    ) -> Result<()> {
        register_logistics_partner::handler(ctx, partner)
    }

    pub fn deregister_logistics_partner(ctx: Context<DeregisterLogisticsPartnerCtx>) -> Result<()> {
        deregister_logistics_partner::handler(ctx)
    }

    pub fn close_batch(ctx: Context<CloseBatchCtx>) -> Result<()> {
        close_batch::handler(ctx)
    }
}
