use crate::account;
use anchor_lang::prelude::*;
pub const POOL_SIZE: usize = 8 + 8 + //discriminator
        8 + // lst amount
        8; // sol amount

#[account]
pub struct Pool {
    pub sol_amount: u64,
    pub lst_amount: u64,
    pub lst_per_sol: u64,
}
