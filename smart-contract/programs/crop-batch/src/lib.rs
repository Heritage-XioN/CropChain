pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("A5aLLkRbgS6oqSo3ikmc7Dww3dF6BHdp6znDG4uXtv1b");

#[program]
pub mod crop_batch {
    use super::*;

    pub fn mint_batch(ctx: Context<MintBatch>, name: String) -> Result<()> {
        mint_batch::handler(ctx, name)
    }
}
