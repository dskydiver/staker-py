use crate::state::{Pool, POOL_SIZE};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(init,space=POOL_SIZE, payer=sender, seeds=[b"pool"], bump)]
    pub pool: Account<'info, Pool>,
    pub system_program: Program<'info, System>,
}
pub fn init_pool(ctx: Context<InitPool>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    pool.lst_per_sol = 10;
    pool.sol_amount += 100_000_000_000;
    pool.lst_amount += 10000000000;
    Ok(())
}
