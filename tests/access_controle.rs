//this is a test for the access control bug

pub fn update_balance(new_balance: u64) {
    BALANCE = new_balance;
}
