// Native compiler. Used within transpiler.rs

use std::collections::HashMap;

use crate::{
    emitter::{instruction_generator::EasyInstructions, signatures::{EasyNativeVar, FunctionSignature}, utils::{get_param_type_by_string, StrongValType}},
    parser::ast::{Expression, Statement},
};

/// easyjs native is a bare bones language feature. Think of it as C for the web.
/// To use easyjs native features, wrap your easyjs code in a native block like so:
/// ```
/// native { // native wrapper
///     raw = @use_mod("raw") // import raw instructions
///     
///     fn add(a:int, b:int):int { // types are required in native blocks!
///         return a + b
///         
///         // You can also use raw wasm calls.
///         raw.local_get(raw.local_from_ident(a))
///         raw.local_get(raw.local_from_ident(b))
///         raw.i32_add()
///         // this gets returned automatically
///         raw.return_() // but you can also use raw
///     }
///
///     // Builtin types [int, float, string, array, bool, dict, any]
///     fn hello_world():string {
///         return "Hello world"
///         // Raw strings are also supported
///         raw.set_local(0, raw.set_string_size(11))
///         
///         raw.const_i32(0x001) // The byte of H
///         raw.i32_store8(0,0,0)
///         // The rest of the string
///         // This is done automatically by the native{} compiler.
///         raw.get_local(0) // return the string pointer
///     }
/// }
///
/// ```
pub fn compile_native(stmts: &Vec<&Statement>) -> Vec<u8> {
    vec![]
}

/// The native context used by the compiler. Keeps track of Functions/Variables/Structs/etc.
struct NativeContext {
    /// A map of function signatures (by idx.)
    /// To get a function by name, loop through this map.
    function_signatures: HashMap<i64, FunctionSignature>,

    /// Scoped variables.
    ///
    /// Variables use EasyNativeVar to track their context.
    variable_scope: Vec<Vec<EasyNativeVar>>,

    /// Whether or not this context is valid.
    /// False when we have an error, invalid statements, unsopported, etc...
    pub not_valid_reason: Option<String>,

}

impl NativeContext {
    fn new() -> Self {
        NativeContext {
            function_signatures: HashMap::new(),
            variable_scope: vec![vec![]],
            not_valid_reason: None
        }
    }

    /// Compile a native statement.
    fn compile_statement(&mut self, stmt: &Statement, is_pub: bool) {
        match stmt {
            Statement::VariableStatement(_, name, val_type, value, should_infer) => {
                // Pseudo code
                // let var_name = self.compile_raw_expression(name);
                // let var_value = self.get_value_from_expression(value);
                // if should_infer {} for i in variables {} or for i in fn {}
            }
            _ => {
                // This stmt is not supported in native blocks (yet)
                self.not_valid_reason = Some("Unsupported statement".to_string());
            }
        }
    }

    /// Compile a native expression (to be used only within NativeContext logic.)
    /// 
    /// This returns a list of Instructions that are then used for compilation.
    fn compile_expression(&mut self, expr: &Expression) -> EasyInstructions {
        vec![]
    }

    /// Compile the raw expression into a String
    /// 
    /// Does not work for all expressions.
    /// 
    /// Works for:
    /// - identifiers
    /// - string literals
    /// - function calls (the name)
    /// - function calls (the arguments)
    fn compile_raw_expression(&mut self, expr: &Expression) -> String {
        match expr {
            Expression::Identifier(_, name) => {
                name.clone()
            }
            Expression::StringLiteral(_, lit) => {
                lit.clone()
            }
            Expression::FunctionLiteral(_, name, _,_ , _) => {
                self.compile_raw_expression(name.as_ref())
            }
            _ => {
                // add error
                self.not_valid_reason = Some("Can not compile raw expression".to_string());
                String::new()
            }
        }
    }

    /// Get the `StrongValType` from an expression.
    /// 
    /// Works for:
    /// - identifiers with types
    /// - literals
    /// - function calls (the return type)
    /// - function calls (the arguments with types)
    fn get_value_from_expression(&mut self, expr: &Expression) -> StrongValType {
        match expr {
            Expression::Identifier(_, name) => {
                let res = get_param_type_by_string(name);
                if res == StrongValType::NotSupported {
                    self.not_valid_reason = Some("Can not get value from expression".to_string());
                    StrongValType::NotSupported
                } else {
                    res
                }
            }
            Expression::IdentifierWithType(_, name, val_type) => {
                self.get_value_from_expression(val_type.as_ref())
            }
            Expression::FunctionLiteral(_, _, _, val_type, _) => {
                if val_type.is_none() {
                    self.not_valid_reason = Some("Can not get value from expression".to_string());
                    StrongValType::NotSupported
                }
                else {
                    self.get_value_from_expression(val_type.clone().unwrap().as_ref())
                }
            }
            _ => {
                // add error
                self.not_valid_reason = Some("Can not get value from expression".to_string());
                StrongValType::NotSupported
            }
        }
    }
}
