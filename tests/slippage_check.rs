use std::env;
use std::fs;
use syn::{
    parse_file, BinOp, Block, Expr, ExprBinary, ExprCall, ExprIf, ExprMethodCall, ExprPath, File,
    ImplItem, ImplItemFn, Item, Stmt,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <contract_file.rs>");
        std::process::exit(1);
    }

    let contract_path = &args[1];

    let issues = analyze_contract(contract_path);

    print_report(issues);
}

fn analyze_contract(file_path: &str) -> Vec<String> {
    let mut issues = Vec::new();

    match parse_contract(file_path) {
        Ok(syntax_tree) => {
            issues.extend(check_slippage_checks(&syntax_tree));
        }
        Err(e) => {
            issues.push(format!("Failed to parse contract: {}", e));
        }
    }

    issues
}

fn print_report(issues: Vec<String>) {
    if issues.is_empty() {
        println!("No vulnerabilities found.");
    } else {
        println!("Potential vulnerabilities detected:\n");
        for issue in issues {
            println!("- {}", issue);
        }
    }
}

fn parse_contract(file_path: &str) -> Result<File, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let syntax_tree = parse_file(&content)?;
    Ok(syntax_tree)
}

fn check_slippage_checks(syntax_tree: &File) -> Vec<String> {
    let mut issues = Vec::new();

    for item in &syntax_tree.items {
        match item {
            Item::Fn(func) => {
                let func_name = func.sig.ident.to_string();

                if is_swap_like_function(&func.block) {
                    let mut swap_positions = Vec::new();
                    let mut slippage_checks = Vec::new();

                    for (index, stmt) in func.block.stmts.iter().enumerate() {
                        find_swap_operations(stmt, &mut swap_positions, index);
                        find_slippage_checks(stmt, &mut slippage_checks, index);
                    }

                    for &swap_pos in &swap_positions {
                        let mut has_slippage_check = false;
                        for &check_pos in &slippage_checks {
                            if check_pos < swap_pos {
                                has_slippage_check = true;
                                break;
                            }
                        }
                        if !has_slippage_check {
                            issues.push(format!(
                                "Function '{}' performs a swap operation without a slippage check.",
                                func_name
                            ));
                            break; // Flag once per function
                        }
                    }
                }
            }
            Item::Impl(item_impl) => {
                for impl_item in &item_impl.items {
                    if let ImplItem::Fn(method) = impl_item {
                        let method_name = method.sig.ident.to_string();

                        if is_swap_like_function(&method.block) {
                            let mut swap_positions = Vec::new();
                            let mut slippage_checks = Vec::new();

                            for (index, stmt) in method.block.stmts.iter().enumerate() {
                                find_swap_operations(stmt, &mut swap_positions, index);
                                find_slippage_checks(stmt, &mut slippage_checks, index);
                            }

                            for &swap_pos in &swap_positions {
                                let mut has_slippage_check = false;
                                for &check_pos in &slippage_checks {
                                    if check_pos < swap_pos {
                                        has_slippage_check = true;
                                        break;
                                    }
                                }
                                if !has_slippage_check {
                                    issues.push(format!(
                                        "Method '{}' performs a swap operation without a slippage check.",
                                        method_name
                                    ));
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    issues
}

// Function to determine if a function is swap-like
fn is_swap_like_function(block: &Block) -> bool {
    block
        .stmts
        .iter()
        .any(|stmt| contains_swap_operation(stmt))
}

// Function to find swap operations and collect their positions
fn find_swap_operations(stmt: &Stmt, positions: &mut Vec<usize>, index: usize) {
    if contains_swap_operation(stmt) {
        positions.push(index);
    }
}

// Function to detect slippage checks and collect their positions
fn find_slippage_checks(stmt: &Stmt, positions: &mut Vec<usize>, index: usize) {
    if is_slippage_check(stmt) {
        positions.push(index);
    }
}

// Function to determine if a statement contains a swap operation
fn contains_swap_operation(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Expr(expr, _) => {
            if is_swap_method_call(expr) {
                true
            } else {
                expr_contains_swap(expr)
            }
        }
        Stmt::Local(local) => {
            if let Some(init) = &local.init {
                expr_contains_swap(&init.expr)
            } else {
                false
            }
        }
        _ => false,
    }
}

fn expr_contains_swap(expr: &Expr) -> bool {
    match expr {
        Expr::Call(_) | Expr::MethodCall(_) => is_swap_method_call(expr),
        Expr::Block(expr_block) => block_contains_swap(&expr_block.block),
        Expr::If(expr_if) => {
            block_contains_swap(&expr_if.then_branch)
                || expr_if
                    .else_branch
                    .as_ref()
                    .map_or(false, |(_, else_expr)| match else_expr.as_ref() {
                        Expr::Block(block) => block_contains_swap(&block.block),
                        _ => expr_contains_swap(else_expr),
                    })
        }
        Expr::Match(expr_match) => {
            expr_match
                .arms
                .iter()
                .any(|arm| expr_contains_swap(&arm.body))
        }
        Expr::While(expr_while) => block_contains_swap(&expr_while.body),
        Expr::ForLoop(expr_for) => block_contains_swap(&expr_for.body),
        Expr::Paren(expr_paren) => expr_contains_swap(&expr_paren.expr),
        Expr::Try(expr_try) => expr_contains_swap(&expr_try.expr),
        Expr::Await(expr_await) => expr_contains_swap(&expr_await.base),
        Expr::Unary(expr_unary) => expr_contains_swap(&expr_unary.expr),
        Expr::Binary(expr_binary) => {
            expr_contains_swap(&expr_binary.left) || expr_contains_swap(&expr_binary.right)
        }
        _ => false,
    }
}

fn block_contains_swap(block: &Block) -> bool {
    block
        .stmts
        .iter()
        .any(|stmt| contains_swap_operation(stmt))
}

// Function to determine if a statement is a slippage check
fn is_slippage_check(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Expr(expr, _) => is_slippage_check_expr(expr),
        Stmt::Local(local) => {
            if let Some(init) = &local.init {
                is_slippage_check_expr(&init.expr)
            } else {
                false
            }
        }
        _ => false,
    }
}

fn is_slippage_check_expr(expr: &Expr) -> bool {
    match expr {
        Expr::If(expr_if) => is_slippage_condition(&expr_if.cond),
        Expr::Block(expr_block) => block_contains_slippage_check(&expr_block.block),
        Expr::Paren(expr_paren) => is_slippage_check_expr(&expr_paren.expr),
        Expr::Match(expr_match) => expr_match
            .arms
            .iter()
            .any(|arm| is_slippage_check_expr(&arm.body)),
        _ => false,
    }
}

fn block_contains_slippage_check(block: &Block) -> bool {
    block
        .stmts
        .iter()
        .any(|stmt| is_slippage_check(stmt))
}

// Function to determine if an expression is a slippage condition
fn is_slippage_condition(expr: &Expr) -> bool {
    match expr {
        Expr::Binary(ExprBinary { left, op, right, .. }) => match op {
            BinOp::Lt(_)
            | BinOp::Le(_)
            | BinOp::Eq(_)
            | BinOp::Ne(_)
            | BinOp::Gt(_)
            | BinOp::Ge(_) => {
                (is_expected_amount_expr(left) && is_actual_amount_expr(right))
                    || (is_expected_amount_expr(right) && is_actual_amount_expr(left))
            }
            _ => false,
        },
        Expr::Paren(expr_paren) => is_slippage_condition(&expr_paren.expr),
        Expr::Unary(expr_unary) => is_slippage_condition(&expr_unary.expr),
        _ => false,
    }
}

// Helper function to identify swap method calls
fn is_swap_method_call(expr: &Expr) -> bool {
    match expr {
        Expr::MethodCall(method_call) => {
            let method_name = method_call.method.to_string().to_lowercase();
            let swap_methods = vec![
                "transfer",
                "transfer_from",
                "swap",
                "deposit",
                "withdraw",
                "exchange",
                "buy",
                "sell",
            ];
            swap_methods.contains(&method_name.as_str())
        }
        Expr::Call(call) => {
            if let Expr::Path(expr_path) = &*call.func {
                let func_name = expr_path
                    .path
                    .segments
                    .last()
                    .unwrap()
                    .ident
                    .to_string()
                    .to_lowercase();
                let swap_functions = vec![
                    "transfer",
                    "transfer_from",
                    "swap",
                    "deposit",
                    "withdraw",
                    "exchange",
                    "buy",
                    "sell",
                ];
                swap_functions.contains(&func_name.as_str())
            } else {
                false
            }
        }
        _ => false,
    }
}

// Helper functions to identify expected and actual amount expressions
fn is_expected_amount_expr(expr: &Expr) -> bool {
    match expr {
        Expr::Path(expr_path) => {
            let ident = expr_path
                .path
                .segments
                .last()
                .unwrap()
                .ident
                .to_string()
                .to_lowercase();
            ident.contains("expected") || ident.contains("min_amount") || ident.contains("min_out")
        }
        _ => false,
    }
}

fn is_actual_amount_expr(expr: &Expr) -> bool {
    match expr {
        Expr::Path(expr_path) => {
            let ident = expr_path
                .path
                .segments
                .last()
                .unwrap()
                .ident
                .to_string()
                .to_lowercase();
            ident.contains("actual")
                || ident.contains("amount_out")
                || ident.contains("received")
        }
        _ => false,
    }
}
