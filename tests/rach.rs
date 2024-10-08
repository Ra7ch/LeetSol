pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info = &accounts[0];

    if instruction_data.is_empty() {
        let mut account_data = AccountData::try_from_slice(&account_info.data.borrow())?;
        account_data.value = 0;
        account_data.serialize(&mut &mut account_info.data.borrow_mut()[..])?;
    } else {
        if !account_info.is_signer {
            msg!("Missing required signature");
            return Err(ProgramError::MissingRequiredSignature);
        }

        let incoming_data = AccountData::try_from_slice(instruction_data)?;
        let mut account_data = AccountData::try_from_slice(&account_info.data.borrow())?;

        account_data.value = incoming_data.value;
        account_data.serialize(&mut &mut account_info.data.borrow_mut()[..])?;
    }

    msg!("Account data updated.");

    Ok(())
}