use wasm_encoder::ValType;

use crate::{lexer::token::Token, parser::ast::{self, Expression}};

/// = Native::i32
pub const I32_TYPE_IDX: i32 = 0;
/// = Native::f32
pub const F32_TYPE_IDX: i32 = 1;
/// = Native::String
pub const STRING_TYPE_IDX: i32 = 2;
/// = Native::Array
pub const ARRAY_TYPE_IDX: i32 = 3;

/// A Type Value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EJType {
    /// void
    None, 
    /// js: Number, wasm: i32
    Int,
    /// js: Number, wasm: f32
    Float,
    /// js: bool, wasm: i32
    Bool,
    /// Custom schema type. TODO
    Custom,
    NotSupported, // i.e. THROW AN ERROR
    /// js: string, wasm: i32 (pointer to string in memory)
    String,
    /// js: Array, wasm: i32 (pointer to array in memory)
    Array,
    /// js: dynamic, wasm: i32. This is also the default type.
    Dynamic,
}

/// String representation of type
/// 
/// This is most useful for Native. I don't see a reason to use this in non native?
pub fn get_string_rep_of_type(strong: &EJType) -> String {
    match strong {
        EJType::Bool => "bool",
        EJType::Float => "float",
        EJType::String => "string",
        EJType::Int => "int",
        EJType::Array => "array",
        _ => "" // ?
    }.to_string()
}

/// Get the param type for native context.
pub fn get_param_type_by_string(string: &str) -> EJType {
    match string {
        "int" => EJType::Int,
        "bool" => EJType::Bool,
        "float" => EJType::Float,
        "string" => EJType::String,
        "array" => EJType::Array,
        "" => EJType::None,
        "none" => EJType::None,
        _ => EJType::NotSupported,
    }
}

/// Get the param type for easyjs context.
pub fn get_param_type_by_string_ej(string: &str) -> EJType {
    let result = get_param_type_by_string(string);
    if result == EJType::NotSupported {
        EJType::None
    } else {
        result
    }
}

/// Get a param type by named expression
pub fn get_param_type_by_named_expression(param: Expression) -> EJType {
    match param {
        Expression::Type(tk, name) => get_param_type_by_string(&name),
        Expression::IdentifierWithType(tk, _, var_type) => {
            get_param_type_by_named_expression(var_type.as_ref().to_owned())
        }
        _ => EJType::NotSupported,
    }
}

/// Get the ValType from a strong.
/// 
/// Only works in Native contextx.
pub fn get_val_type_from_strong(strong: &EJType) -> Option<ValType> {
    match strong {
        EJType::Int => Some(ValType::I32),
        EJType::Float => Some(ValType::F32),
        EJType::Bool => Some(ValType::I32),
        EJType::String => Some(ValType::I32),
        EJType::Array => Some(ValType::I32),
        EJType::None => Some(ValType::I32),
        // TODO: Implement StrongValType::None
        _ => None,
    }
}

/// Get the name of a variable from a ident expression
pub fn get_name_from_ident(ident: &Expression) -> Result<String, &'static str> {  
    match ident {
        Expression::Identifier(_, name) => Ok(name.to_owned()),
        _ => Err("Not an identifier")
    }
}
