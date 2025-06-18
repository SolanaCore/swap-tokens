use anchor_lang::prelude::*;

/*
signer
pass that offer pda that he wants to accept
pass the token_account_0 and token_account_1

*/
#[derive(Accounts)]
pub struct AcceptOffer<'info>{
    //signer
    #[account(mut)]
    pub reciever: signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,

    pub token_0_mint: Box<Account<'info, Mint>>,
    pub token_1_mint: Box<Account<'info, Mint>>,

    //the token accounts of the maker, the offer creator
    //mut because the account lamport balance will change
    #[account(
        mut,
        init_if_needed,
        token::mint = token_0_mint,
        token::authority = maker, 
    )]
    pub token_0: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        init_if_needed,
        token::mint = token_1_mint,
        token::authority = maker, 
    )]
    pub token_1: Box<Account<'info, TokenAccount>>,
    //as we pass the offer pda, the seeds verify that the offer is derived from (constants, maker, offer_pda)
    //and the has_one constraints verify that the offer pda is owned by the maker and
    #[account(
        mut,
        seeds = [b"swap", maker.key().as_ref(), offer_pda.key().as_ref()],
        has_one = maker,
        has_one = token_0_mint,
        has_one = token_1_mint,
    )]
    pub offer_pda: Box<Account<'info, Offer>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

}

pub fn accept_offer(ctx: Context<AcceptOffer>) -> Result<()> {
    Ok(())
}