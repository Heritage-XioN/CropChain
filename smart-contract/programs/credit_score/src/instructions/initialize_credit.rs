//! Creates a CreditAccount PDA for a new farmer.
//! Score starts at 0.

use crate::state::InitializeCredit;
use anchor_lang::prelude::*;

pub fn handle_initialize_credit(ctx: Context<InitializeCredit>) -> Result<()> {
    let credit_account = &mut ctx.accounts.credit_account;
    credit_account.farmer = ctx.accounts.farmer.key();
    credit_account.score = 0;
    credit_account.bump = ctx.bumps.credit_account;

    msg!(
        "Credit account initialized for farmer: {} with score: {}",
        credit_account.farmer,
        credit_account.score
    );
    Ok(())
}
