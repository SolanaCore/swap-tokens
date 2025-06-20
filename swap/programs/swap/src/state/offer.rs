use anchor_lang::prelude::*;
#[allow(unused_imports)]
use crate::utils::transfer_token;
use crate::ErrorCode;
#[account]
#[derive(InitSpace, PartialEq)]
pub struct Offer {
    pub bump: u8,
    pub proposer: Pubkey,
    pub token_0_amount: u64,
    pub token_1_amount: u64, 
    pub token_0_mint: Pubkey,
    pub token_1_mint: Pubkey,
    pub token_0: Pubkey,
    pub token_1: Pubkey,
    pub timestamp: i64,
    pub is_active: bool,
    pub is_fulfilled: bool,
    pub is_edited: bool,
    pub offer_id: u64,
}

// impl Offer {
//     //transfer_token
// }

impl<'info> Offer {
    pub fn create_offer(&mut self, token_0_amount: &u64, token_1_amount: &u64, token_0_mint: &Pubkey, token_1_mint: &Pubkey, proposer: &Pubkey, bump:u8, token_0: &Pubkey, token_1: &Pubkey) -> Result<()> {
        assert!(*token_0_amount != 0 as u64 && *token_1_amount != 0 as u64,"{}" ,ErrorCode::InvalidSwapAmount); 
        self.token_0_amount = *token_0_amount;
        self.token_1_amount = *token_1_amount;

        self.token_0_mint = *token_0_mint;
        self.token_1_mint = *token_1_mint;

        let clock = Clock::get()?;
        self.timestamp = clock.unix_timestamp;
        
        self.proposer = *proposer;

        self.bump = bump;

        self.is_active = true;
        self.token_0 = *token_0;
        self.token_1 = *token_1;
        //now transfer token_0 -> Vault_0
        /*
        &self,
        from: &mut Account<'_, TokenAccount>,
        to: &mut Account<'_, TokenAccount>,
        vault_signer_seeds: Option<&[&[u8]]>,
        amount: u64,
        token_program: &Program<'info, Token>,
        */
        Ok(())
    }

    pub fn edit_offer(&mut self, token_0_amount: &u64, token_1_amount: &u64, token_0_mint: &Pubkey, token_1_mint: &Pubkey) -> Result<()>{

        self.token_0_amount = *token_0_amount;
        self.token_1_amount = *token_1_amount;

        self.token_0_mint = *token_0_mint;
        self.token_1_mint = *token_1_mint;

        let clock = Clock::get()?;
        self.timestamp = clock.unix_timestamp;

        self.is_edited = true;
        Ok(())
    } 

    /*
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    signer: &[&[&[u8]]],
    amount: u64,
    token_program: AccountInfo<'info>,
    */
    pub fn transfer_token(
        &self,
        from:  AccountInfo<'info>,
        to: AccountInfo<'info>,
        vault_signer_seeds: &[&[&[u8]]],
        amount: u64,
        token_program: AccountInfo<'info>,
    ) -> Result<()> {
       
        transfer_token(
            from,
            to,
            vault_signer_seeds,
            amount,
            token_program,
        )?;
        Ok(())
    }

}

//ERRORS:
 /*
        error[E0308]: mismatched types
  --> programs/swap/src/state/offer.rs:85:34
   |
85 |         assert!(from.lamports >= amount,"{}" ,ErrorCode::InsufficientBalance);
   |                 -------------    ^^^^^^ expected `Rc<RefCell<&mut u64>>`, found `u64`
   |                 |
   |                 expected because this is `Rc<RefCell<&'info mut u64>>`
   |
   = note: expected struct `Rc<RefCell<&'info mut u64>>`
                found type `u64`
        */

        /*
         transfer_token(
   |         ^^^^^^^^^^^^^^
88 |             from,
   |             ---- expected `AccountInfo<'_>`, found `&mut AccountInfo<'_>`
89 |             to,
   |             -- expected `AccountInfo<'_>`, found `&mut AccountInfo<'_>`
90 |             vault_signer_seeds,
   |             ------------------ expected `&[&[&[u8]]]`, found `&[&[u8]]`

   expected `AccountInfo<'_>`, found `&Program<'_, Token>`
   --> programs/swap/src/state/offer.rs:101:13
    |
101 |             token_program,
    |             ^^^^^^^^^^^^^
    = note: expected struct `anchor_lang::prelude::AccountInfo<'_>`
            found reference `&anchor_lang::prelude::Program<'info, Token>`
note: function defined here
        */