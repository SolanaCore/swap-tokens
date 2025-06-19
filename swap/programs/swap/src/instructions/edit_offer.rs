use anchor_lang::prelude::*;
//Lifetime
#[derive(Accounts)]
pub struct EditOffer<'info> {

     pub maker: Signer<'info>,

    #[account(
        mut, 
        close = maker,
        seeds = [b"swap", maker.key().as_ref(), offer_pda.key().as_ref()],
        has_one = maker,

    )]
    pub offer_pda: Box<Account<'info, Offer>>,
    
}


pub fn edit_offer() -> Result<()> {
    Ok(())
}
