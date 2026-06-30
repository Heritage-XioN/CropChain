use anchor_lang::prelude::*;

#[constant]
pub const TRADE_ACCOUNT_SEED: &[u8] = b"trade-account";

#[constant]
pub const ESCROW_VAULT_SEED: &[u8] = b"escrow-vault";

#[constant]
pub const TREASURY_PUBKEY: Pubkey = pubkey!("Treasury11111111111111111111111111111111111");
