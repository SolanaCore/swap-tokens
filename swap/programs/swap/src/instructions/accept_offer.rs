use anchor_lang::prelude::*;
use crate::state::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use anchor_spl::associated_token::AssociatedToken;

#[derive(Accounts)]
#[instruction(offer_id: u64)] // <-- Important for PDA derivation
pub struct AcceptOffer<'info> {
    #[account(mut)]
    pub reciever: Signer<'info>,

    pub proposer: SystemAccount<'info>,

    #[account(
        mut,
        close = proposer,
        seeds = [b"swap", proposer.key().as_ref(), &offer_id.to_le_bytes()],
        bump,
        has_one = proposer,
        has_one = token_0_mint,
        has_one = token_1_mint,
        constraint = offer.is_active,
        constraint = !offer.is_fulfilled,
    )]
    pub offer: Box<Account<'info, Offer>>, // ✅ HEAP-allocated

    // Only needed for token::mint verification
    pub token_0_mint: InterfaceAccount<'info, Mint>,
    pub token_1_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = reciever,
        token::mint = token_0_mint,
        token::authority = reciever,
        constraint = token_0.mint == offer.token_1_mint,
    )]
    pub token_0: Box<InterfaceAccount<'info, TokenAccount>>, // Taker's account for receiving proposer token

    #[account(
        init_if_needed,
        payer = reciever,
        token::mint = token_1_mint,
        token::authority = reciever,
        constraint = token_1.mint == offer.token_0_mint,
    )]
    pub token_1: Box<InterfaceAccount<'info, TokenAccount>>, // Taker sends this to vault_0

    #[account(
        mut,
        token::mint = offer.token_0_mint,
        token::authority = offer
    )]
    pub vault_0: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        token::mint = offer.token_1_mint,
        token::authority = offer
    )]
    pub vault_1: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = proposer_token_1.mint == offer.token_1_mint,
        constraint = proposer_token_1.owner == proposer.key(),
        token::authority = proposer,
        token::mint = offer.token_1_mint,
    )]
    pub proposer_token_1: Box<InterfaceAccount<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}


pub fn accept_offer(ctx: Context<AcceptOffer>) -> Result<()> {
    let offer_acc_info = ctx.accounts.offer.to_account_info().clone(); // ✅ clone before borrow
    let offer_acc = ctx.accounts.offer.clone();

    let offer = &mut ctx.accounts.offer;
    let offer_id = offer_acc.offer_id.to_le_bytes();
    let seeds: &[&[u8]] = &[
        b"swap",
        ctx.accounts.proposer.key.as_ref(),
        offer_id.as_ref(), 
        &[offer_acc.bump],
    ];
    let signer_seeds: &[&[&[u8]]] = &[seeds];

    // 1. TAKER -> VAULT_0
    offer.transfer_token(
        ctx.accounts.token_0.to_account_info(),
        ctx.accounts.vault_0.to_account_info(),
        ctx.accounts.proposer.to_account_info(),
        &[],
        offer.token_0_amount,
        &ctx.accounts.token_program.to_account_info(),
    )?;

    // 2. VAULT_1 -> TAKER
    //offer value moved here first so we did offer_pda clone 
    offer.transfer_token(
        ctx.accounts.vault_1.to_account_info(),
        ctx.accounts.token_1.to_account_info(),
        offer_acc_info.clone(),
        signer_seeds,
        offer.token_1_amount,
        &ctx.accounts.token_program.to_account_info(),
    )?;

    // 3. VAULT_0 -> PROPOSER
    offer.transfer_token(
        ctx.accounts.vault_0.to_account_info(),
        ctx.accounts.proposer_token_1.to_account_info(),
        offer_acc_info.clone(),
        signer_seeds,
        offer.token_0_amount,
        &ctx.accounts.token_program.to_account_info(),
    )?;

    offer.is_fulfilled = true;
    offer.is_active = false;
    offer.is_edited = false;
    Ok(())
}
