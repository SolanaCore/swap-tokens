use anchor_lang::prelude::*;
#[allow(unused_imports)]
use crate::utils::transfer_token;
use crate::ErrorCode;
#[account]
#[derive(InitSpace, PartialEq, Eq)]
pub struct Offer {
    pub bump: u8,
    pub proposer: Pubkey,
    pub token_0_amount: u64,
    pub token_1_amount: u64, 
    pub token_0_mint: Pubkey,
    pub token_1_mint: Pubkey,
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
    pub fn create_offer(&mut self, token_0_amount: &u64, token_1_amount: &u64, token_0_mint: &Pubkey, token_1_mint: &Pubkey, proposer: &Pubkey) -> Result<()> {
        assert!(*token_0_amount != 0 as u64 && *token_1_amount != 0 as u64,"{}" ,ErrorCode::InvalidSwapAmount); 
        self.token_0_amount = *token_0_amount;
        self.token_1_amount = *token_1_amount;

        self.token_0_mint = *token_0_mint;
        self.token_1_mint = *token_1_mint;

        let clock = Clock::get()?;
        self.timestamp = clock.unix_timestamp;
        
        self.proposer = *proposer;
        Ok(())
    }

    pub fn edit_offer(&mut self, token_0_amount: &u64, token_1_amount: &u64, token_0_mint: &Pubkey, token_1_mint: &Pubkey, proposer: &Pubkey) -> Result<()>{
        if self.is_edited == true {
            return Err(ErrorCode::TxAlreadyEditedOnce.into());
        }
        if self.proposer != *proposer{
            return Err(ErrorCode::InvalidOwner.into());
        }

        self.token_0_amount = *token_0_amount;
        self.token_1_amount = *token_1_amount;
        self.token_0_mint = *token_0_mint;
        self.token_1_mint = *token_1_mint;
        let clock = Clock::get()?;
        self.timestamp = clock.unix_timestamp;
        self.is_edited = true;
        Ok(())
    } 
    pub fn take_offer() {

    }
}