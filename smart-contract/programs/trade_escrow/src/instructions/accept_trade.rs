use crate::state::{AcceptTradeCtx, TradeStatus};
use anchor_lang::prelude::*;

pub fn handle_accept_trade(ctx: Context<AcceptTradeCtx>) -> Result<()> {
    let trade_account = &mut ctx.accounts.trade_account;
    let clock = Clock::get()?;

    trade_account.status = TradeStatus::Active;
    trade_account.accepted_at = clock.unix_timestamp;

    msg!(
        "Trade for batch {} accepted by farmer. Status is now Active at timestamp {}.",
        trade_account.batch,
        trade_account.accepted_at
    );

    Ok(())
}
