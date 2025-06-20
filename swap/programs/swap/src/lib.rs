pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;
use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;
pub use utils::*;
pub use error::ErrorCode;

declare_id!("Swap3sqTyxNUHeUSNg5M6dmCmvsuMdRNdrqm7o2Khrt");

#[program]
pub mod swap {
    #[allow(unused_imports)]
    use super::*;
    // pub fn create_offer()-> Result<()>{

    // }

    // pub fn accept_offer() -> Result<()>{

    // }

    // pub fn edit_offer() -> Result<()>{

    // }

    // pub fn cancel_offer() -> Result<()>{

    // }
}
