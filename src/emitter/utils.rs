use std::collections::HashMap;

use wasm_encoder::{Instruction, ValType};

use crate::parser::ast::Expression;

use super::{signatures::TypeRegistry, variables::WasmVariables};

pub enum StrongValType {
    None,
    Some(ValType),
    NotSupported,
}

pub fn get_param_type_by_string(string: String) -> Option<ValType> {
    match string.as_str() {
        "int" => Some(ValType::I32),
        "bool" => Some(ValType::I32),
        "float" => Some(ValType::F32),
        _ => None
    }
}

/// Get a param type by named expression
pub fn get_param_type_by_named_expression(param: Expression) -> StrongValType {
    match param {
        Expression::Type(tk, name) => match name.as_ref() {
            "int" => StrongValType::Some(ValType::I32),
            "float" => StrongValType::Some(ValType::F32),
            "bool" => StrongValType::Some(ValType::I32),
            _ => StrongValType::NotSupported,
        },
        Expression::IdentifierWithType(tk, _, var_type) => {
            get_param_type_by_named_expression(var_type.as_ref().to_owned())
        }
        _ => StrongValType::NotSupported,
    }
}

/// Make an instruction for a value.
pub fn make_instruction_for_value(value: &Expression) -> Instruction {
    match value {
        Expression::IntegerLiteral(_, v) => Instruction::I32Const(*v as i32),
        Expression::FloatLiteral(_, v) => Instruction::F32Const(*v as f32),
        _ => Instruction::Nop,
    }
}

/// Infer variable type
pub fn infer_variable_type(
    value: &Expression,
    scoped_variables: &WasmVariables,
    scoped_type_registry: &TypeRegistry,
) -> StrongValType {
    let res = match value {
        Expression::IntegerLiteral(_, _) => ValType::I32,
        Expression::FloatLiteral(_, _) => ValType::F32,
        Expression::Identifier(_, name) => {
            // possible varaible?
            let var = scoped_variables.get_variable_by_name(name);
            if let Some(var) = var {
                var.ty
            } else {
                // check scoped_functions
                let func = scoped_type_registry.get_return_type_of(name.to_owned());
                if let Some(func) = func {
                    func
                } else {
                    return StrongValType::None;
                }
            }
        }
        Expression::CallExpression(_, name, _) => {
            return infer_variable_type(name.as_ref(), scoped_variables, scoped_type_registry);
        }
        Expression::Boolean(_, _) => {
            ValType::I32
        }
        _ => {
            return StrongValType::None;
            // unimplemented!();
        }
    };

    StrongValType::Some(res)
}
