use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Invalid state transition")]
    InvalidStateTransition,
    #[msg("Batch must be sold before closing")]
    BatchNotSold,
    #[msg("Unauthorized access")]
    Unauthorized,
}
