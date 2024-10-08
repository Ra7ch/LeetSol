//this is a test for the access control bug
pub struct Signer {
    pub is_signer: bool,
}

pub static mut BALANCE: u64 = 0;

pub fn update_balance_in_block(new_balance: u64, caller: &Signer) {
    if !caller.is_signer {
        panic!("Caller must sign the transaction");
    }
    unsafe {
        BALANCE = new_balance;
    }
}
