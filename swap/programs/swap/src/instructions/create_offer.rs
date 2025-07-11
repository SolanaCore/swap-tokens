use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use anchor_spl::token_interface::TokenAccount;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::TokenInterface;
use crate::utils::transfer_token;
use crate::state::*;
#[allow(unused_imports)]
use crate::constants::ANCHOR_DISCRIMINATOR;

#[derive(Accounts)]
pub struct CreateOffer<'info> {
    #[account(mut)]
    pub proposer: Signer<'info>,

    #[account(
        init,
        payer = proposer,
        seeds = [b"swap", proposer.key().as_ref()],
        bump,
        space = ANCHOR_DISCRIMINATOR as usize + Offer::INIT_SPACE,
    )]
    pub offer: Account<'info, Offer>,

    #[account(
        mut,
        constraint = token_0.owner == proposer.key(),
        constraint = token_0.mint == token_0_mint.key(),
    )]
    pub token_0: InterfaceAccount<'info, TokenAccount>,

    pub token_0_mint: InterfaceAccount<'info, Mint>,
    pub token_1_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = proposer,
        token::mint = token_0_mint,
        token::authority = offer,
        token::token_program = token_program,
    )]
    pub vault_0: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = proposer,
        token::mint = token_1_mint,
        token::authority = offer,
        token::token_program = token_program,
    )]
    pub vault_1: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn create_offer(
    ctx: &mut Context<CreateOffer>,
    token_0_amount: u64,
    token_1_amount: u64,
) -> Result<()> {
    let offer = &mut ctx.accounts.offer;
    offer.create_offer(
        &token_0_amount,
        &token_1_amount,
        &ctx.accounts.token_0_mint.key(),
        &ctx.accounts.token_1_mint.key(),
        &ctx.accounts.proposer.key(),
        ctx.bumps.offer,
    )?;

    let seeds: &[&[u8]] = &[
        b"swap",
        ctx.accounts.proposer.key.as_ref(),
        &[ctx.accounts.offer.bump],
    ];

    let signer_seeds: &[&[&[u8]]] = &[seeds];

    transfer_token(
        ctx.accounts.token_0.to_account_info(),
        ctx.accounts.vault_0.to_account_info(),
        ctx.accounts.proposer.to_account_info(),
        signer_seeds,
        token_0_amount,
        ctx.accounts.token_program.to_account_info(),
    )?;

    Ok(())
}
