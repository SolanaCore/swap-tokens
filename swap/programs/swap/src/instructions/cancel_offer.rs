use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CancelOffer<'info>{
    pub maker: Signer<'info>,

    #[account(
        mut, 
        close = maker,
        seeds = [b"swap", maker.key().as_ref(), offer_pda.key().as_ref()],
        has_one = maker,

    )]
    pub offer_pda: Box<Account<'info, Offer>>,

}

pub fn cancel_offer(ctx: Context<CancelOffer>) -> Result<()> {
    Ok(())
}
