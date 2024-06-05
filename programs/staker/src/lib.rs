pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
use instructions::*;

declare_id!("4Xi45X7QeEybUF5g1Dhu81BTeXz6Z6VSHveJqsg5uT5J");

#[program]
pub mod staker {
    use super::*;
    pub fn init(ctx: Context<Initialize>) -> Result<()> {
        instructions::init_data::initialize(ctx)
    }
    pub fn pool_init(ctx: Context<InitPool>) -> Result<()> {
        instructions::init_pool::init_pool(ctx)
    }
    pub fn stake(ctx: Context<Operation>, deposit_amount: u64) -> Result<()> {
        instructions::stake::stake(ctx, deposit_amount)
    }
    pub fn unstake(ctx: Context<Operation>, draw_amount: u64) -> Result<()> {
        instructions::unstake::unstake(ctx, draw_amount)
    }
}
