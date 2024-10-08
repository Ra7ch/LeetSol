use std::env;
use std::fs;
use std::collections::HashSet;
use syn::{
    parse_file, Expr, ExprCall, ExprIf, ExprBinary, File,
    ImplItem, ImplItemFn, Item, Member, Stmt, BinOp, Block, Pat,
};

struct AccessControlPatterns {
    functions: Vec<String>,
    methods: Vec<String>,
    identifiers: Vec<String>,
}

impl Default for AccessControlPatterns {
    fn default() -> Self {
        AccessControlPatterns {
            functions: vec![
                "assert_eq".to_string(),
                "assert_ne".to_string(),
                "require".to_string(),
                "assert".to_string(),
                "require_keys_unequal".to_string(),
                "require_signer".to_string(),
                "check_authority".to_string(),
            ],
            methods: vec![
                "is_signer".to_string(),
                "has_role".to_string(),
                "has_signer".to_string(),
                "is_authorized".to_string(),
            ],
            identifiers: vec![
                "owner".to_string(),
                "authority".to_string(),
                "admin".to_string(),
            ],
        }
    }
}

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
                            break;
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

fn is_swap_like_function(block: &Block) -> bool {
    block
        .stmts
        .iter()
        .any(|stmt| contains_swap_operation(stmt))
}

fn find_swap_operations(stmt: &Stmt, positions: &mut Vec<usize>, index: usize) {
    if contains_swap_operation(stmt) {
        positions.push(index);
    }
}

fn find_slippage_checks(stmt: &Stmt, positions: &mut Vec<usize>, index: usize) {
    if is_slippage_check(stmt) {
        positions.push(index);
    }
}

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
        Expr::If(expr_if) => {
            is_slippage_condition(&expr_if.cond) || block_contains_slippage_check(&expr_if.then_branch)
        }
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
                "send",
                "receive",
                "trade",
                "mint",
                "burn",
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
                    "send",
                    "receive",
                    "trade",
                    "mint",
                    "burn",
                ];
                swap_functions.contains(&func_name.as_str())
            } else {
                false
            }
        }
        _ => false,
    }
}

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
            ident.contains("expected")
                || ident.contains("min_amount")
                || ident.contains("min_out")
                || ident.contains("minimum")
                || ident.contains("limit")
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
                || ident.contains("output")
                || ident.contains("result")
        }
        _ => false,
    }
}

fn expr_modifies_state(expr: &Expr, state_variables: &HashSet<String>) -> bool {
    match expr {
        Expr::Assign(expr_assign) => {
            expr_is_state_variable(&expr_assign.left, state_variables)
        }
        Expr::MethodCall(method_call) => {
            let method_name = method_call.method.to_string();
            if method_name == "serialize" || method_name == "try_to_vec" {
                let receiver = &method_call.receiver;
                if expr_is_state_variable(receiver, state_variables) {
                    return true;
                }
            }
            method_call
                .args
                .iter()
                .any(|arg| expr_modifies_state(arg, state_variables))
        }
        Expr::Block(expr_block) => expr_block
            .block
            .stmts
            .iter()
            .any(|stmt| stmt_modifies_state(stmt, state_variables)),
        Expr::If(expr_if) => {
            expr_modifies_state(&expr_if.cond, state_variables)
                || expr_if
                    .then_branch
                    .stmts
                    .iter()
                    .any(|stmt| stmt_modifies_state(stmt, state_variables))
                || expr_if
                    .else_branch
                    .as_ref()
                    .map_or(false, |(_, else_expr)| {
                        expr_modifies_state(else_expr, state_variables)
                    })
        }
        _ => false,
    }
}

