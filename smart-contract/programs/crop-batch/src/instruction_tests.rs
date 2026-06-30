#[cfg(test)]
mod tests {
    use crate::state::{
        BatchState, BatchStatus, CheckpointState, FarmerState, LogisticsPartnerState,
    };
    use crate::{
        constants::{BATCH_SEED, CHECKPOINT_SEED, FARMER_SEED, LOGISTICS_PARTNER_SEED},
        id,
    };
    use anchor_lang::prelude::*;

    // ---------------------------------------------------------------------------
    // PDA derivation — batch account
    // ---------------------------------------------------------------------------
    #[test]
    fn test_batch_pda_derivation() {
        let program_id = id();
        let signer = Pubkey::new_unique();
        let name = "SummerHarvest";

        let (batch_pda, bump) = Pubkey::find_program_address(
            &[BATCH_SEED, signer.as_ref(), name.as_bytes()],
            &program_id,
        );

        let (batch_pda_2, bump_2) = Pubkey::find_program_address(
            &[BATCH_SEED, signer.as_ref(), name.as_bytes()],
            &program_id,
        );

        assert_eq!(batch_pda, batch_pda_2);
        assert_eq!(bump, bump_2);
    }

    // ---------------------------------------------------------------------------
    // PDA derivation — farmer account
    // ---------------------------------------------------------------------------
    #[test]
    fn test_farmer_pda_derivation() {
        let program_id = id();
        let signer = Pubkey::new_unique();

        let (farmer_pda, _) =
            Pubkey::find_program_address(&[FARMER_SEED, signer.as_ref()], &program_id);

        let (farmer_pda_2, _) =
            Pubkey::find_program_address(&[FARMER_SEED, signer.as_ref()], &program_id);

        assert_eq!(farmer_pda, farmer_pda_2);
    }

    // ---------------------------------------------------------------------------
    // Account space calculation matches constraint formula
    // ---------------------------------------------------------------------------
    #[test]
    fn test_batch_state_space() {
        let name = "SummerHarvest";
        let batch = BatchState {
            authority: Pubkey::new_unique(),
            bump: 42,
            checkpoint_count: 0,
            status: BatchStatus::Checkpoint(42),
            name: name.to_string(),
        };

        // we use try_serialize to dump the discriminator + data into a buffer
        let mut serialized = Vec::new();
        batch.try_serialize(&mut serialized).unwrap();
        let expected_space = serialized.len();

        // Constraint formula: 8 (discriminator) + 32 (pubkey) + 1 (u8) + 8 (u64 checkpoint_count) + 9 (enum status) + (4 + string len)
        let constraint_space = 8 + 32 + 1 + 8 + 9 + (4 + name.len());

        assert_eq!(
            expected_space, constraint_space,
            "Account space mismatch: serialized={} vs constraint={}",
            expected_space, constraint_space
        );
    }

    // ---------------------------------------------------------------------------
    // BatchState serialize / deserialize round-trip
    // ---------------------------------------------------------------------------
    #[test]
    fn test_batch_state_roundtrip() {
        let original = BatchState {
            authority: Pubkey::new_unique(),
            bump: 255,
            checkpoint_count: 123,
            status: BatchStatus::Checkpoint(999),
            name: "TestHarvest".to_string(),
        };

        let mut bytes = Vec::new();
        original.try_serialize(&mut bytes).unwrap();

        // Use try_deserialize to read it back out from a slice reference
        let mut bytes_slice = bytes.as_slice();
        let deserialized = BatchState::try_deserialize(&mut bytes_slice).unwrap();

        assert_eq!(deserialized.authority, original.authority);
        assert_eq!(deserialized.bump, original.bump);
        assert_eq!(deserialized.checkpoint_count, original.checkpoint_count);
        assert_eq!(deserialized.status, original.status);
        assert_eq!(deserialized.name, original.name);
    }

    // ---------------------------------------------------------------------------
    // FarmerState serialize / deserialize round-trip
    // ---------------------------------------------------------------------------
    #[test]
    fn test_farmer_state_roundtrip() {
        let authority_key = Pubkey::new_unique();
        let original = FarmerState {
            authority: authority_key,
        };

        let mut bytes = Vec::new();
        original.try_serialize(&mut bytes).unwrap();

        let mut bytes_slice = bytes.as_slice();
        let deserialized = FarmerState::try_deserialize(&mut bytes_slice).unwrap();

        assert_eq!(deserialized.authority, authority_key);
    }

    // ---------------------------------------------------------------------------
    // Account space calculation matches constraint formula — FarmerState
    // ---------------------------------------------------------------------------
    #[test]
    fn test_farmer_state_space() {
        let farmer = FarmerState {
            authority: Pubkey::new_unique(),
        };

        let mut serialized = Vec::new();
        farmer.try_serialize(&mut serialized).unwrap();
        let expected_space = serialized.len();

        // Constraint formula: 8 (discriminator) + 32 (pubkey)
        let constraint_space = 8 + 32;

        assert_eq!(
            expected_space, constraint_space,
            "Account space mismatch: serialized={} vs constraint={}",
            expected_space, constraint_space
        );
    }

