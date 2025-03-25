// Native compiler. Used within transpiler.rs

use std::collections::HashMap;

use crate::{
    emitter::{instruction_generator::EasyInstructions, signatures::{EasyNativeFN, EasyNativeVar, FunctionSignature}, utils::{expression_is_ident, get_param_type_by_string, StrongValType}},
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
pub fn compile_native(stmts: &Vec<Statement>) -> Vec<u8> {
    vec![]
}

/// The native context used by the compiler. Keeps track of Functions/Variables/Structs/etc.
struct NativeContext {
    /// A vector of functions.
    /// 
    /// To access each one loop through the vector.
    functions: Vec<EasyNativeFN>,

    /// Scoped variables.
    ///
    /// Variables use EasyNativeVar to track their context.
    variable_scope: Vec<Vec<EasyNativeVar>>,

    /// Whether or not this context is valid.
    /// False when we have an error, invalid statements, unsopported, etc...
    errors: Vec<String>,

    /// Is the context currently global.
    is_currently_global: bool
}

impl NativeContext {
    fn new() -> Self {
        NativeContext {
            functions: Vec::new(),
            variable_scope: vec![vec![]],
            errors: Vec::new(),
            is_currently_global: true
        }
    }

    /// Compile a native statement.
    fn compile_statement(&mut self, stmt: &Statement, is_pub: bool) {
        match stmt {
            Statement::VariableStatement(_, name, val_type, value, should_infer) => {
                // Pseudo code
                let var_name = self.compile_raw_expression(name);
                // let mut val_value 
                // let var_value = {
                //     if !*should_infer {
                //         self.get_val_type_from_expression(&value)
                //     } else {
                //         for scope in self.variable_scope.iter().rev() {
                //             for variable in scope.iter() {
                //                 if variable.name == var_name {
                //                     return variable.val_type.clone();
                //                 }
                //             }
                //         }
                //         for function in self.functions.iter() {
                //             if function.name == var_name {
                //                 return function.signature.results_strong[0].clone();
                //             }
                //         }
                //         StrongValType::None
                //     }
                // };
                // When we infer we need to check if the value is a identifier or not.

                // if *should_infer {
                //     if expression_is_ident(value) {
                //         // Look for Variables or functions in scope.
                //         // for scope in self.variable_scope.iter().rev() {
                //         //     // for variable in scope {
                //         //     //     if variable.name == var_name {
                //         //     //         // we got the variable
                                    
                //         //     //     }
                //         //     // }
                //         // }
                //     }
                // }
                // if should_infer {} for i in variables {} or for i in fn {}
            }
            _ => {
                // This stmt is not supported in native blocks (yet)
                self.add_error("Unsupported statement");
            }
        }
    }

    /// Compile a native expression (to be used only within NativeContext logic.)
    /// 
    /// This returns a list of Instructions that are then used for compilation.
    fn compile_expression(&mut self, expr: &Expression) -> EasyInstructions {
        match expr {
            _ => {
                vec![]
            }
        }
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
                self.add_error("Can not compile raw expression");
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
    fn get_val_type_from_expression(&mut self, expr: &Expression) -> StrongValType {
        match expr {
            Expression::Identifier(_, name) => {
                let res = get_param_type_by_string(name);
                if res == StrongValType::NotSupported {
                    self.add_error("Can not get value from expression");
                    StrongValType::NotSupported
                } else {
                    res
                }
            }
            Expression::IdentifierWithType(_, name, val_type) => {
                self.get_val_type_from_expression(val_type.as_ref())
            }
            Expression::FunctionLiteral(_, _, _, val_type, _) => {
                // There is no way to infer the return type of a function (not yet)
                if val_type.is_none() {
                    self.add_error("Can not get value from expression.");
                    // self.not_valid_reason = Some("Can not get value from expression".to_string());
                    StrongValType::NotSupported
                }
                else {
                    self.get_val_type_from_expression(val_type.clone().unwrap().as_ref())
                }
            }
            _ => {
                // add error
                self.add_error("Can not get value from expression");
                StrongValType::NotSupported
            }
        }
    }

    /// Add a error
    fn add_error(&mut self, error: &str) {
        self.errors.push(error.to_string());
    }
}
