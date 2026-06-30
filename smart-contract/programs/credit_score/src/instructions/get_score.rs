use crate::state::{CreditProfile, EligibilityStatus, GetScoreCtx};
use anchor_lang::prelude::*;

pub fn handle_get_score(ctx: Context<GetScoreCtx>) -> Result<CreditProfile> {
    let credit_account = &ctx.accounts.credit_account;

    // Eligibility logic: Eligible if score is >= 50
    let eligibility = if credit_account.score >= 50 {
        EligibilityStatus::Eligible
    } else {
        EligibilityStatus::Ineligible
    };

    Ok(CreditProfile {
        score: credit_account.score,
        eligibility,
    })
}
