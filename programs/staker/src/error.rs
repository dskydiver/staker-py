use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The pool token is not initialized")]
    InitializeError,
    #[msg("The custom error")]
    CustonmError,
    #[msg("The pool is invalid")]
    InvalidPool,
    #[msg("Pool has not enough LST")]
    NotEnoughLST,
    #[msg("Pool has not enough SOL")]
    NotEnoughSOL,
}
