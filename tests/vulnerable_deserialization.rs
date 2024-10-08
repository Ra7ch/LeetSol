use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;

pub fn process(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info = &accounts[0];

    // Vulnerable: No ownership check before deserialization
    let mut data = Data::try_from_slice(&account_info.data.borrow())?;

    // Modify data
    data.value += 1;

    Ok(())
}

#[derive(BorshDeserialize)]
pub struct Data {
    pub value: u64,
}

