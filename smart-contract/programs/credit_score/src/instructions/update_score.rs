use crate::state::UpdateScoreCtx;
use anchor_lang::prelude::*;

pub fn handle_update_score(ctx: Context<UpdateScoreCtx>, trade_value: u64) -> Result<()> {
    let credit_account = &mut ctx.accounts.credit_account;

    // Logic: Increment score based on trade value and history (previous score)
    // Base increment: 10 points
    // Value-based bonus: 1 point per 1,000 lamports/units of trade value, capped at 100 points
    let base_increment = 10u64;
    let value_bonus = (trade_value / 1000).min(100);
    let total_increment = base_increment.checked_add(value_bonus).unwrap();

    credit_account.score = credit_account.score.checked_add(total_increment).unwrap();

    msg!(
        "Credit score updated for farmer {}. Added: {}, New Score: {}",
        credit_account.farmer,
        total_increment,
        credit_account.score
    );
    Ok(())
}
