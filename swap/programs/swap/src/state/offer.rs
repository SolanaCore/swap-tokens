use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Clone)]
pub struct Offer {
    bump: u8,
    proposer: Pubkey,
    token_0_amount: u64,
    token_1_amount: u64,
    token_0_mint: Pubkey,
    token_1_mint: Pubkey,
    timestamp: i64,
    is_active: bool,
    is_fulfilled: bool,
    is_cancelled: bool,
    offer_id: u64,
}