pub fn update_balance(accounts: &[AccountInfo], new_balance: u64) -> ProgramResult {
    let account_info = &accounts[0];
    let mut account_data = AccountData::try_from_slice(&account_info.data.borrow())?;
    account_data.balance = new_balance; // <-- State-modifying operation
    account_data.serialize(&mut &mut account_info.data.borrow_mut()[..])?;

    Ok(())
}
