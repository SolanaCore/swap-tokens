use crate::transactions::*;
use trident_fuzz::fuzzing::*;
/// FuzzTransactions contains all available transactions
///
/// You can create your own transactions by adding new variants to the enum.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-transactions/
#[derive(Arbitrary, TransactionSelector)]
pub enum FuzzTransactions {
    AcceptOfferTransaction(AcceptOfferTransaction),
    CancelOfferTransaction(CancelOfferTransaction),
    CreateOfferTransaction(CreateOfferTransaction),
    EditOfferTransaction(EditOfferTransaction),
}
/// FuzzAccounts contains all available accounts
///
/// You can create your own accounts by adding new fields to the struct.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-accounts/
#[derive(Default)]
pub struct FuzzAccounts {
    pub token_1: AccountsStorage,
    pub new_token_0_mint: AccountsStorage,
    pub new_vault_0: AccountsStorage,
    pub token_1_mint: AccountsStorage,
    pub new_token_0: AccountsStorage,
    pub reciever: AccountsStorage,
    pub token_program: AccountsStorage,
    pub proposer_token_1: AccountsStorage,
    pub vault_0: AccountsStorage,
    pub offer: AccountsStorage,
    pub token_0_mint: AccountsStorage,
    pub system_program: AccountsStorage,
    pub rent: AccountsStorage,
    pub token_0: AccountsStorage,
    pub new_token_1_mint: AccountsStorage,
    pub associated_token_program: AccountsStorage,
    pub old_token_0: AccountsStorage,
    pub proposer: AccountsStorage,
    pub vault_1: AccountsStorage,
    pub old_vault_0: AccountsStorage,
}
