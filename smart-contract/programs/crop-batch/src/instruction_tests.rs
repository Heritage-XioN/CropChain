#[cfg(test)]
mod tests {
    use crate::state::{BatchState, FarmerState};
    use crate::{constants::SEED, id};
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
            &[b"batch", signer.as_ref(), name.as_bytes()],
            &program_id,
        );

        let (batch_pda_2, bump_2) = Pubkey::find_program_address(
            &[b"batch", signer.as_ref(), name.as_bytes()],
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
            Pubkey::find_program_address(&[b"farmer", signer.as_ref()], &program_id);

        let (farmer_pda_2, _) =
            Pubkey::find_program_address(&[b"farmer", signer.as_ref()], &program_id);

        assert_eq!(farmer_pda, farmer_pda_2);
    }

    // ---------------------------------------------------------------------------
    // Account space calculation matches constraint formula
    // ---------------------------------------------------------------------------
    #[test]
    fn test_batch_state_space() {
        let name = "SummerHarvest";
        let batch = BatchState {
            signer: Pubkey::new_unique(),
            bump: 42,
            name: name.to_string(),
        };

        // we use try_serialize to dump the discriminator + data into a buffer
        let mut serialized = Vec::new();
        batch.try_serialize(&mut serialized).unwrap();
        let expected_space = serialized.len();

        // Constraint formula: 8 (discriminator) + 32 (pubkey) + 1 (u8) + (4 + string len)
        let constraint_space = 8 + 32 + 1 + (4 + name.len());

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
            signer: Pubkey::new_unique(),
            bump: 255,
            name: "TestHarvest".to_string(),
        };

        let mut bytes = Vec::new();
        original.try_serialize(&mut bytes).unwrap();

        // Use try_deserialize to read it back out from a slice reference
        let mut bytes_slice = bytes.as_slice();
        let deserialized = BatchState::try_deserialize(&mut bytes_slice).unwrap();

        assert_eq!(deserialized.signer, original.signer);
        assert_eq!(deserialized.bump, original.bump);
        assert_eq!(deserialized.name, original.name);
    }

    // ---------------------------------------------------------------------------
    // FarmerState serialize / deserialize round-trip
    // ---------------------------------------------------------------------------
    #[test]
    fn test_farmer_state_roundtrip() {
        let signer_key = Pubkey::new_unique();
        let original = FarmerState { signer: signer_key };

        let mut bytes = Vec::new();
        original.try_serialize(&mut bytes).unwrap();

        let mut bytes_slice = bytes.as_slice();
        let deserialized = FarmerState::try_deserialize(&mut bytes_slice).unwrap();

        assert_eq!(deserialized.signer, signer_key);
    }

    // ---------------------------------------------------------------------------
    // Different batch names → different PDAs (same signer)
    // ---------------------------------------------------------------------------
    #[test]
    fn test_unique_batch_names_different_pdas() {
        let program_id = id();
        let signer = Pubkey::new_unique();

        let (pda_1, _) = Pubkey::find_program_address(
            &[b"batch", signer.as_ref(), b"SummerHarvest"],
            &program_id,
        );

        let (pda_2, _) = Pubkey::find_program_address(
            &[b"batch", signer.as_ref(), b"WinterHarvest"],
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
            &[b"batch", signer_1.as_ref(), b"SummerHarvest"],
            &program_id,
        );

        let (pda_2, _) = Pubkey::find_program_address(
            &[b"batch", signer_2.as_ref(), b"SummerHarvest"],
            &program_id,
        );

        assert_ne!(pda_1, pda_2);
    }

    // ---------------------------------------------------------------------------
    // Seed constant
    // ---------------------------------------------------------------------------
    #[test]
    fn test_seed_constant() {
        assert_eq!(SEED, "anchor");
    }
}