fn check_rent_exemption(syntax_tree: &File) -> Vec<String> {
    let mut issues = Vec::new();

    for item in &syntax_tree.items {
        match item {
            Item::Fn(func) => {
                let func_name = func.sig.ident.to_string();
                if creates_new_account(&func.block) {
                    if !has_rent_exemption_check(&func.block) {
                        issues.push(format!(
                            "Function '{}' creates a new account without checking for rent exemption.",
                            func_name
                        ));
                    }
                }
            }
            Item::Impl(item_impl) => {
                for impl_item in &item_impl.items {
                    if let ImplItem::Fn(method) = impl_item {
                        let method_name = method.sig.ident.to_string();
                        if creates_new_account(&method.block) {
                            if !has_rent_exemption_check(&method.block) {
                                issues.push(format!(
                                    "Method '{}' creates a new account without checking for rent exemption.",
                                    method_name
                                ));
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

fn creates_new_account(block: &Block) -> bool {
    block.stmts.iter().any(|stmt| contains_account_creation(stmt))
}

fn has_rent_exemption_check(block: &Block) -> bool {
    block
        .stmts
        .iter()
        .any(|stmt| contains_rent_exemption_check(stmt))
}

fn contains_account_creation(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Expr(expr, _) => expr_contains_account_creation(expr),
        Stmt::Local(local) => {
            if let Some(init) = &local.init {
                expr_contains_account_creation(&init.expr)
            } else {
                false
            }
        }
        _ => false,
    }
}

fn contains_rent_exemption_check(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Expr(expr, _) => expr_contains_rent_exemption_check(expr),
        Stmt::Local(local) => {
            if let Some(init) = &local.init {
                expr_contains_rent_exemption_check(&init.expr)
            } else {
                false
            }
        }
        _ => false,
    }
}

// here im recursively checking if an expression contains account creation with common methods in contracts args also
fn expr_contains_account_creation(expr: &Expr) -> bool {
    match expr {
        Expr::MethodCall(method_call) => {
            let method_name = method_call.method.to_string().to_lowercase();
            let account_creation_methods = vec![
                "create_account",
                "create_account_with_seed",
                "create_program_account",
                "new_account",
                "new_account_with_seed",
                "assign",
                "allocate",
            ];
            if account_creation_methods.contains(&method_name.as_str()) {
                return true;
            }
            method_call
                .args
                .iter()
                .any(|arg| expr_contains_account_creation(arg))
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
                let account_creation_functions = vec![
                    "create_account",
                    "create_account_with_seed",
                    "create_program_account",
                    "new_account",
                    "new_account_with_seed",
                    "assign",
                    "allocate",
                ];
                if account_creation_functions.contains(&func_name.as_str()) {
                    return true;
                }
            }
            call.args
                .iter()
                .any(|arg| expr_contains_account_creation(arg))
        }
        Expr::Block(expr_block) => {
            expr_block
                .block
                .stmts
                .iter()
                .any(|stmt| contains_account_creation(stmt))
        }
        Expr::If(expr_if) => {
            expr_contains_account_creation(&expr_if.cond)
                || expr_if
                    .then_branch
                    .stmts
                    .iter()
                    .any(|stmt| contains_account_creation(stmt))
                || expr_if
                    .else_branch
                    .as_ref()
                    .map_or(false, |(_, else_expr)| expr_contains_account_creation(else_expr))
        }
        Expr::Match(expr_match) => {
            expr_contains_account_creation(&expr_match.expr)
                || expr_match
                    .arms
                    .iter()
                    .any(|arm| expr_contains_account_creation(&arm.body))
        }
        Expr::Await(expr_await) => expr_contains_account_creation(&expr_await.base),
        Expr::Try(expr_try) => expr_contains_account_creation(&expr_try.expr),
        Expr::Paren(expr_paren) => expr_contains_account_creation(&expr_paren.expr),
        Expr::Unary(expr_unary) => expr_contains_account_creation(&expr_unary.expr),
        Expr::Binary(expr_binary) => {
            expr_contains_account_creation(&expr_binary.left)
                || expr_contains_account_creation(&expr_binary.right)
        }
        _ => false,
    }
}

// saame as expression
fn expr_contains_rent_exemption_check(expr: &Expr) -> bool {
    match expr {
        Expr::MethodCall(method_call) => {
            if let Expr::Path(expr_path) = &*method_call.receiver {
                let receiver_name = expr_path
                    .path
                    .segments
                    .last()
                    .unwrap()
                    .ident
                    .to_string()
                    .to_lowercase();
                let method_name = method_call.method.to_string().to_lowercase();

                if receiver_name == "rent" && method_name == "is_exempt" {
                    return true;
                }
            }
            method_call
                .args
                .iter()
                .any(|arg| expr_contains_rent_exemption_check(arg))
        }
        Expr::Call(call) => {
            call.args
                .iter()
                .any(|arg| expr_contains_rent_exemption_check(arg))
        }
        Expr::If(expr_if) => {
            expr_contains_rent_exemption_check(&expr_if.cond)
                || expr_if
                    .then_branch
                    .stmts
                    .iter()
                    .any(|stmt| contains_rent_exemption_check(stmt))
                || expr_if
                    .else_branch
                    .as_ref()
                    .map_or(false, |(_, else_expr)| expr_contains_rent_exemption_check(else_expr))
        }
        Expr::Block(expr_block) => {
            expr_block
                .block
                .stmts
                .iter()
                .any(|stmt| contains_rent_exemption_check(stmt))
        }
        Expr::Match(expr_match) => {
            expr_contains_rent_exemption_check(&expr_match.expr)
                || expr_match
                    .arms
                    .iter()
                    .any(|arm| expr_contains_rent_exemption_check(&arm.body))
        }
        Expr::Await(expr_await) => expr_contains_rent_exemption_check(&expr_await.base),
        Expr::Try(expr_try) => expr_contains_rent_exemption_check(&expr_try.expr),
        Expr::Paren(expr_paren) => expr_contains_rent_exemption_check(&expr_paren.expr),
        Expr::Unary(expr_unary) => expr_contains_rent_exemption_check(&expr_unary.expr),
        Expr::Binary(expr_binary) => {
            expr_contains_rent_exemption_check(&expr_binary.left)
                || expr_contains_rent_exemption_check(&expr_binary.right)
        }
        _ => false,
    }
}

fn analyze_contract(file_path: &str) -> Vec<String> {
    let mut issues = Vec::new();

    match parse_contract(file_path) {
        Ok(syntax_tree) => {
            issues.extend(check_access_control(&syntax_tree));
            issues.extend(check_account_ownership(&syntax_tree));
            issues.extend(check_slippage_checks(&syntax_tree)); 
            issues.extend(check_rent_exemption(&syntax_tree));
        }
        Err(e) => {
            issues.push(format!("Failed to parse contract: {}", e));
        }
    }

    issues
}

fn check_access_control(syntax_tree: &File) -> Vec<String> {
    let mut issues = Vec::new();
    let patterns = AccessControlPatterns::default();

    for item in &syntax_tree.items {
        match item {
            Item::Fn(func) => {
                let func_name = func.sig.ident.to_string();
                if modifies_state(func) && !has_access_control_checks(func, &patterns) {
                    issues.push(format!(
                        "Function '{}' may lack access control.",
                        func_name
                    ));
                }
            }
            Item::Impl(item_impl) => {
                for impl_item in &item_impl.items {
                    if let ImplItem::Fn(method) = impl_item {
                        let method_name = method.sig.ident.to_string();

                        if modifies_state(method) && !has_access_control_checks(method, &patterns) {
                            issues.push(format!(
                                "Method '{}' may lack access control.",
                                method_name
                            ));
                        }
                    }
                }
            }
            _ => {}
        }
    }

    issues
}

trait HasBlock {
    fn block(&self) -> &syn::Block;
}

impl HasBlock for syn::ItemFn {
    fn block(&self) -> &syn::Block {
        &self.block
    }
}

impl HasBlock for ImplItemFn {
    fn block(&self) -> &syn::Block {
        &self.block
    }
}

fn modifies_state(func: &impl HasBlock) -> bool {
    let mut state_variables = HashSet::new();
    for stmt in &func.block().stmts {
        collect_state_variables_stmt(stmt, &mut state_variables);
    }

    func.block()
        .stmts
        .iter()
        .any(|stmt| stmt_modifies_state(stmt, &state_variables))
}

fn collect_state_variables_stmt(stmt: &Stmt, state_variables: &mut HashSet<String>) {
    match stmt {
        Stmt::Local(local) => {
            if let Some(init) = &local.init {
                if let Pat::Ident(pat_ident) = &local.pat {
                    let mut expr = &*init.expr;
                    while let &Expr::Try(ref expr_try) = expr {
                        expr = &*expr_try.expr;
                    }
                    if expr_is_deserialization_of_account_data(expr) {
                        state_variables.insert(pat_ident.ident.to_string());
                    }
                }
            }
        }
        Stmt::Expr(expr, _) => {
            collect_state_variables_expr(expr, state_variables);
        }
        _ => {}
    }
}

fn collect_state_variables_expr(expr: &Expr, state_variables: &mut HashSet<String>) {
    match expr {
        Expr::Block(expr_block) => {
            for stmt in &expr_block.block.stmts {
                collect_state_variables_stmt(stmt, state_variables);
            }
        }
        Expr::If(expr_if) => {
            collect_state_variables_expr(&expr_if.cond, state_variables);
            for stmt in &expr_if.then_branch.stmts {
                collect_state_variables_stmt(stmt, state_variables);
            }
            if let Some((_, else_expr)) = &expr_if.else_branch {
                collect_state_variables_expr(else_expr, state_variables);
            }
        }
        Expr::Match(expr_match) => {
            collect_state_variables_expr(&expr_match.expr, state_variables);
            for arm in &expr_match.arms {
                collect_state_variables_expr(&arm.body, state_variables);
            }
        }
        Expr::Call(_) | Expr::MethodCall(_) => {
            if expr_is_deserialization_of_account_data(expr) {
                if let Some(ident) = extract_assigned_ident(expr) {
                    state_variables.insert(ident);
                }
            }
        }
        Expr::Assign(expr_assign) => {
            collect_state_variables_expr(&expr_assign.right, state_variables);
            if expr_is_deserialization_of_account_data(&expr_assign.right) {
                if let Some(ident) = extract_ident_from_expr(&expr_assign.left) {
                    state_variables.insert(ident);
                }
            }
        }
        _ => {}
    }
}

fn extract_assigned_ident(expr: &Expr) -> Option<String> {
    if let Expr::Assign(expr_assign) = expr {
        extract_ident_from_expr(&expr_assign.left)
    } else {
        None
    }
}

fn extract_ident_from_expr(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Path(expr_path) => {
            expr_path.path.get_ident().map(|ident| ident.to_string())
        }
        Expr::Field(expr_field) => {
            extract_ident_from_expr(&expr_field.base)
        }
        _ => None,
    }
}

fn expr_is_deserialization_of_account_data(expr: &Expr) -> bool {
    match expr {
        Expr::Call(call) => {
            if let Expr::Path(expr_path) = &*call.func {
                let func_name = expr_path.path.segments.last().unwrap().ident.to_string();
                let deserialization_methods = vec!["try_from_slice", "unpack", "deserialize"];
                if deserialization_methods.contains(&func_name.as_str()) {
                    if let Some(arg) = call.args.first() {
                        let mut arg = arg;
                        if let &Expr::Reference(ref expr_ref) = arg {
                            arg = &*expr_ref.expr;
                        }
                        return expr_is_account_data_borrow(arg);
                    }
                }
            }
            false
        }
        Expr::MethodCall(method_call) => {
            let method_name = method_call.method.to_string();
            let deserialization_methods = vec!["try_from_slice", "unpack", "deserialize"];
            if deserialization_methods.contains(&method_name.as_str()) {
                let mut arg = &*method_call.receiver;
                if let &Expr::Reference(ref expr_ref) = arg {
                    arg = &*expr_ref.expr;
                }
                if expr_is_account_data_borrow(arg) {
                    return true;
                }
            }
            false
        }
        Expr::Try(expr_try) => {
            expr_is_deserialization_of_account_data(&*expr_try.expr)
        }
        _ => false,
    }
}

fn expr_is_account_data_borrow(expr: &Expr) -> bool {
    match expr {
        Expr::MethodCall(method_call) => {
            let method_name = method_call.method.to_string();
            if method_name == "borrow" || method_name == "borrow_mut" {
                if let Expr::Field(expr_field) = &*method_call.receiver {
                    if let Expr::Path(expr_path) = &*expr_field.base {
                        let ident = expr_path.path.segments.last().unwrap().ident.to_string();
                        let field = get_member_name(&expr_field.member);
                        return ident == "account_info" && field == "data";
                    }
                }
            }
            false
        }
        Expr::Reference(expr_ref) => {
            expr_is_account_data_borrow(&*expr_ref.expr)
        }
        _ => false,
    }
}

fn expr_is_state_variable(expr: &Expr, state_variables: &HashSet<String>) -> bool {
    match expr {
        Expr::Path(expr_path) => {
            let ident = expr_path.path.segments.last().unwrap().ident.to_string();
            state_variables.contains(&ident)
        }
        Expr::Field(expr_field) => {
            if let Expr::Path(expr_path) = &*expr_field.base {
                let ident = expr_path.path.segments.last().unwrap().ident.to_string();
                state_variables.contains(&ident)
            } else {
                false
            }
        }
        _ => false,
    }
}

fn stmt_modifies_state_simple(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Expr(expr, _) => expr_modifies_state_simple(expr),
        Stmt::Local(local) => {
            if let Some(init) = &local.init {
                expr_modifies_state_simple(&init.expr)
            } else {
                false
            }
        }
        _ => false,
    }
}

fn expr_modifies_state_simple(expr: &Expr) -> bool {
    match expr {
        Expr::Assign(_) => true,
        Expr::Binary(expr_binary) => {
            matches!(expr_binary.op,
                BinOp::AddAssign(_)
                | BinOp::SubAssign(_)
                | BinOp::MulAssign(_)
                | BinOp::DivAssign(_)
                | BinOp::RemAssign(_)
                | BinOp::BitXorAssign(_)
                | BinOp::BitAndAssign(_)
                | BinOp::BitOrAssign(_)
                | BinOp::ShlAssign(_)
                | BinOp::ShrAssign(_)
            )
        }
        Expr::MethodCall(method_call) => {
            let method_name = method_call.method.to_string();
            if method_name == "serialize" || method_name == "try_to_vec" {
                return true;
            }
            method_call.args.iter().any(expr_modifies_state_simple)
        }
        Expr::Block(expr_block) => expr_block
            .block
            .stmts
            .iter()
            .any(stmt_modifies_state_simple),
        Expr::If(expr_if) => {
            expr_modifies_state_simple(&expr_if.cond)
                || expr_if
                    .then_branch
                    .stmts
                    .iter()
                    .any(|stmt| stmt_modifies_state_simple(stmt))
                || expr_if
                    .else_branch
                    .as_ref()
                    .map_or(false, |(_, else_expr)| {
                        expr_modifies_state_simple(else_expr)
                    })
        }
        _ => false,
    }
}

fn has_access_control_checks(func: &impl HasBlock, patterns: &AccessControlPatterns) -> bool {
    analyze_block_for_access_control(func.block(), patterns)
}

fn analyze_block_for_access_control(block: &Block, patterns: &AccessControlPatterns) -> bool {
    let mut access_control_present = false;

    for stmt in &block.stmts {
        if check_stmt_for_access_control(stmt, patterns) {
            access_control_present = true;
        }

        if stmt_modifies_state_simple(stmt) {
            if !access_control_present {
                return false;
            }
            access_control_present = false;
        }

        if let Stmt::Expr(expr, _) = stmt {
            if let Expr::If(expr_if) = expr {
                let branches_have_access_control = analyze_if_expr(expr_if, patterns);
                if !branches_have_access_control {
                    return false;
                }
            }
        }
    }

    true
}

fn analyze_if_expr(expr_if: &ExprIf, patterns: &AccessControlPatterns) -> bool {
    let then_has_access_control = analyze_block_for_access_control(&expr_if.then_branch, patterns);

    let else_has_access_control = if let Some((_, else_expr)) = &expr_if.else_branch {
        match &**else_expr {
            Expr::Block(else_block) => {
                analyze_block_for_access_control(&else_block.block, patterns)
            }
            Expr::If(nested_if_expr) => analyze_if_expr(nested_if_expr, patterns),
            _ => true,
        }
    } else {
        true
    };

    then_has_access_control && else_has_access_control
}

fn check_stmt_for_access_control(stmt: &Stmt, patterns: &AccessControlPatterns) -> bool {
    match stmt {
        Stmt::Expr(expr, _) => check_expr_for_access_control(expr, patterns),
        Stmt::Local(local) => {
            if let Some(init) = &local.init {
                check_expr_for_access_control(&init.expr, patterns)
            } else {
                false
            }
        }
        _ => false,
    }
}

fn stmt_modifies_state(stmt: &Stmt, state_variables: &HashSet<String>) -> bool {
    match stmt {
        Stmt::Expr(expr, _) => expr_modifies_state(expr, state_variables),
        Stmt::Local(local) => {
            if let Some(init) = &local.init {
                expr_modifies_state(&init.expr, state_variables)
            } else {
                false
            }
        }
        _ => false,
    }
}

fn check_expr_for_access_control(expr: &Expr, patterns: &AccessControlPatterns) -> bool {
    match expr {
        Expr::Call(expr_call) => {
            if let Expr::Path(expr_path) = &*expr_call.func {
                let func_name = expr_path.path.segments.last().unwrap().ident.to_string();
                if patterns.functions.contains(&func_name) {
                    return true;
                }
            }
            expr_call.args.iter().any(|arg| check_expr_for_access_control(arg, patterns))
        }
        Expr::MethodCall(method_call) => {
            let method_name = method_call.method.to_string();
            if patterns.methods.contains(&method_name) {
                return true;
            }
            check_expr_for_access_control(&method_call.receiver, patterns)
                || method_call.args.iter().any(|arg| check_expr_for_access_control(arg, patterns))
        }
        Expr::Field(expr_field) => {
            let field_name = get_member_name(&expr_field.member);
            if patterns.identifiers.contains(&field_name) {
                return true;
            }
            check_expr_for_access_control(&expr_field.base, patterns)
        }
        Expr::If(expr_if) => {
            if check_condition_for_access_control(&expr_if.cond, patterns) {
                return true;
            }
            check_block_for_access_control(&expr_if.then_branch, patterns)
                || expr_if
                    .else_branch
                    .as_ref()
                    .map_or(false, |(_, else_expr)| {
                        check_expr_for_access_control(else_expr, patterns)
                    })
        }
        Expr::Binary(expr_binary) => {
            if check_condition_for_access_control(expr, patterns) {
                return true;
            }
            check_expr_for_access_control(&expr_binary.left, patterns)
                || check_expr_for_access_control(&expr_binary.right, patterns)
        }
        Expr::Unary(expr_unary) => check_expr_for_access_control(&expr_unary.expr, patterns),
        Expr::Paren(expr_paren) => check_expr_for_access_control(&expr_paren.expr, patterns),
        _ => false,
    }
}

fn check_condition_for_access_control(cond: &Expr, patterns: &AccessControlPatterns) -> bool {
    match cond {
        Expr::Binary(expr_binary) => {
            check_expr_for_access_control(&expr_binary.left, patterns)
                || check_expr_for_access_control(&expr_binary.right, patterns)
        }
        Expr::Unary(expr_unary) => {
            check_expr_for_access_control(&expr_unary.expr, patterns)
        }
        Expr::MethodCall(method_call) => {
            let method_name = method_call.method.to_string();
            patterns.methods.contains(&method_name)
        }
        Expr::Path(expr_path) => {
            let ident = expr_path.path.segments.last().unwrap().ident.to_string();
            patterns.identifiers.contains(&ident)
        }
        _ => false,
    }
}

fn check_block_for_access_control(block: &Block, patterns: &AccessControlPatterns) -> bool {
    for stmt in &block.stmts {
        if check_stmt_for_access_control(stmt, patterns) {
            return true;
        }
    }
    false
}

// fn get_member_name(member: &Member) -> String {
//     match member {
//         Member::Named(ident) => ident.to_string(),
//         Member::Unnamed(index) => index.index.to_string(),
//     }
// }

fn check_account_ownership(syntax_tree: &File) -> Vec<String> {
    let mut issues = Vec::new();

    for item in &syntax_tree.items {
        match item {
            Item::Fn(func) => {
                let func_name = func.sig.ident.to_string();
                let mut deserialization_positions = Vec::new();
                let mut ownership_checks = Vec::new();

                for (index, stmt) in func.block.stmts.iter().enumerate() {
                    find_deserialization_calls(stmt, &mut deserialization_positions, index);
                    find_ownership_checks(stmt, &mut ownership_checks, index);
                }

                for deserial_pos in deserialization_positions {
                    let mut has_ownership_check = false;
                    for &check_pos in &ownership_checks {
                        if check_pos < deserial_pos {
                            has_ownership_check = true;
                            break;
                        }
                    }
                    if !has_ownership_check {
                        issues.push(format!(
                            "Function '{}' deserializes an account without checking ownership.",
                            func_name
                        ));
                    }
                }
            }
            _ => {}
        }
    }

    issues
}

fn find_deserialization_calls(stmt: &Stmt, positions: &mut Vec<usize>, index: usize) {
    match stmt {
        Stmt::Expr(expr, _) => {
            if is_deserialization_call(expr) {
                positions.push(index);
            }
        }
        Stmt::Local(local) => {
            if let Some(local_init) = &local.init {
                let init_expr = &local_init.expr;
                if is_deserialization_call(init_expr) {
                    positions.push(index);
                }
            }
        }
        _ => {}
    }
}

fn find_ownership_checks(stmt: &Stmt, positions: &mut Vec<usize>, index: usize) {
    if is_ownership_check(stmt) {
        positions.push(index);
    }
}


fn is_deserialization_call(expr: &Expr) -> bool {
    match expr {
        Expr::MethodCall(method_call) => {
            let method_name = method_call.method.to_string();
            let deserialization_methods = vec!["try_from_slice", "unpack", "deserialize"];
            deserialization_methods.contains(&method_name.as_str())
        }
        Expr::Call(ExprCall { func, .. }) => {
            if let Expr::Path(expr_path) = func.as_ref() {
                let segments = &expr_path.path.segments;
                if let Some(last_segment) = segments.last() {
                    let func_name = last_segment.ident.to_string();
                    let deserialization_methods = vec!["try_from_slice", "unpack", "deserialize"];
                    deserialization_methods.contains(&func_name.as_str())
                } else {
                    false
                }
            } else {
                false
            }
        }
        Expr::Try(expr_try) => {
            is_deserialization_call(&expr_try.expr)
        }
        _ => false,
    }
}
// fn is_ownership_check(expr: &Expr) -> bool {
//     if let Expr::If(expr_if) = expr {
//         if let Expr::Binary(ExprBinary { left, op, right, .. }) = &*expr_if.cond {
//             if matches!(op, BinOp::Ne(_) | BinOp::Eq(_)) {
//                 if (is_account_owner_expr(left) && is_program_id_expr(right))
//                     || (is_account_owner_expr(right) && is_program_id_expr(left))
//                 {
//                     return true;
//                 }
//             }
//         }
//     }
//     false
// }


fn is_ownership_check(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Expr(expr, _) => is_ownership_check_expr(expr),
        Stmt::Local(local) => {
            if let Some(init) = &local.init {
                is_ownership_check_expr(&init.expr)
            } else {
                false
            }
        }
        _ => false,
    }
}

fn is_ownership_check_expr(expr: &Expr) -> bool {
    match expr {
        Expr::If(expr_if) => is_ownership_check_condition(&expr_if.cond),
        Expr::Match(expr_match) => {
            expr_match.arms.iter().any(|arm| is_ownership_check_expr(&arm.body))
        }
        Expr::Block(expr_block) => {
            expr_block.block.stmts.iter().any(|stmt| is_ownership_check(stmt))
        }
        _ => false,
    }
}

fn is_ownership_check_condition(expr: &Expr) -> bool {
    match expr {
        // binary ex `==` or `!=` also check for possible orders lol
        Expr::Binary(ExprBinary { left, op, right, .. }) => {
            if matches!(op, BinOp::Eq(_) | BinOp::Ne(_)) {
                (is_account_owner_expr(left) && is_program_id_expr(right))
                    || (is_account_owner_expr(right) && is_program_id_expr(left))
            } else {
                false
            }
        }
        Expr::MethodCall(expr_method) => {
            let method_name = expr_method.method.to_string().to_lowercase();
            method_name == "is_signer" || method_name == "is_writable"
        }
        Expr::Unary(expr_unary) => is_ownership_check_condition(&expr_unary.expr),
        Expr::Paren(expr_paren) => is_ownership_check_condition(&expr_paren.expr),
        _ => false,
    }
}

fn is_account_owner_expr(expr: &Expr) -> bool {
    match expr {
        Expr::Field(expr_field) => {
            if let Expr::Path(expr_path) = &*expr_field.base {
                let base_ident = expr_path.path.segments.last().unwrap().ident.to_string().to_lowercase();
                let field_ident = get_member_name(&expr_field.member).to_lowercase();
                
                (base_ident.contains("account") || base_ident.contains("info"))
                    && (field_ident == "owner" || field_ident == "key")
            } else {
                false
            }
        }
        Expr::MethodCall(expr_method) => {
            let method_name = expr_method.method.to_string().to_lowercase();
            method_name == "owner" || method_name == "key"
        }
        _ => false,
    }
}

fn is_program_id_expr(expr: &Expr) -> bool {
    match expr {
        Expr::Path(expr_path) => {
            let ident = expr_path.path.segments.last().unwrap().ident.to_string().to_lowercase();
            ident.contains("program_id") || ident.ends_with("_id") || ident == "id"
        }
        Expr::Field(expr_field) => {
            let field_ident = get_member_name(&expr_field.member).to_lowercase();
            field_ident.contains("program_id") || field_ident.ends_with("_id") || field_ident == "id"
        }
        _ => false,
    }
}

fn get_member_name(member: &Member) -> String {
    match member {
        Member::Named(ident) => ident.to_string(),
        Member::Unnamed(index) => index.index.to_string(),
    }
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
