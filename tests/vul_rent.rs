use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

pub fn vulnerable_create_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data_size: usize,
) -> ProgramResult {
    let account_info = &accounts[0];
    let payer_info = &accounts[1];
    let system_program_info = &accounts[2];

    create_account(
        payer_info,
        account_info,
        data_size,
        program_id,
        system_program_info,
    )?;


    Ok(())
}