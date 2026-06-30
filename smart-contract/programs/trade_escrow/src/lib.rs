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
}
