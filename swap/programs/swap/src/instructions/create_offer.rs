use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::ANCHOR_DISCRIMINATOR;
//Lifetimes
/*
signer
offer pda (init)
user's token_0
user's token_1
token_0 amount
token_1 amount
token_0 mint
token_1 mnt
vault_0
vault_1
*/
#[derive(Accounts)]
pub struct CreateOffer<'info>{
    //signer
    //mut because the account lamport balance will change
    #[account(mut)]
    pub proposer: signer<'info>,
    //offer pda
    //init_if_needed allows the offer to be created if it doesn't exist
    #[account(
        init_if_needed,
        seeds = [b"swap", proposer.key().as_ref()],
        bump,
        space = ANCHOR_DISCRIMINATOR + Offer::INIT_SPACE,
        payer = proposer
    )]
    pub offer: Box<Account<'info, Offer>>,

    //proposer's token_account_0 and token_account_1
    //mut because the account lamport balance will change
    #[account(
        mut,
        constraints = token_0.owner == proposer.key(),
        constraint = token_0.mint == token_0_mint.key(),
    )]
    pub token_0: Box<Account<'info, TokenAccount>>,
    
    #[account(
        mut,
        constraints = token_1.owner == proposer.key(),
        constraint = token_1.mint == token_1_mint.key(),
    )]
    pub token_1: Box<Account<'info, TokenAccount>>,

    //mints
    pub token_0_mint: Box<Account<'info, Mint>>,
    pub token_1_mint: Box<Account<'info, Mint>>,

    //amount
    pub token_0_amount: u64,
    pub token_1_amount: u64,

    //vaults
    #[account(
        mut,
        init,
        seeds = [b"vault", token_0_mint.key().as_ref(), token_1_mint.key().as_ref(), proposer.key().as_ref()],
        bump,
        payer = proposer,
        token::mint = token_0_mint,
        token::authority = offer,   
    )]
    pub const vault_0 : Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        init,
        seeds = [b"vault", token_1_mint.key().as_ref(), token_0_mint.key().as_ref(), proposer.key().as_ref()],
        bump,
        payer = proposer,
        token::mint = token_1_mint,
        token::authority = offer,
    )]
    pub vault_1: Box<Account<'info, TokenAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn create_offer() -> Result<()> {
    Ok(())
}