use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("The signer is not authorized to perform this operation.")]
    Unauthorized,
}
