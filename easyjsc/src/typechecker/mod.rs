use wasm_encoder::ValType;

use crate::parser::ast::Expression;

/// A Type Value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StrongValType {
    /// i.e. any
    None, 
    /// i.e. easyjs: Number, native: i32
    Int,
    /// i.e. easyjs: Number, native: f32
    Float,
    /// i.e. easyjs: bool, native: i32
    Bool,
    /// Custom schema type. TODO
    Custom,
    NotSupported, // i.e. THROW AN ERROR
    /// i.e. easyjs: string, native: i32 (pointer to string in memory)
    String,
}

/// String representation of type
/// 
/// This is most useful for Native. I don't see a reason to use this in non native?
pub fn get_string_rep_of_type(strong: &StrongValType) -> String {
    match strong {
        StrongValType::Bool => "bool",
        StrongValType::Float => "float",
        StrongValType::String => "string",
        StrongValType::Int => "int",
        _ => "" // ?
    }.to_string()
}

/// Get the param type for native context.
pub fn get_param_type_by_string(string: &str) -> StrongValType {
    match string {
        "int" => StrongValType::Int,
        "bool" => StrongValType::Bool,
        "float" => StrongValType::Float,
        "string" => StrongValType::String,
        _ => StrongValType::NotSupported,
    }
}

/// Get the param type for easyjs context.
pub fn get_param_type_by_string_ej(string: &str) -> StrongValType {
    let result = get_param_type_by_string(string);
    if result == StrongValType::NotSupported {
        StrongValType::None
    } else {
        result
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
/// 
/// Only works in Native contextx.
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

/// Get the name of a variable from a ident expression
pub fn get_name_from_ident(ident: &Expression) -> Result<String, &'static str> {  
    match ident {
        Expression::Identifier(_, name) => Ok(name.to_owned()),
        _ => Err("Not an identifier")
    }
}
