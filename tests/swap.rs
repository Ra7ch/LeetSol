pub fn swap_tokens(expected_amount: u64) {
    let actual_amount = perform_swap();

    // Missing slippage check

    // Proceed with the swap
    transfer_tokens(actual_amount);
}
