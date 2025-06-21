use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint, Token}; // ✅ include Token
use anchor_spl::associated_token::AssociatedToken;
use crate::utils::transfer_token;
use crate::state::*;
#[allow(unused_imports)]
use crate::constants::ANCHOR_DISCRIMINATOR;

#[derive(Accounts)]
pub struct CreateOffer<'info> {
    // signer
    #[account(mut)]
    pub proposer: Signer<'info>,

    // offer PDA
    #[account(
        init,
        payer = proposer,
        seeds = [b"swap", proposer.key().as_ref()],
        bump,
        space = ANCHOR_DISCRIMINATOR as usize + Offer::INIT_SPACE,
    )]
    pub offer: Box<Account<'info, Offer>>,

    // token accounts
    #[account(
        mut,
        constraint = token_0.owner == proposer.key(),
        constraint = token_0.mint == token_0_mint.key(),
    )]
    pub token_0: Box<Account<'info, TokenAccount>>,

    // mints
    pub token_0_mint: Box<Account<'info, Mint>>,
    pub token_1_mint: Box<Account<'info, Mint>>,

    // vaults
    #[account(
        init_if_needed,
        payer = proposer,
        token::mint = token_0_mint,
        token::authority = offer,
        token::token_program = token_program,
    )]
    pub vault_0: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = proposer,
        token::mint = token_1_mint,
        token::authority = offer,
        token::token_program = token_program,
    )]
    pub vault_1: Box<Account<'info, TokenAccount>>,

    // programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>, // ✅ FIXED: use `Program`, not `Interface`
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn create_offer(
    ctx: &mut Context<CreateOffer>,
    token_0_amount: u64,
    token_1_amount: u64,
) -> Result<()> {
    /*token_0_amount: &u64, token_1_amount: &u64, token_0_mint: &Pubkey, token_1_mint: &Pubkey, proposer: &Pubkey, bump:u8*/
    let offer = &mut ctx.accounts.offer;
    offer.create_offer(&token_0_amount, &token_1_amount, &ctx.accounts.token_0_mint.key(),&ctx.accounts.token_1_mint.key(), &ctx.accounts.proposer.key(), ctx.bumps.offer)?;
let seeds: &[&[u8]] = &[
    b"swap",
    ctx.accounts.proposer.key.as_ref(),
    &[ctx.accounts.offer.bump],
];

let signer_seeds: &[&[&[u8]]] = &[&seeds[..]]; // type: &[&[&[u8]]]
    transfer_token(
            ctx.accounts.token_0.to_account_info(),
            ctx.accounts.vault_0.to_account_info(),
            signer_seeds,
            token_0_amount,
            ctx.accounts.token_program.to_account_info(),
        )?;
    Ok(())
}
