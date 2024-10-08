pub fn secure_swap(user_limit: u64) {
    let trade_result = execute_trade();

    if trade_result < user_limit {
        panic!("Slippage too high: trade result is less than the user's limit");
    }

    transfer(trade_result);
}

fn execute_trade() -> u64 {
    95
}

fn transfer(amount: u64) {
}

