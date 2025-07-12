pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;
use anchor_lang::prelude::*;

 #[allow(unused_imports)]
 use constants::*;
 use instructions::*;
 use state::*;
 #[allow(unused_imports)]
 use utils::*;
 use error::ErrorCode;

declare_id!("67Hi9cdvL7f4LcoxRBTMeYT6RQaoK6At42wWqbqd5ygd");

#[program]
pub mod swap {
    use super::*;
    
 pub fn create_offer(mut ctx: Context<CreateOffer>, token_0_amount: u64, token_1_amount: u64, offer_id:u64) -> Result<()> {
        instructions::create_offer(&mut ctx, token_0_amount, token_1_amount, offer_id)?;
        Ok(())
    }

    pub fn accept_offer(ctx: Context<AcceptOffer>) -> Result<()>{
        instructions::accept_offer(ctx)?;
        Ok(())
    }

    pub fn edit_offer(mut ctx: Context<EditOffer>,token_0_amount: u64, token_1_amount: u64) -> Result<()>{
        instructions::edit_offer(&mut ctx, token_0_amount, token_1_amount)?;
        Ok(())
    }

    pub fn cancel_offer(_ctx: Context<CancelOffer>) -> Result<()>{
        instructions::cancel_offer(&_ctx)?;
        Ok(())
    }
}

/*
error[E0596]: cannot borrow `ctx` as mutable, as it is not declared as mutable
  --> programs/swap/src/lib.rs:21:36
   |
21 |         instructions::create_offer(&mut ctx, token_0_amount, token_1_amount)?;
   |                                    ^^^^^^^^ cannot borrow as mutable
   |
help: consider changing this to be mutable
   |
20 |  pub fn create_offer(mut ctx: Context<CreateOffer>, token_0_amount: u64, token_1_amount: u64) -> Result<()> {
   |                      +++
*/