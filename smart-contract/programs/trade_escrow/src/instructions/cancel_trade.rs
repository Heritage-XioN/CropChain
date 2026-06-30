use crate::constants::ESCROW_VAULT_SEED;
use crate::state::CancelTradeCtx;
use anchor_lang::prelude::*;

pub fn handle_cancel_trade(ctx: Context<CancelTradeCtx>) -> Result<()> {
    let trade_account = &ctx.accounts.trade_account;

    let batch_key = ctx.accounts.batch_account.key();
    let bump = ctx.bumps.escrow_vault;
    let vault_seeds = &[ESCROW_VAULT_SEED, batch_key.as_ref(), &[bump]];
    let signer = &[&vault_seeds[..]];

    // Transfer the total deposited SOL back to the buyer
    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.system_program.key(),
        anchor_lang::system_program::Transfer {
            from: ctx.accounts.escrow_vault.to_account_info(),
            to: ctx.accounts.buyer.to_account_info(),
        },
        signer,
    );
    anchor_lang::system_program::transfer(transfer_ctx, trade_account.amount)?;

    msg!(
        "Trade trade-account for batch {} successfully canceled by buyer {}. Escrow vault refunded.",
        batch_key,
        ctx.accounts.buyer.key()
    );
    Ok(())
}
