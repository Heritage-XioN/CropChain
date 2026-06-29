//! Deregisters a logistics partner PDA and reclaims rent.
//! Callable only by the farmer (batch authority).

use crate::state::DeregisterLogisticsPartnerCtx;
use anchor_lang::prelude::*;

pub fn handle_deregister_logistics_partner(
    ctx: Context<DeregisterLogisticsPartnerCtx>,
) -> Result<()> {
    let partner_state = &ctx.accounts.partner_state;

    msg!(
        "Logistics partner {} deregistered by farmer {}. Rent reclaimed.",
        partner_state.partner,
        ctx.accounts.farmer.key()
    );
    Ok(())
}
