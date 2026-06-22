use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct MintBatchCtx<'info> {
    // signer details
    #[account(mut)]
    pub signer: Signer<'info>,
    // farmer account info
    #[account(has_one = signer)]
    pub farmer: Account<'info, FarmerState>,
    // batch account info
    #[account(
            init,
            payer = signer,
            space = 8 + 32 + 1 + (4 + name.len()),
            seeds = [b"batch", signer.key().as_ref(), name.as_bytes()],
            bump
        )]
    pub batch_account: Account<'info, BatchState>,
    // system program
    pub system_program: Program<'info, System>,
}

#[account]
pub struct FarmerState {
    pub signer: Pubkey,
}

#[account]
pub struct BatchState {
    pub signer: Pubkey, // 32 bytes
    pub bump: u8,       // 1 byte
    pub name: String,   // 4 bytes + len
}
