use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token;
use anchor_spl::token::{Mint, MintTo, Token, TokenAccount};
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        seeds = [b"pool_token"],
        bump,
        mint::decimals = 9,
        mint::authority = pool_token,
    )]
    pub pool_token: Account<'info, Mint>,
    #[account(init, 
        payer=payer,
        seeds=[b"vault_pool_token"],
        bump,
        token::mint = pool_token,
        token::authority = vault_pool_token
    )]
    pub vault_pool_token: Account<'info, TokenAccount>,
    /// CHECK: vault for holding token
    #[account(
        mut, 
        seeds=[b"vault_sol"],
        bump,
    )]
    pub vault_sol: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

 pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        //rent
        let lamports = Rent::get()?.minimum_balance(0);
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.vault_sol.to_account_info(),
                },
            ),
            lamports,
        )?;
        //sol
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.vault_sol.to_account_info(),
                },
            ),
            1_000_000_000,
        )?;
        
        let mint_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                to: ctx.accounts.vault_pool_token.to_account_info(),
                mint: ctx.accounts.pool_token.to_account_info(),
                authority: ctx.accounts.pool_token.to_account_info(),
            },
        );
        let bump = ctx.bumps.pool_token;
        let pda_sign: &[&[u8]] = &[b"pool_token", &[bump]];
        token::mint_to(mint_ctx.with_signer(&[pda_sign]), 10_000_000_000)?;

        Ok(())
    }
