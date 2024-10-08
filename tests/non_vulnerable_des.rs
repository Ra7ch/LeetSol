use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError};

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info = &accounts[0];

    if account_info.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut data = Data::try_from_slice(&account_info.data.borrow())?;

    // Modify data
    data.value += 1;

    Ok(())
}

#[derive(BorshDeserialize)]
pub struct Data {
    pub value: u64,
}
