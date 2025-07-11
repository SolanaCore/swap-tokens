use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, TransferChecked, TokenAccount, Token, Mint};

pub fn transfer_token<'info>(
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    signer_seeds: &[&[&[u8]]],
    amount: u64,
    token_program: AccountInfo<'info>,
) -> Result<()> {
    let cpi_accounts = Transfer {
        from,
        to,
        authority,
    };

    let cpi_ctx = CpiContext::new_with_signer(
        token_program, //pid
        cpi_accounts, // accs
        signer_seeds, //data / signer_seeds
    );

    token::transfer(cpi_ctx, amount)?;
    Ok(())
}
