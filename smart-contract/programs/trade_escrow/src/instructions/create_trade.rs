use crate::state::CreateTradeCtx;
use anchor_lang::prelude::*;

pub fn handle_create_trade(ctx: Context<CreateTradeCtx>, amount: u64) -> Result<()> {
    let trade_account = &mut ctx.accounts.trade_account;
    trade_account.buyer = ctx.accounts.buyer.key();
    trade_account.batch = ctx.accounts.batch_account.key();
    trade_account.amount = amount;
    trade_account.bump = ctx.bumps.trade_account;

    // Perform CPI to transfer SOL from buyer to escrow_vault
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.key(),
        anchor_lang::system_program::Transfer {
            from: ctx.accounts.buyer.to_account_info(),
            to: ctx.accounts.escrow_vault.to_account_info(),
        },
    );
    anchor_lang::system_program::transfer(cpi_context, amount)?;

    msg!(
        "Trade created for batch {}. Buyer: {}, Amount: {}, Vault: {}",
        trade_account.batch,
        trade_account.buyer,
        trade_account.amount,
        ctx.accounts.escrow_vault.key()
    );

    Ok(())
}
