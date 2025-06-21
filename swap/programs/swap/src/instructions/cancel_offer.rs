use anchor_lang::prelude::*;
use crate::Offer;
#[derive(Accounts)]
pub struct CancelOffer<'info>{
    pub proposer: Signer<'info>,

    #[account(
        mut, 
        close = proposer,
        seeds = [b"swap", proposer.key().as_ref(), offer.key().as_ref()],
        bump,
        has_one = proposer,

    )]
    pub offer: Box<Account<'info, Offer>>,

}

pub fn cancel_offer(_ctx: &Context<CancelOffer>) -> Result<()> {
    Ok(())
}
