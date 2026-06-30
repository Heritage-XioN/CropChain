//! Registers a logistics partner PDA.
//! Callable only by the farmer (batch authority).

use crate::state::RegisterLogisticsPartnerCtx;
use anchor_lang::prelude::*;

pub fn handle_register_logistics_partner(
    ctx: Context<RegisterLogisticsPartnerCtx>,
    partner: Pubkey,
) -> Result<()> {
    let partner_state = &mut ctx.accounts.partner_state;
    partner_state.farmer = ctx.accounts.farmer.key();
    partner_state.partner = partner;
    partner_state.bump = ctx.bumps.partner_state;

    msg!(
        "Logistics partner {} registered by farmer {}",
        partner,
        partner_state.farmer
    );
    Ok(())
}
