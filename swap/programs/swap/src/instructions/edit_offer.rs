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
        constraint = offer_pda.is_active == true,
        constraint = offer_pda.is_fulfilled == false,
        constraint = offer_pda.is_edited == false,

    )]
    pub offer_pda: Box<Account<'info, Offer>>,
    
}

/*

token_0_amount: u64,
    token_1_amount: u64, 
    token_0_mint: Pubkey,
    token_1_mint: Pubkey,

*/
pub fn edit_offer(ctx: &mut Context<EditOffer>,token_0_amount: &u64, token_1_amount: &u64, token_0_mint: &Pubkey, token_1_mint: &Pubkey ) -> Result<()> {
    let offer = &mut ctx.accounts.offer_pda;
    offer.edit_offer(token_0_amount, token_1_amount, token_0_mint, token_1_mint)?;
    Ok(())
}