    // ---------------------------------------------------------------------------
    // Different batch names → different PDAs (same signer)
    // ---------------------------------------------------------------------------
    #[test]
    fn test_unique_batch_names_different_pdas() {
        let program_id = id();
        let signer = Pubkey::new_unique();

        let (pda_1, _) = Pubkey::find_program_address(
            &[BATCH_SEED, signer.as_ref(), b"SummerHarvest"],
            &program_id,
        );

        let (pda_2, _) = Pubkey::find_program_address(
            &[BATCH_SEED, signer.as_ref(), b"WinterHarvest"],
            &program_id,
        );

        assert_ne!(pda_1, pda_2);
    }

    // ---------------------------------------------------------------------------
    // Different signers → different PDAs (same batch name)
    // ---------------------------------------------------------------------------
    #[test]
    fn test_different_signers_different_pdas() {
        let program_id = id();
        let signer_1 = Pubkey::new_unique();
        let signer_2 = Pubkey::new_unique();

        let (pda_1, _) = Pubkey::find_program_address(
            &[BATCH_SEED, signer_1.as_ref(), b"SummerHarvest"],
            &program_id,
        );

        let (pda_2, _) = Pubkey::find_program_address(
            &[BATCH_SEED, signer_2.as_ref(), b"SummerHarvest"],
            &program_id,
        );

        assert_ne!(pda_1, pda_2);
    }

    // ---------------------------------------------------------------------------
    // Checkpoint PDA derivation
    // ---------------------------------------------------------------------------
    #[test]
    fn test_checkpoint_pda_derivation() {
        let program_id = id();
        let batch_account = Pubkey::new_unique();
        let checkpoint_index: u64 = 0;

        let (checkpoint_pda, bump) = Pubkey::find_program_address(
            &[
                CHECKPOINT_SEED,
                batch_account.as_ref(),
                &checkpoint_index.to_le_bytes(),
            ],
            &program_id,
        );

        let (checkpoint_pda_2, bump_2) = Pubkey::find_program_address(
            &[
                CHECKPOINT_SEED,
                batch_account.as_ref(),
                &checkpoint_index.to_le_bytes(),
            ],
            &program_id,
        );

        assert_eq!(checkpoint_pda, checkpoint_pda_2);
        assert_eq!(bump, bump_2);
    }

    // ---------------------------------------------------------------------------
    // Account space calculation matches constraint formula — CheckpointState
    // ---------------------------------------------------------------------------
    #[test]
    fn test_checkpoint_state_space() {
        let name = "DistributionCenter";
        let checkpoint = CheckpointState {
            authority: Pubkey::new_unique(),
            batch: Pubkey::new_unique(),
            index: 42,
            bump: 254,
            name: name.to_string(),
        };

        let mut serialized = Vec::new();
        checkpoint.try_serialize(&mut serialized).unwrap();
        let expected_space = serialized.len();

        // Constraint formula: 8 (discriminator) + 32 (pubkey authority) + 32 (pubkey batch) + 8 (u64 index) + 1 (u8 bump) + (4 + string len)
        let constraint_space = 8 + 32 + 32 + 8 + 1 + (4 + name.len());

        assert_eq!(
            expected_space, constraint_space,
            "Account space mismatch: serialized={} vs constraint={}",
            expected_space, constraint_space
        );
    }

    // ---------------------------------------------------------------------------
    // CheckpointState serialize / deserialize round-trip
    // ---------------------------------------------------------------------------
    #[test]
    fn test_checkpoint_state_roundtrip() {
        let original = CheckpointState {
            authority: Pubkey::new_unique(),
            batch: Pubkey::new_unique(),
            index: 10,
            bump: 253,
            name: "ShipmentDeparted".to_string(),
        };

        let mut bytes = Vec::new();
        original.try_serialize(&mut bytes).unwrap();

        let mut bytes_slice = bytes.as_slice();
        let deserialized = CheckpointState::try_deserialize(&mut bytes_slice).unwrap();

        assert_eq!(deserialized.authority, original.authority);
        assert_eq!(deserialized.batch, original.batch);
        assert_eq!(deserialized.index, original.index);
        assert_eq!(deserialized.bump, original.bump);
        assert_eq!(deserialized.name, original.name);
    }

    // ---------------------------------------------------------------------------
    // Mint batch initial status
    // ---------------------------------------------------------------------------
    #[test]
    fn test_mint_batch_initial_status() {
        let batch = BatchState {
            authority: Pubkey::new_unique(),
            bump: 42,
            checkpoint_count: 0,
            status: BatchStatus::Active,
            name: "InitialBatch".to_string(),
        };
        assert_eq!(batch.status, BatchStatus::Active);
        assert_eq!(batch.checkpoint_count, 0);
    }

