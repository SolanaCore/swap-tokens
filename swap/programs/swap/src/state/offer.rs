use anchor_lang::prelude::*;
#[allow(unused_imports)]
use crate::utils::transfer_token;
use crate::ErrorCode;

#[account]
#[derive(InitSpace, PartialEq)]
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

impl Offer {
    pub fn create_offer(
        &mut self,
        token_0_amount: &u64,
        token_1_amount: &u64,
        token_0_mint: &Pubkey,
        token_1_mint: &Pubkey,
        proposer: &Pubkey,
        bump: u8,
    ) -> Result<()> {
        require!(
            *token_0_amount > 0 && *token_1_amount > 0,
            ErrorCode::InvalidSwapAmount
        );

        self.token_0_amount = *token_0_amount;
        self.token_1_amount = *token_1_amount;
        self.token_0_mint = *token_0_mint;
        self.token_1_mint = *token_1_mint;
        self.proposer = *proposer;
        self.bump = bump;

        self.is_active = true;
        self.is_fulfilled = false;
        self.is_edited = false;

        self.timestamp = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn edit_offer<'info>(
        &mut self,
        token_0_amount: u64,
        token_1_amount: u64,
        new_token_0_mint: &Pubkey,
        new_token_1_mint: &Pubkey,
        old_vault_0: AccountInfo<'info>,
        old_token_0: AccountInfo<'info>,
        new_vault_0: AccountInfo<'info>,
        new_token_0: AccountInfo<'info>,
        authority: AccountInfo<'info>,
        token_program: AccountInfo<'info>,
    ) -> Result<()> {
        let seeds = &[
            b"swap",
            self.proposer.as_ref(),
            &[self.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        // Return old tokens
        self.transfer_token(
            old_vault_0,
            old_token_0,
            authority.clone(),
            signer_seeds,
            self.token_0_amount,
            &token_program,
        )?;

        // Update offer details
        self.token_0_amount = token_0_amount;
        self.token_1_amount = token_1_amount;
        self.token_0_mint = *new_token_0_mint;
        self.token_1_mint = *new_token_1_mint;
        self.timestamp = Clock::get()?.unix_timestamp;
        self.is_edited = true;

        // Deposit new token_0
        self.transfer_token(
            new_token_0,
            new_vault_0,
            authority.clone(),
            &[], // user-signed
            token_0_amount,
            &token_program,
        )?;

        Ok(())
    }

    pub fn transfer_token<'info>(
        &self,
        from: AccountInfo<'info>,
        to: AccountInfo<'info>,
        authority: AccountInfo<'info>, // pass signer or PDA here
        signer_seeds: &[&[&[u8]]],
        amount: u64,
        token_program: &AccountInfo<'info>,
    ) -> Result<()> {
        transfer_token(
            from,
            to,
            authority.clone(), // authority assumed as proposer unless PDA
            signer_seeds,
            amount,
            token_program.clone(),
        )
    }
}
