use crate::fuzz_transactions::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, TridentInstruction)]
#[program_id("Swap3sqTyxNUHeUSNg5M6dmCmvsuMdRNdrqm7o2Khrt")]
# [discriminator ([227u8 , 82u8 , 234u8 , 131u8 , 1u8 , 18u8 , 48u8 , 2u8 ,])]
pub struct AcceptOfferInstruction {
    pub accounts: AcceptOfferInstructionAccounts,
    pub data: AcceptOfferInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
#[instruction_data(AcceptOfferInstructionData)]
#[storage(FuzzAccounts)]
pub struct AcceptOfferInstructionAccounts {
    #[account(mut, signer)]
    reciever: TridentAccount,
    #[account(mut)]
    proposer: TridentAccount,
    #[account(mut)]
    offer: TridentAccount,
    token_0_mint: TridentAccount,
    token_1_mint: TridentAccount,
    #[account(mut, signer)]
    token_0: TridentAccount,
    #[account(mut, signer)]
    token_1: TridentAccount,
    #[account(mut)]
    vault_0: TridentAccount,
    #[account(mut)]
    vault_1: TridentAccount,
    #[account(mut)]
    proposer_token_1: TridentAccount,
    #[account(address = "11111111111111111111111111111111")]
    system_program: TridentAccount,
    #[account(address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")]
    token_program: TridentAccount,
    #[account(address = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")]
    associated_token_program: TridentAccount,
}
/// Instruction Data
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct AcceptOfferInstructionData {}
/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for AcceptOfferInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_data(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        todo!()
    }
    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        todo!()
    }
}
