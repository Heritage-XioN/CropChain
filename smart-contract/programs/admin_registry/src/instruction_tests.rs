#[cfg(test)]
mod tests {
    use crate::constants::CONFIG_SEED;
    use crate::state::{AdminState, ProgramConfig};
    use anchor_lang::prelude::*;

    // ---------------------------------------------------------------------------
    // Verifies that the serialized AdminState matches the allocated space.
    // ---------------------------------------------------------------------------
    #[test]
    fn test_admin_state_space() {
        // 8 discriminator + 32 admin + 1 bump = 41
        let expected_space = 8 + 32 + 1;

        let state = AdminState {
            admin: Pubkey::new_unique(),
            bump: 254,
        };

        let mut data = Vec::new();
        state.serialize(&mut data).unwrap();
        assert_eq!(data.len(), expected_space - 8);
    }

    // ---------------------------------------------------------------------------
    // Verifies that AdminState can be correctly serialized and deserialized.
    // ---------------------------------------------------------------------------
    #[test]
    fn test_admin_state_roundtrip() {
        let state = AdminState {
            admin: Pubkey::new_unique(),
            bump: 255,
        };

        let mut data = Vec::new();
        state.serialize(&mut data).unwrap();

        let deserialized = AdminState::deserialize(&mut &data[..]).unwrap();
        assert_eq!(state.admin, deserialized.admin);
        assert_eq!(state.bump, deserialized.bump);
    }

    // ---------------------------------------------------------------------------
    // Verifies the PDA derivation seeds and bump matching for AdminState.
    // ---------------------------------------------------------------------------
    #[test]
    fn test_admin_pda_derivation() {
        let admin_key = Pubkey::new_unique();
        let program_id = crate::id();

        let (pda, _bump) =
            Pubkey::find_program_address(&[b"admin", admin_key.as_ref()], &program_id);

        let derived =
            Pubkey::create_program_address(&[b"admin", admin_key.as_ref(), &[_bump]], &program_id)
                .unwrap();

        assert_eq!(pda, derived);
    }

    // ---------------------------------------------------------------------------
    // Verifies that the serialized ProgramConfig matches the allocated space.
    // ---------------------------------------------------------------------------
    #[test]
    fn test_program_config_space() {
        // 8 discriminator + 32 master_authority + 1 bump = 41
        let expected_space = 8 + 32 + 1;

        let state = ProgramConfig {
            master_authority: Pubkey::new_unique(),
            bump: 254,
        };

        let mut data = Vec::new();
        state.serialize(&mut data).unwrap();
        assert_eq!(data.len(), expected_space - 8);
    }

    // ---------------------------------------------------------------------------
    // Verifies that ProgramConfig can be correctly serialized and deserialized.
    // ---------------------------------------------------------------------------
    #[test]
    fn test_program_config_roundtrip() {
        let state = ProgramConfig {
            master_authority: Pubkey::new_unique(),
            bump: 255,
        };

        let mut data = Vec::new();
        state.serialize(&mut data).unwrap();

        let deserialized = ProgramConfig::deserialize(&mut &data[..]).unwrap();
        assert_eq!(state.master_authority, deserialized.master_authority);
        assert_eq!(state.bump, deserialized.bump);
    }

    // ---------------------------------------------------------------------------
    // Verifies the PDA derivation seeds and bump matching for ProgramConfig.
    // ---------------------------------------------------------------------------
    #[test]
    fn test_program_config_pda_derivation() {
        let program_id = crate::id();

        let (pda, _bump) = Pubkey::find_program_address(&[CONFIG_SEED], &program_id);

        let derived =
            Pubkey::create_program_address(&[CONFIG_SEED, &[_bump]], &program_id).unwrap();

        assert_eq!(pda, derived);
    }
}
