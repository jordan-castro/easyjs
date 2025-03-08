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
    None,
    Int,
    Float,
    Bool,
    NotSupported,
    String,
}

pub fn get_param_type_by_string(string: String) -> StrongValType {
    match string.as_str() {
        "int" => StrongValType::Int,
        "bool" => StrongValType::Bool,
        "float" => StrongValType::Float,
        "string" => StrongValType::String,
        _ => StrongValType::None,
    }
}

/// Get a param type by named expression
pub fn get_param_type_by_named_expression(param: Expression) -> StrongValType {
    match param {
        Expression::Type(tk, name) => get_param_type_by_string(name),
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
        // Expression::StringLiteral(_, v) => {
        //     let str_length = v.len() as i32;
        //     let str_byts = v.as_bytes();

        //     let mut instructions = vec![];

        //     // allocate memory for the string
        //     instructions.push(Instruction::I32Const(str_length));
        //     instructions.append(&mut call(ALLOCATE_STRING_IDX));
        //     instructions.append(&mut set_local(1));

        //     instructions
        //     // let str_length = v.len() as i32;
        //     // let str_bytes = v.as_bytes();

        //     // // Step 1: Allocate memory for the string.
        //     // let allocate_instr = vec![
        //     //     Instruction::I32Const(str_length), // string length
        //     //     Instruction::Call(ALLOCATE_STRING_IDX), // allocate the string
        //     // ];

        //     // // Step 2: Store string length at the start of the allocated memory.
        //     // let store_length_instr = vec![
        //     //     Instruction::I32Const(str_length),
        //     //     Instruction::I32Const(4), // length + 4 bytes
        //     //     Instruction::I32Add,
        //     //     Instruction::I32Const(0), // offset for the length
        //     //     Instruction::Call(STORE_STRING_IDX), // store the length
        //     // ];

        //     // // Step 3: Copy the string bytes into memory.
        //     // let mut store_bytes_instr = Vec::new();
        //     // for (i, &byte) in str_bytes.iter().enumerate() {
        //     //     store_bytes_instr.push(Instruction::I32Const(i as i32 + 4)); // offset to start after length
        //     //     store_bytes_instr.push(Instruction::I32Const(byte as i32));
        //     //     store_bytes_instr.push(Instruction::I32Store8(MemArg {
        //     //         align: 0,
        //     //         offset: 0,
        //     //         memory_index: 0,
        //     //     }));
        //     // }

        //     // // Combine all instructions
        //     // [allocate_instr, store_length_instr, store_bytes_instr].concat()
        // }
        _ => vec![Instruction::Nop],
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
