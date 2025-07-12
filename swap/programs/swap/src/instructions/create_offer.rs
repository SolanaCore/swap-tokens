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
    #[instruction(offer_id: u64)]
    pub struct CreateOffer<'info> {
        #[account(mut)]
        pub proposer: Signer<'info>,

        #[account(
            init,
            payer = proposer,
            seeds = [b"offer", proposer.key().as_ref(), offer_id.to_le_bytes().as_ref()],
            bump,
            space = ANCHOR_DISCRIMINATOR as usize + Offer::INIT_SPACE,
        )]      
        pub offer: Box<Account<'info, Offer>>,

        #[account(
            mut,
            constraint = token_0.owner == proposer.key(),
            constraint = token_0.mint == token_0_mint.key(),
        )]
        pub token_0: Box<InterfaceAccount<'info, TokenAccount>>,

        pub token_0_mint: Box<InterfaceAccount<'info, Mint>>,
        pub token_1_mint: Box<InterfaceAccount<'info, Mint>>,

        #[account(
            init_if_needed,
            payer = proposer,
            associated_token::mint = token_0_mint,
            associated_token::authority = offer,
            associated_token::token_program = token_program,
        )]
        pub vault_0: Box<InterfaceAccount<'info, TokenAccount>>,

        #[account(
            init_if_needed,
            payer = proposer,
            associated_token::mint = token_1_mint,
            associated_token::authority = offer,
            associated_token::token_program = token_program,
        )]
        pub vault_1: Box<InterfaceAccount<'info, TokenAccount>>,

        pub system_program: Program<'info, System>,
        pub token_program: Interface<'info, TokenInterface>,
        pub associated_token_program: Program<'info, AssociatedToken>,
    }

    pub fn create_offer(
        ctx: &mut Context<CreateOffer>,
        token_0_amount: u64,
        token_1_amount: u64,
        offer_id: u64,
    ) -> Result<()> {
        let offer = &mut ctx.accounts.offer;
        offer.create_offer(
            &token_0_amount,
            &token_1_amount,
            &ctx.accounts.token_0_mint.key(),
            &ctx.accounts.token_1_mint.key(),
            &ctx.accounts.proposer.key(),
            ctx.bumps.offer,
            &offer_id,
        )?;
        
        // let binding = offer_id.to_le_bytes(); 
        //     let seeds: &[&[u8]] = &[
        //     b"offer",
        //     ctx.accounts.proposer.key.as_ref(),
        //     binding.as_ref(), 
        //     &[ctx.bumps.offer],
        // ];

        // let signer_seeds: &[&[&[u8]]] = &[seeds];

        transfer_token(
            ctx.accounts.token_0.to_account_info(),
            ctx.accounts.vault_0.to_account_info(),
            ctx.accounts.proposer.to_account_info(),
            &[],
            token_0_amount,
            ctx.accounts.token_program.to_account_info(),
        )?;
        msg!{
            "offer id: {}", offer_id
        }
        Ok(())
    }
