use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
/// File containing all custom types which can be used
/// in transactions and instructions or invariant checks.
///
/// You can define your own custom types here.
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct Offer {
    bump: u8,
    proposer: TridentPubkey,
    token_0_amount: u64,
    token_1_amount: u64,
    token_0_mint: TridentPubkey,
    token_1_mint: TridentPubkey,
    timestamp: i64,
    is_active: bool,
    is_fulfilled: bool,
    is_edited: bool,
    offer_id: u64,
}
