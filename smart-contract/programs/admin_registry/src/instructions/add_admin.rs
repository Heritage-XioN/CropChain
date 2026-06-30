use crate::state::AddAdminCtx;
use anchor_lang::prelude::*;

pub fn handle_add_admin(ctx: Context<AddAdminCtx>) -> Result<()> {
    let authority = ctx.accounts.authority.key();
    let admin_state = &mut ctx.accounts.admin_state;
    admin_state.admin = ctx.accounts.admin_to_add.key();
    admin_state.bump = ctx.bumps.admin_state;

    msg!(
        "Admin authorized: {} (registered by master authority {})",
        admin_state.admin,
        authority
    );
    Ok(())
}
