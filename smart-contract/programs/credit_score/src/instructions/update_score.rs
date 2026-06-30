use crate::state::UpdateScoreCtx;
use anchor_lang::prelude::*;

pub fn handle_update_score(ctx: Context<UpdateScoreCtx>, trade_value: u64) -> Result<()> {
    let authority = ctx.accounts.authority.key();
    let farmer = ctx.accounts.farmer.key();
    let batch_account = &ctx.accounts.batch_account;

    // Validate that batch_account authority matches farmer
    {
        let data = batch_account.try_borrow_data()?;
        require!(data.len() >= 40, crate::error::ErrorCode::Unauthorized);
        let authority_bytes: [u8; 32] = data[8..40].try_into().unwrap();
        let batch_authority = Pubkey::from(authority_bytes);
        require!(
            batch_authority == farmer,
            crate::error::ErrorCode::Unauthorized
        );
    }

    let is_farmer = authority == farmer;
    let mut is_authorized = is_farmer;

    if !is_authorized {
        let (expected_trade_pda, _) = Pubkey::find_program_address(
            &[b"trade-account", batch_account.key().as_ref()],
            &ctx.accounts.trade_escrow_program.key(),
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
