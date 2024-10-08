// Filename: access_control_bug.rs

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AccountData {
    pub value: u64,
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info = &accounts[0];

    // Deserialize the incoming data
    let incoming_data = AccountData::try_from_slice(instruction_data)?;
    // Deserialize the account data
    let mut account_data = AccountData::try_from_slice(&account_info.data.borrow())?;

    // BUG: No access control check before modifying account data
    account_data.value = incoming_data.value;

    // Serialize the updated data back into the account
    account_data.serialize(&mut &mut account_info.data.borrow_mut()[..])?;

    msg!("Account data updated to: {:?}", account_data.value);

    Ok(())
}
