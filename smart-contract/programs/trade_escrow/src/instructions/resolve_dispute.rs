use crate::state::{DisputeResolution, ResolveDisputeCtx, TradeStatus};
use anchor_lang::prelude::*;

pub const ADMIN_PUBKEY: Pubkey = pubkey!("3sDe7RaEhKvGhby6krh6N3jDT8V9E6P6bdZfibeLRrsy");

pub fn handle_resolve_dispute(
    ctx: Context<ResolveDisputeCtx>,
    resolution: DisputeResolution,
) -> Result<()> {
    let admin = ctx.accounts.admin.key();
    require!(admin == ADMIN_PUBKEY, crate::error::ErrorCode::Unauthorized);

    let trade_account = &mut ctx.accounts.trade_account;

    let batch_key = ctx.accounts.batch_account.key();
    let bump = ctx.bumps.escrow_vault;
    let vault_seeds = &[b"escrow-vault", batch_key.as_ref(), &[bump]];
    let signer = &[&vault_seeds[..]];

    let destination = match resolution {
        DisputeResolution::RefundBuyer => ctx.accounts.buyer.to_account_info(),
        DisputeResolution::PayFarmer => ctx.accounts.farmer.to_account_info(),
    };

    // Transfer total deposited SOL back to destination
    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.system_program.key(),
        anchor_lang::system_program::Transfer {
            from: ctx.accounts.escrow_vault.to_account_info(),
            to: destination.clone(),
        },
        signer,
    );
    anchor_lang::system_program::transfer(transfer_ctx, trade_account.amount)?;

    // Set status to Completed
    trade_account.status = TradeStatus::Completed;

    msg!(
        "Dispute resolved for batch {}. Resolution: {:?}, Funds routed to: {}",
        batch_key,
        resolution,
        destination.key()
    );
    Ok(())
}
