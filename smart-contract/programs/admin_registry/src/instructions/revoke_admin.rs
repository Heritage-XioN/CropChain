use crate::state::RevokeAdminCtx;
use anchor_lang::prelude::*;

pub fn handle_revoke_admin(ctx: Context<RevokeAdminCtx>) -> Result<()> {
    let authority = ctx.accounts.authority.key();

    msg!(
        "Admin revoked: {} (by master authority {})",
        ctx.accounts.admin_to_revoke.key(),
        authority
    );
    Ok(())
}
