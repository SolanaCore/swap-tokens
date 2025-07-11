use crate::fuzz_transactions::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, TridentInstruction)]
#[program_id("Swap3sqTyxNUHeUSNg5M6dmCmvsuMdRNdrqm7o2Khrt")]
# [discriminator ([49u8 , 95u8 , 219u8 , 104u8 , 73u8 , 133u8 , 159u8 , 178u8 ,])]
pub struct EditOfferInstruction {
    pub accounts: EditOfferInstructionAccounts,
    pub data: EditOfferInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
#[instruction_data(EditOfferInstructionData)]
#[storage(FuzzAccounts)]
pub struct EditOfferInstructionAccounts {
    #[account(mut, signer)]
    proposer: TridentAccount,
    new_token_0_mint: TridentAccount,
    new_token_1_mint: TridentAccount,
    #[account(mut)]
    offer: TridentAccount,
    #[account(mut)]
    old_vault_0: TridentAccount,
    #[account(mut)]
    old_token_0: TridentAccount,
    #[account(mut, signer)]
    new_token_0: TridentAccount,
    #[account(mut, signer)]
    new_vault_0: TridentAccount,
    #[account(address = "11111111111111111111111111111111")]
    system_program: TridentAccount,
    #[account(address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")]
    token_program: TridentAccount,
    #[account(address = "SysvarRent111111111111111111111111111111111")]
    rent: TridentAccount,
}
/// Instruction Data
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct EditOfferInstructionData {
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
impl InstructionHooks for EditOfferInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_data(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        todo!()
    }
    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        todo!()
    }
}
