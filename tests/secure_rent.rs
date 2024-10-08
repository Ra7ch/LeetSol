use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

pub fn correct_create_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data_size: usize,
) -> ProgramResult {
    let account_info = &accounts[0];
    let payer_info = &accounts[1];
    let system_program_info = &accounts[2];
    let rent_sysvar_info = &accounts[3];

    let rent = Rent::from_account_info(rent_sysvar_info)?;
    let minimum_balance = rent.minimum_balance(data_size);

    create_account(
        payer_info,
        account_info,
        minimum_balance,
        data_size,
        program_id,
        system_program_info,
    )?;

    if !rent.is_exempt(account_info.lamports(), data_size) {
        return Err(ProgramError::AccountNotRentExempt.into());
    }

    Ok(())
}

fn create_account(
    payer: &AccountInfo,
    new_account: &AccountInfo,
    lamports: u64,
    space: usize,
    owner: &Pubkey,
    system_program: &AccountInfo,
) -> ProgramResult {

    Ok(())
}