    // ---------------------------------------------------------------------------
    // Batch status transitions
    // ---------------------------------------------------------------------------
    #[test]
    fn test_status_transitions() {
        let active = BatchStatus::Active;
        let in_transit = BatchStatus::InTransit;
        let checkpoint = BatchStatus::Checkpoint(0);
        let sold = BatchStatus::Sold;

        // Valid transitions from Active
        assert!(active.can_transition_to(&in_transit));
        assert!(active.can_transition_to(&checkpoint));
        assert!(active.can_transition_to(&sold));
        assert!(!active.can_transition_to(&active));

        // Valid transitions from InTransit
        assert!(in_transit.can_transition_to(&in_transit));
        assert!(in_transit.can_transition_to(&checkpoint));
        assert!(in_transit.can_transition_to(&sold));
        assert!(!in_transit.can_transition_to(&active));

        // Valid transitions from Checkpoint
        assert!(checkpoint.can_transition_to(&in_transit));
        assert!(checkpoint.can_transition_to(&BatchStatus::Checkpoint(1)));
        assert!(checkpoint.can_transition_to(&sold));
        assert!(!checkpoint.can_transition_to(&active));

        // Invalid transitions from Sold (Sold is terminal)
        assert!(!sold.can_transition_to(&active));
        assert!(!sold.can_transition_to(&in_transit));
        assert!(!sold.can_transition_to(&checkpoint));
        assert!(!sold.can_transition_to(&sold));
    }

    // ---------------------------------------------------------------------------
    // LogisticsPartnerState PDA derivation
    // ---------------------------------------------------------------------------
    #[test]
    fn test_logistics_partner_pda_derivation() {
        let program_id = id();
        let farmer = Pubkey::new_unique();
        let partner = Pubkey::new_unique();

        let (partner_pda, bump) = Pubkey::find_program_address(
            &[LOGISTICS_PARTNER_SEED, farmer.as_ref(), partner.as_ref()],
            &program_id,
        );

        let (partner_pda_2, bump_2) = Pubkey::find_program_address(
            &[LOGISTICS_PARTNER_SEED, farmer.as_ref(), partner.as_ref()],
            &program_id,
        );

        assert_eq!(partner_pda, partner_pda_2);
        assert_eq!(bump, bump_2);
    }

    // ---------------------------------------------------------------------------
    // Account space calculation matches constraint formula — LogisticsPartnerState
    // ---------------------------------------------------------------------------
    #[test]
    fn test_logistics_partner_state_space() {
        let partner_state = LogisticsPartnerState {
            farmer: Pubkey::new_unique(),
            partner: Pubkey::new_unique(),
            bump: 255,
        };

        let mut serialized = Vec::new();
        partner_state.try_serialize(&mut serialized).unwrap();
        let expected_space = serialized.len();

        // Constraint formula: 8 (discriminator) + 32 (pubkey farmer) + 32 (pubkey partner) + 1 (u8 bump)
        let constraint_space = 8 + 32 + 32 + 1;

        assert_eq!(
            expected_space, constraint_space,
            "Account space mismatch: serialized={} vs constraint={}",
            expected_space, constraint_space
        );
    }

    // ---------------------------------------------------------------------------
    // LogisticsPartnerState serialize / deserialize round-trip
    // ---------------------------------------------------------------------------
    #[test]
    fn test_logistics_partner_state_roundtrip() {
        let original = LogisticsPartnerState {
            farmer: Pubkey::new_unique(),
            partner: Pubkey::new_unique(),
            bump: 42,
        };

        let mut bytes = Vec::new();
        original.try_serialize(&mut bytes).unwrap();

        let mut bytes_slice = bytes.as_slice();
        let deserialized = LogisticsPartnerState::try_deserialize(&mut bytes_slice).unwrap();

        assert_eq!(deserialized.farmer, original.farmer);
        assert_eq!(deserialized.partner, original.partner);
        assert_eq!(deserialized.bump, original.bump);
    }

    // ---------------------------------------------------------------------------
    // Verifies the farmer seed constant value.
    // ---------------------------------------------------------------------------
    #[test]
    fn test_farmer_seed_constant() {
        assert_eq!(FARMER_SEED, b"farmer");
    }

    // ---------------------------------------------------------------------------
    // Verifies the batch seed constant value.
    // ---------------------------------------------------------------------------
    #[test]
    fn test_batch_seed_constant() {
        assert_eq!(BATCH_SEED, b"batch");
    }

    // ---------------------------------------------------------------------------
    // Verifies the checkpoint seed constant value.
    // ---------------------------------------------------------------------------
    #[test]
    fn test_checkpoint_seed_constant() {
        assert_eq!(CHECKPOINT_SEED, b"checkpoint");
    }

    // ---------------------------------------------------------------------------
    // Verifies the logistics partner seed constant value.
    // ---------------------------------------------------------------------------
    #[test]
    fn test_logistics_partner_seed_constant() {
        assert_eq!(LOGISTICS_PARTNER_SEED, b"logistics-partner");
    }
}
