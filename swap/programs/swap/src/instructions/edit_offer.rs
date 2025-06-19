use anchor_lang::prelude::*;
use crate::Offer;
//Lifetime
#[derive(Accounts)]
pub struct EditOffer<'info> {

     pub proposer: Signer<'info>,

    #[account(
        mut, 
        close = proposer,
        seeds = [b"swap", proposer.key().as_ref(), offer_pda.key().as_ref()],
        bump,
        has_one = proposer,

    )]
    pub offer_pda: Box<Account<'info, Offer>>,
    
}

/*

token_0_amount: u64,
    token_1_amount: u64, 
    token_0_mint: Pubkey,
    token_1_mint: Pubkey,

*/
pub fn edit_offer(ctx: &mut Context<EditOffer>,token_0_amount: &u64, token_1_amount: &u64, token_0_mint: &Pubkey, token_1_mint: &Pubkey, proposer: &Pubkey ) -> Result<()> {
    let offer = &mut ctx.accounts.offer_pda;
    offer.edit_offer(token_0_amount, token_1_amount, token_0_mint, token_1_mint, proposer)?;
    Ok(())
}
