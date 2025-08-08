use std::collections::HashMap;

use wasm_encoder::{Instruction, MemArg, ValType};

use crate::parser::ast::Expression;



/// Find out if a expression is a Identifier
pub fn expression_is_ident(expr: &Expression) -> bool {
    match expr {
        Expression::Identifier(_, _) => true,
        _ => false
    }
}

