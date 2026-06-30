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

declare_id!("76vg8FiFH3hoT98ntU3Sb5apdZC3fQbr5mzzZLCgw1aF");

#[program]
pub mod trade_escrow {
    use super::*;

    pub fn create_trade(ctx: Context<CreateTradeCtx>, amount: u64) -> Result<()> {
        create_trade::handle_create_trade(ctx, amount)
    }

    pub fn accept_trade(ctx: Context<AcceptTradeCtx>) -> Result<()> {
        accept_trade::handle_accept_trade(ctx)
    }

    pub fn confirm_delivery(ctx: Context<ConfirmDeliveryCtx>) -> Result<()> {
        confirm_delivery::handle_confirm_delivery(ctx)
    }

    pub fn raise_dispute(ctx: Context<RaiseDisputeCtx>) -> Result<()> {
        raise_dispute::handle_raise_dispute(ctx)
    }

    pub fn resolve_dispute(
        ctx: Context<ResolveDisputeCtx>,
        resolution: DisputeResolution,
    ) -> Result<()> {
        resolve_dispute::handle_resolve_dispute(ctx, resolution)
    }

    pub fn cancel_trade(ctx: Context<CancelTradeCtx>) -> Result<()> {
        cancel_trade::handle_cancel_trade(ctx)
    }
}
