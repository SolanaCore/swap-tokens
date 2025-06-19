pub fn transfer_token<'info>(
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    signer: &[&[&[u8]]],
    amount: u64,
    token_program: AccountInfo<'info>,
) -> Result<()> {
    let cpi_ctx = CpiContext::new_with_signer(
        token_program,
        Transfer {
            from,
            to,
            authority,
        },
        signer,
    );
    transfer(cpi_ctx, amount)?;
    Ok(())
}
