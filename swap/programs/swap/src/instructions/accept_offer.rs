use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint, Token}; // ✅ include Token
use anchor_spl::associated_token::AssociatedToken;
use crate::state::*;
#[allow(unused_imports)]
use crate::constants::ANCHOR_DISCRIMINATOR;
/*
signer
pass that offer pda that he wants to accept
pass the token_account_0 and token_account_1

*/
#[derive(Accounts)]
pub struct AcceptOffer<'info>{
    //signer
    #[account(mut)]
    pub reciever: Signer<'info>,

    #[account(mut)]
    pub proposer: SystemAccount<'info>,

        //as we pass the offer pda, the seeds verify that the offer is derived from (constants, maker, offer)
    //and the has_one constraints verify that the offer pda is owned by the maker and
    #[account(
        mut,
        close = proposer,
        seeds = [b"swap", proposer.key().as_ref(), offer.key().as_ref()],
        bump,
        has_one = proposer,
        has_one = token_0_mint,
        has_one = token_1_mint,
        constraint = offer.is_active == true,
        constraint = offer.is_fulfilled == false,
    )]
    pub offer: Box<Account<'info, Offer>>,
    #[account(
        constraint = offer.token_0_mint == token_0_mint.key(),
    )]
    pub token_0_mint: Box<Account<'info, Mint>>,
    #[account(
        constraint = offer.token_1_mint == token_1_mint.key(),
    )]
    pub token_1_mint: Box<Account<'info, Mint>>,

    //the token accounts of the maker, the offer creator
    //mut because the account lamport balance will change
    #[account(
        init_if_needed,
        payer = reciever,
        token::mint = token_0_mint,
        token::authority = reciever, 
        constraint = token_0.mint.key() == offer.token_1_mint,
        
    )]
    pub token_0: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = reciever,
        token::mint = token_1_mint,
        token::authority = reciever, 
        constraint = token_1.mint.key() == offer.token_0_mint,

    )]
    pub token_1: Box<Account<'info, TokenAccount>>,

    //the vaults of the offer, the offer creator
    #[account(
        mut,
        token::mint = offer.token_0_mint,
        token::authority = offer,
        constraint = vault_0.mint == offer.token_0_mint.key(),
    )]
    pub vault_0: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        token::mint = offer.token_1_mint,
        token::authority = offer,
        constraint = vault_1.mint == offer.token_1_mint.key(),
    )]
    pub vault_1: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        token::mint = offer.token_1_mint,
        token::authority = proposer,
        constraint = proposer_token_1.mint == offer.token_1_mint.key(),
        constraint = proposer_token_1.owner == proposer.key()
    )]
    pub proposer_token_1: Box<Account<'info, TokenAccount>>,


    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

}

pub fn accept_offer(ctx: Context<AcceptOffer>) -> Result<()> {
    let  offer =  &mut ctx.accounts.offer;
let seeds: &[&[u8]] = &[
    b"swap",
    ctx.accounts.proposer.key.as_ref(),
    &[offer.bump],
];

let signer_seeds: &[&[&[u8]]] = &[&seeds[..]]; // type: &[&[&[u8]]]
    
    //TAKER -> VAULT
    offer.transfer_token(
        ctx.accounts.token_0.to_account_info(),
        ctx.accounts.vault_0.to_account_info(),
        &[],
        offer.token_0_amount,
        ctx.accounts.token_program.to_account_info(),
    )?;

    //VAULT -> TAKER
    offer.transfer_token(
        ctx.accounts.vault_1.to_account_info(),
        ctx.accounts.token_1.to_account_info(),
        signer_seeds,
        offer.token_1_amount,
        ctx.accounts.token_program.to_account_info(),
    )?;

    //VAULT -> MAKER
    offer.transfer_token(
        ctx.accounts.vault_0.to_account_info(),
        ctx.accounts.proposer_token_1.to_account_info(),
        signer_seeds,
        offer.token_0_amount,
        ctx.accounts.token_program.to_account_info(),
    )?;
    offer.is_fulfilled = true; // Mark the offer as fulfilled
    offer.is_active = false; // Mark the offer as inactive
    offer.is_edited = false; // Reset the edited state
    Ok(())

}