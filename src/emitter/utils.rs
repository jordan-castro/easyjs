use std::collections::HashMap;

use wasm_encoder::{Instruction, MemArg, ValType};

use crate::parser::ast::Expression;

use super::{signatures::TypeRegistry, strings::{ALLOCATE_STRING_IDX, STORE_STRING_IDX}, variables::WasmVariables};

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
        "string" => Some(ValType::I32),
        _ => None
    }
}

/// Get a param type by named expression
pub fn get_param_type_by_named_expression(param: Expression) -> StrongValType {
    match param {
        Expression::Type(tk, name) => {
            let res = get_param_type_by_string(name);
            if res.is_some() {
                StrongValType::Some(res.unwrap())
            } else {
                StrongValType::NotSupported
            }
        },
        Expression::IdentifierWithType(tk, _, var_type) => {
            get_param_type_by_named_expression(var_type.as_ref().to_owned())
        }
        _ => StrongValType::NotSupported,
    }
}

/// Make an instruction for a value.
pub fn make_instruction_for_value(value: &Expression) -> Vec<Instruction> {
    match value {
        Expression::IntegerLiteral(_, v) => vec![Instruction::I32Const(*v as i32)],
        Expression::FloatLiteral(_, v) => vec![Instruction::F32Const(*v as f32)],
        Expression::StringLiteral(_, v) => {
            let str_length = v.len() as i32;
            let str_bytes = v.as_bytes();
        
            // Step 1: Allocate memory for the string.
            let allocate_instr = vec![
                Instruction::I32Const(str_length), // string length
                Instruction::Call(ALLOCATE_STRING_IDX), // allocate the string
            ];
        
            // Step 2: Store string length at the start of the allocated memory.
            let store_length_instr = vec![
                Instruction::I32Const(str_length),
                Instruction::I32Const(4), // length + 4 bytes
                Instruction::I32Add,
                Instruction::I32Const(0), // offset for the length
                Instruction::Call(STORE_STRING_IDX), // store the length
            ];
        
            // Step 3: Copy the string bytes into memory.
            let mut store_bytes_instr = Vec::new();
            for (i, &byte) in str_bytes.iter().enumerate() {
                store_bytes_instr.push(Instruction::I32Const(i as i32 + 4)); // offset to start after length
                store_bytes_instr.push(Instruction::I32Const(byte as i32));
                store_bytes_instr.push(Instruction::I32Store8(MemArg {
                    align: 0,
                    offset: 0,
                    memory_index: 0,
                }));
            }
        
            // Combine all instructions
            [allocate_instr, store_length_instr, store_bytes_instr].concat()
        }
        _ => vec![Instruction::Nop],
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
        Expression::StringLiteral(_, _) => ValType::I32,
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
        }
    };

    StrongValType::Some(res)
}
