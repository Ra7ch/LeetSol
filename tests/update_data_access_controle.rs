use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MyAccountData {
    pub owner: Pubkey,
    pub data: u64,
}

pub fn update_data(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let account = next_account_info(account_info_iter)?;
    
    let mut account_data = MyAccountData::try_from_slice(&account.data.borrow())?;
    account_data.data += 1;
    account_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
    
    msg!("Data updated successfully.");
    
    Ok(())
}
