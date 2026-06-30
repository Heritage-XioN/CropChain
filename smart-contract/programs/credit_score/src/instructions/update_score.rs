use crate::state::UpdateScoreCtx;
use anchor_lang::prelude::*;

pub fn handle_update_score(ctx: Context<UpdateScoreCtx>, trade_value: u64) -> Result<()> {
    let authority = ctx.accounts.authority.key();
    let farmer = ctx.accounts.farmer.key();
    let batch_account = &ctx.accounts.batch_account;

    let is_farmer = authority == farmer;
    let mut is_authorized = is_farmer;

    if !is_authorized {
        let trade_escrow_program_id = pubkey!("76vg8FiFH3hoT98ntU3Sb5apdZC3fQbr5mzzZLCgw1aF");
        let (expected_trade_pda, _) = Pubkey::find_program_address(
            &[b"trade-account", batch_account.key().as_ref()],
            &trade_escrow_program_id,
        );
        if authority == expected_trade_pda {
            is_authorized = true;
        }
    }

    require!(is_authorized, crate::error::ErrorCode::Unauthorized);

    let credit_account = &mut ctx.accounts.credit_account;

    // Logic: Increment score based on trade value and history (previous score)
    // Base increment: 10 points
    // Value-based bonus: 1 point per 1,000 lamports/units of trade value, capped at 100 points
    let base_increment = 10u64;
    let value_bonus = (trade_value / 1000).min(100);
    let total_increment = base_increment
        .checked_add(value_bonus)
        .ok_or(error!(crate::error::ErrorCode::MathOverflow))?;

    credit_account.score = credit_account
        .score
        .checked_add(total_increment)
        .ok_or(error!(crate::error::ErrorCode::MathOverflow))?;

    msg!(
        "Credit score updated for farmer {}. Added: {}, New Score: {}",
        credit_account.farmer,
        total_increment,
        credit_account.score
    );
    Ok(())
}
