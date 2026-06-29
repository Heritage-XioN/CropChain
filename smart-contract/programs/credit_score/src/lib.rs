pub mod constants;
pub mod error;
pub mod instruction_tests;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("B3AdtMZWq4K4XHuRjDwv88BqYxg4YYCF6FtKnvuZzoNB");

#[program]
pub mod credit_score {
    use super::*;

    pub fn initialize_credit(ctx: Context<InitializeCredit>) -> Result<()> {
        initialize_credit::handler(ctx)
    }
}
