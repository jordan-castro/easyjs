use std::collections::HashMap;

use wasm_encoder::{Instruction, MemArg, ValType};

use crate::parser::ast::Expression;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StrongValType {
    None, // i.e. any
    Int, // i.e. i32
    Float, // i.e. f32
    Bool, // i.e. i32
    NotSupported, // i.e. THROW AN ERROR
    String, // i.e. i32 (pointer to string in memory)
}

pub fn get_param_type_by_string(string: &str) -> StrongValType {
    match string {
        "int" => StrongValType::Int,
        "bool" => StrongValType::Bool,
        "float" => StrongValType::Float,
        "string" => StrongValType::String,
        _ => StrongValType::NotSupported,
    }
}

/// Get a param type by named expression
pub fn get_param_type_by_named_expression(param: Expression) -> StrongValType {
    match param {
        Expression::Type(tk, name) => get_param_type_by_string(&name),
        Expression::IdentifierWithType(tk, _, var_type) => {
            get_param_type_by_named_expression(var_type.as_ref().to_owned())
        }
        _ => StrongValType::NotSupported,
    }
}

/// Get the ValType from a strong.
pub fn get_val_type_from_strong(strong: &StrongValType) -> Option<ValType> {
    match strong {
        StrongValType::Int => Some(ValType::I32),
        StrongValType::Float => Some(ValType::F32),
        StrongValType::Bool => Some(ValType::I32),
        StrongValType::String => Some(ValType::I32),
        // TODO: Implement StrongValType::None
        _ => None,
    }
}

/// Find out if a expression is a Identifier
pub fn expression_is_ident(expr: &Expression) -> bool {
    match expr {
        Expression::Identifier(_, _) => true,
        _ => false
    }
}

/// Parse StrongValType from expression.
/// 
/// !Important: Expression must be a Type.
pub fn parse_strong_from_expression(expr: &Expression) -> StrongValType {
    match expr {
        Expression::Type(_, name) => get_param_type_by_string(name),
        _ => StrongValType::NotSupported
    }
}

/// Get the name of a variable from a ident expression
pub fn get_name_from_ident(ident: &Expression) -> Result<String, &'static str> {  
    match ident {
        Expression::Identifier(_, name) => Ok(name.to_owned()),
        _ => Err("Not an identifier")
    }
}