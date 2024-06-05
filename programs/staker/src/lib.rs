pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
//3mE2Jc9kmsVGoNFZXeXTBwDc8KxBTbZBSSEXrEqu8eMc
declare_id!("2xVHedjn1q57KTESiWTMA9GR4M7Pd5S21ssuurqfaFtd");

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
