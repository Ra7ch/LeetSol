use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
    msg,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AccountData {
    pub balance: u64,
}

pub fn update_balance(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    new_balance: u64,
) -> ProgramResult {
    let account_info = &accounts[0];

    if !account_info.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    if account_info.owner != _program_id {
        msg!("Caller does not own this account");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut account_data = AccountData::try_from_slice(&account_info.data.borrow())?;

    account_data.balance = new_balance;

    account_data.serialize(&mut &mut account_info.data.borrow_mut()[..])?;

    Ok(())
}
