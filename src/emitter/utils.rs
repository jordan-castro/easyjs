use wasm_encoder::ValType;

use crate::parser::ast::Expression;

pub fn get_param_type(param: Expression) -> ValType {
    match param {
        Expression::Identifier(tk, name) => match name.as_ref() {
            "i32" => ValType::I32,
            "f32" => ValType::F32,
            "f64" => ValType::F64,
            "i64" => ValType::I64,
            "bool" => ValType::I32,
            _ => panic!("Unsupported type"),
        },
        Expression::IdentifierWithType(tk, _, var_type) => {
            get_param_type(var_type.as_ref().to_owned())
        }
        _ => panic!("Unsupported type"),
    }
}
