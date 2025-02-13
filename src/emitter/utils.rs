use wasm_encoder::{Instruction, ValType};

use crate::parser::ast::Expression;

pub enum StrongValType {
    None,
    Some(ValType),
    NotSupported
}

pub fn get_param_type(param: Expression) -> StrongValType {
    match param {
        Expression::Identifier(tk, name) => match name.as_ref() {
            "i32" => StrongValType::Some(ValType::I32),
            "f32" => StrongValType::Some(ValType::F32),
            "f64" => StrongValType::Some(ValType::F64),
            "i64" => StrongValType::Some(ValType::I64),
            "bool" => StrongValType::Some(ValType::I32),
            _ => StrongValType::NotSupported,
        },
        Expression::IdentifierWithType(tk, _, var_type) => {
            get_param_type(var_type.as_ref().to_owned())
        }
        _ => StrongValType::NotSupported,
    }
}

/// Make an instruction for a value.
pub fn make_instruction_for_value(value: &Expression) -> Instruction {
    match value {
        Expression::IntegerLiteral(_, v) => {
            Instruction::I32Const(*v as i32)
        }
        Expression::FloatLiteral(_, v) => {
            Instruction::F32Const(*v as f32)
        }
        _ => Instruction::Nop,
    }
}
