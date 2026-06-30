#[cfg(test)]
mod tests {
    use crate::constants::{ESCROW_VAULT_SEED, TRADE_ACCOUNT_SEED};
    use crate::state::{TradeAccount, TradeStatus};
    use anchor_lang::prelude::*;

    // ---------------------------------------------------------------------------
    // Trade account space calculation matches constraint
    // ---------------------------------------------------------------------------
    #[test]
    fn test_trade_account_state_space() {
        // 8 discriminator + 32 buyer + 32 batch + 8 amount + 1 status + 8 accepted_at + 1 bump = 90
        let expected_space = 8 + 32 + 32 + 8 + 1 + 8 + 1;

        let trade = TradeAccount {
            buyer: Pubkey::new_unique(),
            batch: Pubkey::new_unique(),
            amount: 1000,
            status: TradeStatus::Pending,
            accepted_at: 0,
            bump: 254,
        };

        let mut data = Vec::new();
        trade.serialize(&mut data).unwrap();
        assert_eq!(data.len(), expected_space - 8);
    }

    // ---------------------------------------------------------------------------
    // Trade account serialize / deserialize round-trip
    // ---------------------------------------------------------------------------
    #[test]
    fn test_trade_account_state_roundtrip() {
        let trade = TradeAccount {
            buyer: Pubkey::new_unique(),
            batch: Pubkey::new_unique(),
            amount: 5000,
            status: TradeStatus::Active,
            accepted_at: 1234567890,
            bump: 255,
        };

        let mut data = Vec::new();
        trade.serialize(&mut data).unwrap();

        let deserialized = TradeAccount::deserialize(&mut &data[..]).unwrap();
        assert_eq!(trade.buyer, deserialized.buyer);
        assert_eq!(trade.batch, deserialized.batch);
        assert_eq!(trade.amount, deserialized.amount);
        assert_eq!(trade.status, deserialized.status);
        assert_eq!(trade.accepted_at, deserialized.accepted_at);
        assert_eq!(trade.bump, deserialized.bump);
    }

    // ---------------------------------------------------------------------------
    // Trade account PDA derivation
    // ---------------------------------------------------------------------------
    #[test]
    fn test_trade_account_pda_derivation() {
        let batch_key = Pubkey::new_unique();
        let program_id = crate::id();

        let (pda, _bump) =
            Pubkey::find_program_address(&[TRADE_ACCOUNT_SEED, batch_key.as_ref()], &program_id);

        let derived = Pubkey::create_program_address(
            &[TRADE_ACCOUNT_SEED, batch_key.as_ref(), &[_bump]],
            &program_id,
        )
        .unwrap();

        assert_eq!(pda, derived);
    }

    // ---------------------------------------------------------------------------
    // Escrow account PDA derivation
    // ---------------------------------------------------------------------------
    #[test]
    fn test_escrow_vault_pda_derivation() {
        let batch_key = Pubkey::new_unique();
        let program_id = crate::id();

        let (pda, _bump) =
            Pubkey::find_program_address(&[ESCROW_VAULT_SEED, batch_key.as_ref()], &program_id);

        let derived = Pubkey::create_program_address(
            &[ESCROW_VAULT_SEED, batch_key.as_ref(), &[_bump]],
            &program_id,
        )
        .unwrap();

        assert_eq!(pda, derived);
    }
}
