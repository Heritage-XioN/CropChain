use crate::state::{RaiseDisputeCtx, TradeStatus};
use anchor_lang::prelude::*;

pub fn handle_raise_dispute(ctx: Context<RaiseDisputeCtx>) -> Result<()> {
    let trade_account = &mut ctx.accounts.trade_account;

    trade_account.status = TradeStatus::Disputed;

    msg!(
        "Dispute raised for batch {} by authority {}. Status is now Disputed.",
        trade_account.batch,
        ctx.accounts.authority.key()
    );

    Ok(())
}
