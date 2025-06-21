use anchor_lang::prelude::*;
use crate::Offer;
 use anchor_spl::token::{Token, TokenAccount, Mint}; // ✅ include Token
//Lifetime
#[derive(Accounts)]
pub struct EditOffer<'info> {
    #[account(mut)]
    pub proposer: Signer<'info>,

    pub new_token_0_mint: Box<Account<'info, Mint>>,
    pub new_token_1_mint: Box<Account<'info, Mint>>,

    #[account(
        mut, 
        close = proposer,
        seeds = [b"swap", proposer.key().as_ref(), offer.key().as_ref()],
        bump = offer.bump,
        has_one = proposer,
        constraint = offer.is_active == true,
        constraint = offer.is_fulfilled == false,
        constraint = offer.is_edited == false,
    )]
    pub offer: Box<Account<'info, Offer>>,

    #[account(
        mut,
        constraint = old_vault_0.owner == offer.key(),
        constraint = old_vault_0.mint == offer.token_0_mint,
    )]
    pub old_vault_0: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = old_token_0.owner == proposer.key(),
        constraint = old_token_0.mint == offer.token_0_mint,
    )]
    pub old_token_0: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = proposer,
        token::mint = new_token_0_mint,
        token::authority = proposer,
                token::token_program = token_program,

    )]
    pub new_token_0: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = proposer,
        token::mint = new_token_0_mint,
        token::authority = offer,
                token::token_program = token_program,

    )]
    pub new_vault_0: Box<Account<'info, TokenAccount>>,

    // Required for init_if_needed
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

/*

token_0_amount: u64,
    token_1_amount: u64, 
    token_0_mint: Pubkey,
    token_1_mint: Pubkey,

*/
pub fn edit_offer(ctx: &mut Context<EditOffer>,token_0_amount: u64, token_1_amount: u64) -> Result<()> {
    let offer = &mut ctx.accounts.offer;
    //old_vault_0:AccountInfo<'_>, old_token_0: AccountInfo<'_>, new_token_0: AccountInfo<'_>, new_vault_0: AccountInfo<'_>)
    offer.edit_offer(token_0_amount, token_1_amount, &ctx.accounts.new_token_0_mint.key(), &ctx.accounts.new_token_1_mint.key(), ctx.accounts.old_vault_0.to_account_info(), ctx.accounts.old_token_0.to_account_info(), ctx.accounts.new_vault_0.to_account_info(), ctx.accounts.new_token_0.to_account_info(), ctx.accounts.token_program.to_account_info())?;
    Ok(())
}
