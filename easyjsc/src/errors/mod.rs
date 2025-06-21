use crate::lexer::token::Token;

fn make_error(token: &Token, error_msg: &str) -> String {
    format!("File: {} at line: {} and col: {}. ERROR {}. Token details: type: {}, literal: {}", token.file_name, token.line_number, token.col_number, error_msg, token.typ, token.literal)
}

pub fn make_native_error(token: &Token, error_msg: &str) -> String {
    make_error(token, format!("Native: {}", error_msg).as_str())
}

pub fn native_can_not_compile_raw_expression(token: &Token) -> String {
    make_native_error(token, "Can not compile raw expression.")
}

pub fn native_can_not_get_value_from_expression(token: &Token) -> String {
    make_native_error(token, "Can not get value from expression.")
}

pub fn native_could_not_parse_function(token: &Token, function_name: &str) -> String {
    make_native_error(token, format!("Could not parse function {}", function_name).as_str())
}

/// Native ERROR: Unsupported Operator: {operator}
pub fn native_unsupported_operator(token: &Token, operator: &str) -> String {
    make_native_error(token, format!("Unsupported operator: {}", operator).as_str())
}

/// Native ERROR: Unsupported operation: {left} {operation} {right}
pub fn native_unsupported_operation(token: &Token, left: &str, operation: &str, right: &str) -> String {
    make_native_error(token, format!("Unsupported operation: {} {} {}", left, operation, right).as_str())
}

/// Native ERROR: Error compiling identifier: {ident}
pub fn native_error_compiling_identifier(token: &Token, identifier: &str) -> String {
    make_native_error(token, format!("Error compiling identifier: {}", identifier).as_str())
}

/// Native ERROR: Unsupported expression as value for global variable
pub fn native_unsupported_expression_as_value_for_global_variable(token: &Token) -> String {
    make_native_error(token, "Unsupported expression as value for global variable")
}

/// Native ERROR: Unsupported statement
pub fn native_unsupported_statement(token: &Token) -> String {
    make_native_error(token, "Unsupported statement")
}

pub fn native_unsupported_index_expression(token: &Token) -> String {
    make_native_error(token, format!("Unsupported index expression for: {} {}", token.typ, token.literal).as_str())
}

/// ERROR Native: Unsupported prefix {prefix}.
pub fn native_unsupported_prefix_expression(token: &Token, prefix: &str) -> String {
    make_native_error(token, format!("Unsupported prefix {}", prefix).as_str())
}

/// ERROR Native: Unsupported builtin call
pub fn native_unsupported_builtin_call(token: &Token) -> String {
    make_native_error(token, "Unsupported builtin call:")
}

/// ERROR Native: if expressions must go within functions.
pub fn native_if_expression_must_go_within_functions