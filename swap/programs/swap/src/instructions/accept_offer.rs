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

        //as we pass the offer pda, the seeds verify that the offer is derived from (constants, maker, offer_pda)
    //and the has_one constraints verify that the offer pda is owned by the maker and
    #[account(
        mut,
        seeds = [b"swap", proposer.key().as_ref(), offer_pda.key().as_ref()],
        bump,
        has_one = proposer,
        has_one = token_0_mint,
        has_one = token_1_mint,
    )]
    pub offer_pda: Box<Account<'info, Offer>>,

    pub token_0_mint: Box<Account<'info, Mint>>,
    pub token_1_mint: Box<Account<'info, Mint>>,

    //the token accounts of the maker, the offer creator
    //mut because the account lamport balance will change
    #[account(
        init_if_needed,
        payer = reciever,
        token::mint = token_0_mint,
        token::authority = proposer, 
    )]
    pub token_0: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = reciever,
        token::mint = token_1_mint,
        token::authority = proposer, 
    )]
    pub token_1: Box<Account<'info, TokenAccount>>,


    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

}

pub fn accept_offer(_ctx: &Context<AcceptOffer>) -> Result<()> {
    Ok(())
}