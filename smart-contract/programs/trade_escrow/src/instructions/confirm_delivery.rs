use crate::constants::{ESCROW_VAULT_SEED, TRADE_ACCOUNT_SEED};
use crate::state::{ConfirmDeliveryCtx, TradeStatus};
use anchor_lang::prelude::*;

pub fn handle_confirm_delivery(ctx: Context<ConfirmDeliveryCtx>) -> Result<()> {
    let trade_account = &mut ctx.accounts.trade_account;

    // Calculate 1% fee
    let amount = trade_account.amount;
    let fee = amount
        .checked_div(100)
        .ok_or(error!(crate::error::ErrorCode::MathOverflow))?;
    let farmer_amount = amount
        .checked_sub(fee)
        .ok_or(error!(crate::error::ErrorCode::MathOverflow))?;

    // Derive escrow vault PDA signer seeds
    let batch_key = ctx.accounts.batch_account.key();
    let bump = ctx.bumps.escrow_vault;
    let vault_seeds = &[ESCROW_VAULT_SEED, batch_key.as_ref(), &[bump]];
    let signer = &[&vault_seeds[..]];

    // Transfer fee to treasury
    let transfer_to_treasury = CpiContext::new_with_signer(
        ctx.accounts.system_program.key(),
        anchor_lang::system_program::Transfer {
            from: ctx.accounts.escrow_vault.to_account_info(),
            to: ctx.accounts.treasury.to_account_info(),
        },
        signer,
    );
    anchor_lang::system_program::transfer(transfer_to_treasury, fee)?;

    // Transfer remaining SOL to farmer
    let transfer_to_farmer = CpiContext::new_with_signer(
        ctx.accounts.system_program.key(),
        anchor_lang::system_program::Transfer {
            from: ctx.accounts.escrow_vault.to_account_info(),
            to: ctx.accounts.farmer.to_account_info(),
        },
        signer,
    );
    anchor_lang::system_program::transfer(transfer_to_farmer, farmer_amount)?;

    // Update status to Completed
    trade_account.status = TradeStatus::Completed;

    // Call credit_score::update_score via CPI
    let cpi_accounts = credit_score::cpi::accounts::UpdateScoreCtx {
        authority: trade_account.to_account_info(),
        farmer: ctx.accounts.farmer.to_account_info(),
        batch_account: ctx.accounts.batch_account.to_account_info(),
        credit_account: ctx.accounts.credit_account.to_account_info(),
    };

    let trade_bump = trade_account.bump;
    let trade_seeds = &[TRADE_ACCOUNT_SEED, batch_key.as_ref(), &[trade_bump]];
    let trade_signer = &[&trade_seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.credit_score_program.key(),
        cpi_accounts,
        trade_signer,
    );

    credit_score::cpi::update_score(cpi_ctx, amount)?;

    msg!(
        "Trade completed for batch {}. Farmer received: {}, Treasury fee: {}. Credit score updated.",
        batch_key,
        farmer_amount,
        fee
    );
    Ok(())
}
