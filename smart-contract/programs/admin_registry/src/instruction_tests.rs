#[cfg(test)]
mod tests {
    use crate::state::AdminState;
    use anchor_lang::prelude::*;

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

    #[test]
    fn test_admin_pda_derivation() {
        let admin_key = Pubkey::new_unique();
        let program_id = crate::id();

        let (pda, _bump) = Pubkey::find_program_address(
            &[
                b"admin",
                admin_key.as_ref(),
            ],
            &program_id,
        );

        let derived = Pubkey::create_program_address(
            &[
                b"admin",
                admin_key.as_ref(),
                &[_bump],
            ],
            &program_id,
        ).unwrap();

        assert_eq!(pda, derived);
    }
}
