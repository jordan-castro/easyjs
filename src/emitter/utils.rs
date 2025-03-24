use std::collections::HashMap;

use wasm_encoder::{Instruction, MemArg, ValType};

use crate::parser::ast::Expression;

use super::{
    instruction_generator::{call, get_local, set_local},
    signatures::TypeRegistry,
    variables::WasmVariables,
};

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

/// Infer variable type
pub fn infer_variable_type(
    value: &Expression,
    scoped_variables: &WasmVariables,
    scoped_type_registry: &TypeRegistry,
    global_variables: Option<&WasmVariables>,
) -> StrongValType {
    match value {
        Expression::IntegerLiteral(_, _) => StrongValType::Int,
        Expression::FloatLiteral(_, _) => StrongValType::Float,
        Expression::StringLiteral(_, _) => StrongValType::String,
        Expression::Identifier(_, name) => {
            // possible varaible?
            let var = scoped_variables.get_variable_by_name(name);
            if let Some(var) = var {
                var.ty
            } else {
                // check scoped_functions
                let func = scoped_type_registry.get_strong_return_type_of(name.to_owned());
                if let Some(func) = func {
                    func
                } else {
                    // check global too
                    if global_variables.is_none() {
                        StrongValType::None
                    } else {
                        let var = global_variables.unwrap().get_variable_by_name(name);
                        if let Some(var) = var {
                            var.ty
                        } else {
                            StrongValType::None
                        }
                    }
                }
            }
        }
        Expression::CallExpression(_, name, _) => {
            return infer_variable_type(name.as_ref(), scoped_variables, scoped_type_registry, global_variables);
        }
        Expression::Boolean(_, _) => StrongValType::Bool,
        _ => StrongValType::None,
    }
}

/// Get the ValType from a strong.
pub fn get_val_type_from_strong(strong: &StrongValType) -> Option<ValType> {
    match strong {
        StrongValType::Int => Some(ValType::I32),
        StrongValType::Float => Some(ValType::F32),
        StrongValType::Bool => Some(ValType::I32),
        StrongValType::String => Some(ValType::I32),
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