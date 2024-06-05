use crate::error::ErrorCode;
use crate::state::pool::Pool;
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct Operation<'info> {
    pub pool_treasure: Account<'info, Mint>,
    pub pool_token: Account<'info, Mint>,
    #[account(mut, seeds=[b"vault_pool_token", system_program.key().as_ref()], bump)]
    pub pool_token_vault: Account<'info, TokenAccount>, // mint of synthetic token X
    #[account(mut, seeds=[b"vault_sol", system_program.key().as_ref()], bump)]
    /// CHECK: vault for holding token
    pub vault_sol: UncheckedAccount<'info>, // mint to hold token X
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mut)]
    pub sender_pool_token: Account<'info, TokenAccount>,
    #[account(mut, seeds=[b"pool"],bump)]
    pub pool: Account<'info, Pool>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn stake(ctx: Context<Operation>, deposit_amount: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let lst_per_sol = pool.lst_per_sol;

    require!(
        pool.lst_amount > lst_per_sol * deposit_amount,
        ErrorCode::NotEnoughLST
    );
    //transfer SOL from sender -> PDA vault
    let transfer_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.sender.to_account_info(),
            to: ctx.accounts.vault_sol.to_account_info(),
        },
    );
    system_program::transfer(transfer_ctx, deposit_amount)?;
    pool.sol_amount += deposit_amount;
    //transfer pool_token to sender
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
            from: ctx.accounts.pool_token_vault.to_account_info(),
            to: ctx.accounts.sender_pool_token.to_account_info(),
            authority: ctx.accounts.pool_token_vault.to_account_info(),
        },
    );
    let bump = ctx.bumps.pool_token_vault;
    let pubkey = ctx.accounts.system_program.key();
    let pda_sign = &[b"vault_pool_token", pubkey.as_ref(), &[bump]];
    token::transfer(
        transfer_ctx.with_signer(&[pda_sign]),
        deposit_amount * lst_per_sol,
    )?;
    pool.lst_amount -= deposit_amount * lst_per_sol;

    Ok(())
}
