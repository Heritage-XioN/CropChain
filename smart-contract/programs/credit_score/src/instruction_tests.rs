#[cfg(test)]
mod tests {
    use crate::state::{CreditAccount, EligibilityStatus};
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
    // update credit score
    // ---------------------------------------------------------------------------
    #[test]
    fn test_update_score_logic() {
        let mut credit_account = CreditAccount {
            farmer: Pubkey::new_unique(),
            score: 50,
            bump: 254,
        };

        // 1. Small trade value: trade_value = 500
        // Base increment = 10
        // Bonus = (500 / 1000).min(100) = 0
        // Total = 10. New score = 50 + 10 = 60
        let base_increment = 10u64;
        let trade_value_1 = 500u64;
        let bonus_1 = (trade_value_1 / 1000).min(100);
        let increment_1 = base_increment + bonus_1;
        credit_account.score += increment_1;
        assert_eq!(credit_account.score, 60);

        // 2. Large trade value: trade_value = 50,000
        // Base increment = 10
        // Bonus = (50000 / 1000).min(100) = 50
        // Total = 60. New score = 60 + 60 = 120
        let trade_value_2 = 50000u64;
        let bonus_2 = (trade_value_2 / 1000).min(100);
        let increment_2 = base_increment + bonus_2;
        credit_account.score += increment_2;
        assert_eq!(credit_account.score, 120);

        // 3. Huge trade value (exceeds cap): trade_value = 500,000
        // Base increment = 10
        // Bonus = (500000 / 1000).min(100) = 100 (capped!)
        // Total = 110. New score = 120 + 110 = 230
        let trade_value_3 = 500000u64;
        let bonus_3 = (trade_value_3 / 1000).min(100);
        let increment_3 = base_increment + bonus_3;
        credit_account.score += increment_3;
        assert_eq!(credit_account.score, 230);
    }

    // ---------------------------------------------------------------------------
    // get score logic test
    // ---------------------------------------------------------------------------
    #[test]
    fn test_get_score_logic() {
        // 1. Ineligible case (score < 50)
        let account_ineligible = CreditAccount {
            farmer: Pubkey::new_unique(),
            score: 49,
            bump: 255,
        };
        let status_ineligible = if account_ineligible.score >= 50 {
            EligibilityStatus::Eligible
        } else {
            EligibilityStatus::Ineligible
        };
        assert_eq!(status_ineligible, EligibilityStatus::Ineligible);

        // 2. Eligible case (score >= 50)
        let account_eligible = CreditAccount {
            farmer: Pubkey::new_unique(),
            score: 50,
            bump: 255,
        };
        let status_eligible = if account_eligible.score >= 50 {
            EligibilityStatus::Eligible
        } else {
            EligibilityStatus::Ineligible
        };
        assert_eq!(status_eligible, EligibilityStatus::Eligible);
    }

    // ---------------------------------------------------------------------------
    // Seed constants
    // ---------------------------------------------------------------------------
    #[test]
    fn test_credit_account_seed_constant() {
        assert_eq!(CREDIT_ACCOUNT_SEED, b"credit-account");
    }
}
