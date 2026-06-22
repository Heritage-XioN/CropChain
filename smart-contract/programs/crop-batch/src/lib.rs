pub mod constants;
pub mod error;
pub mod instruction_tests;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("7M66PRBrpGP2mT1tTUmaENEGaQ3M9VFCtjEAMyemxKMh");

#[program]
pub mod crop_batch {
    use super::*;

    pub fn mint_batch(ctx: Context<MintBatchCtx>, name: String) -> Result<()> {
        mint_batch::handler(ctx, name)
    }
}
