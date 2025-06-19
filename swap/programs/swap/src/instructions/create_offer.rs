use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint, Token}; // ✅ include Token
use anchor_spl::associated_token::AssociatedToken;
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
        seeds = [b"swap", proposer.key().as_ref()],
        bump,
        payer = proposer,
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

    #[account(
        mut,
        constraint = token_1.owner == proposer.key(),
        constraint = token_1.mint == token_1_mint.key(),
    )]
    pub token_1: Box<Account<'info, TokenAccount>>,

    // mints
    pub token_0_mint: Box<Account<'info, Mint>>,
    pub token_1_mint: Box<Account<'info, Mint>>,

    // vaults
    #[account(
        init,
        payer = proposer,
        token::mint = token_0_mint,
        token::authority = offer,
    )]
    pub vault_0: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer = proposer,
        token::mint = token_1_mint,
        token::authority = offer,
    )]
    pub vault_1: Box<Account<'info, TokenAccount>>,

    // programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>, // ✅ FIXED: use `Program`, not `Interface`
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn create_offer(
    _ctx: &Context<CreateOffer>,
    _token_0_amount: u64,
    _token_1_amount: u64,
) -> Result<()> {
    Ok(())
}
