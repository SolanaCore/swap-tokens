use anchor_lang::prelude::*;

//3 | #[constant]
//  | ^^^^^^^^^^^ function or associated item not found in `usize`
#[constant]
pub const ANCHOR_DISCRIMINATOR: u8 = 8;