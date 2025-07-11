use crate::fuzz_transactions::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, TridentInstruction)]
#[program_id("Swap3sqTyxNUHeUSNg5M6dmCmvsuMdRNdrqm7o2Khrt")]
# [discriminator ([92u8 , 203u8 , 223u8 , 40u8 , 92u8 , 89u8 , 53u8 , 119u8 ,])]
pub struct CancelOfferInstruction {
    pub accounts: CancelOfferInstructionAccounts,
    pub data: CancelOfferInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
#[instruction_data(CancelOfferInstructionData)]
#[storage(FuzzAccounts)]
pub struct CancelOfferInstructionAccounts {
    #[account(signer)]
    proposer: TridentAccount,
    #[account(mut)]
    offer: TridentAccount,
}
/// Instruction Data
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct CancelOfferInstructionData {}
/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for CancelOfferInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_data(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        todo!()
    }
    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        todo!()
    }
}
