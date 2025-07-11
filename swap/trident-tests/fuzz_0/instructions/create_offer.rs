use crate::fuzz_transactions::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, TridentInstruction)]
#[program_id("Swap3sqTyxNUHeUSNg5M6dmCmvsuMdRNdrqm7o2Khrt")]
# [discriminator ([237u8 , 233u8 , 192u8 , 168u8 , 248u8 , 7u8 , 249u8 , 241u8 ,])]
pub struct CreateOfferInstruction {
    pub accounts: CreateOfferInstructionAccounts,
    pub data: CreateOfferInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
#[instruction_data(CreateOfferInstructionData)]
#[storage(FuzzAccounts)]
pub struct CreateOfferInstructionAccounts {
    #[account(mut, signer)]
    proposer: TridentAccount,
    #[account(mut)]
    offer: TridentAccount,
    #[account(mut)]
    token_0: TridentAccount,
    token_0_mint: TridentAccount,
    token_1_mint: TridentAccount,
    #[account(mut, signer)]
    vault_0: TridentAccount,
    #[account(mut, signer)]
    vault_1: TridentAccount,
    #[account(address = "11111111111111111111111111111111")]
    system_program: TridentAccount,
    #[account(address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")]
    token_program: TridentAccount,
    #[account(address = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")]
    associated_token_program: TridentAccount,
}
/// Instruction Data
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct CreateOfferInstructionData {
    token_0_amount: u64,
    token_1_amount: u64,
}
/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for CreateOfferInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_data(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        todo!()
    }
    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        todo!()
    }
}
