use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Define the state data structure
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct VaultData {
    pub authority: Pubkey,
    pub balance: u64,
}

pub fn withdraw(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let recipient_account = next_account_info(account_info_iter)?;
    
    let mut vault_data = VaultData::try_from_slice(&vault_account.data.borrow())?;
    let amount = 10;
    if vault_data.balance < amount {
        msg!("Insufficient balance.");
        return Err(ProgramError::InsufficientFunds);
    }
    
    vault_data.balance -= amount;
    vault_data.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    msg!("Withdrawn {} tokens to {:?}", amount, recipient_account.key);
    
    Ok(())
}
