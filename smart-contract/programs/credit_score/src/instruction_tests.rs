#[cfg(test)]
mod tests {
    use crate::state::CreditAccount;
    use crate::{constants::CREDIT_ACCOUNT_SEED, id};
    use anchor_lang::prelude::*;

    // ---------------------------------------------------------------------------
    // CreditAccount PDA derivation
    // ---------------------------------------------------------------------------
    #[test]
    fn test_credit_account_pda_derivation() {
        let program_id = id();
        let farmer = Pubkey::new_unique();

        let (pda, bump) =
            Pubkey::find_program_address(&[CREDIT_ACCOUNT_SEED, farmer.as_ref()], &program_id);

        let (pda_2, bump_2) =
            Pubkey::find_program_address(&[CREDIT_ACCOUNT_SEED, farmer.as_ref()], &program_id);

        assert_eq!(pda, pda_2);
        assert_eq!(bump, bump_2);
    }

    // ---------------------------------------------------------------------------
    // CreditAccount space calculation matches constraint
    // ---------------------------------------------------------------------------
    #[test]
    fn test_credit_account_state_space() {
        let state = CreditAccount {
            farmer: Pubkey::new_unique(),
            score: 12345,
            bump: 255,
        };

        let mut serialized = Vec::new();
        state.try_serialize(&mut serialized).unwrap();
        let expected_space = serialized.len();

        // Space formula: 8 (discriminator) + 32 (pubkey farmer) + 8 (u64 score) + 1 (u8 bump)
        let constraint_space = 8 + 32 + 8 + 1;

        assert_eq!(
            expected_space, constraint_space,
            "Account space mismatch: serialized={} vs constraint={}",
            expected_space, constraint_space
        );
    }

    // ---------------------------------------------------------------------------
    // CreditAccount serialize / deserialize round-trip
    // ---------------------------------------------------------------------------
    #[test]
    fn test_credit_account_state_roundtrip() {
        let original = CreditAccount {
            farmer: Pubkey::new_unique(),
            score: 9876543210,
            bump: 42,
        };

        let mut bytes = Vec::new();
        original.try_serialize(&mut bytes).unwrap();

        let mut bytes_slice = bytes.as_slice();
        let deserialized = CreditAccount::try_deserialize(&mut bytes_slice).unwrap();

        assert_eq!(deserialized.farmer, original.farmer);
        assert_eq!(deserialized.score, original.score);
        assert_eq!(deserialized.bump, original.bump);
    }

    // ---------------------------------------------------------------------------
    // Seed constants
    // ---------------------------------------------------------------------------
    #[test]
    fn test_credit_account_seed_constant() {
        assert_eq!(CREDIT_ACCOUNT_SEED, b"credit-account");
    }
}
