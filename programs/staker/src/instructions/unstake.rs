use crate::error::ErrorCode;
use crate::instructions::stake::Operation;
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token;

pub fn unstake(ctx: Context<Operation>, draw_amount: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let lst_per_sol = pool.lst_per_sol;

    require!(
        pool.sol_amount > draw_amount / lst_per_sol,
        ErrorCode::NotEnoughSOL
    );
    //send back the LST token to the pool
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
            from: ctx.accounts.sender_pool_token.to_account_info(),
            to: ctx.accounts.pool_token_vault.to_account_info(),
            authority: ctx.accounts.sender.to_account_info(),
        },
    );
    token::transfer(transfer_ctx, draw_amount)?;
    pool.lst_amount += draw_amount;
    // take out pool token from sender
    let pool_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.vault_sol.to_account_info(),
            to: ctx.accounts.sender.to_account_info(),
        },
    );
    let pubkey = ctx.accounts.system_program.key();
    let bump = ctx.bumps.vault_sol;
    let pda_sign = &[b"vault_sol", pubkey.as_ref(), &[bump]];
    system_program::transfer(pool_ctx.with_signer(&[pda_sign]), draw_amount / lst_per_sol)?;
    pool.sol_amount -= draw_amount / lst_per_sol;
    Ok(())
}
