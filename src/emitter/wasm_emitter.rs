use std::collections::HashMap;

use wasm_encoder::{Function, Module};

use crate::parser::ast::{self, Expression, Statement};

struct EasyWasm {
    module: Module,
    vars: HashMap<String, i32>,
    functions: HashMap<String, i32>,
    structs: HashMap<String, i32>,
}

/// Emits a WebAssembly module.
pub fn emit_wasm(stmts : Vec<Statement>) -> Vec<u8> {
    // create a web assembly module
    let mut module = Module::new();

    let mut easy_wasm = EasyWasm {
        module,
        vars: HashMap::new(),
        functions: HashMap::new(),
        structs: HashMap::new(),
    };

    // iterate over the statements and add them to the module
    for stmt in stmts {
        easy_wasm.add_statement(stmt);
        // module = add_statement(module, stmt);
    }

    easy_wasm.module.finish()
}

impl EasyWasm {
    fn add_statement(&mut self, stmt: Statement) {
        match stmt {
            Statement::VariableStatement(tk, var, var_type, value) => {
                self.add_variable( var.as_ref().to_owned(), value.as_ref().to_owned())
            }
            Statement::ExpressionStatement(tk, expr) => {
                match expr.as_ref().to_owned() {
                    Expression::FunctionLiteral(tk, name, params, var_type, body) => {
                        self.add_function(name.as_ref().to_owned(), params.as_ref().to_owned(), body.as_ref().to_owned());
                    }
                    _ => {
                        panic!("Unsupported expression statement");
                    }
                }
            }
            _ => {
                // panic becuase this is not supported
                panic!("Unsupported native statement");
            }
        }
    }

    /// Add a variable to a WASM module.
    fn add_variable(&mut self, var: ast::Expression, value: ast::Expression) {

    }

    fn add_function(&mut self, name: Expression, params: Vec<Expression>, body: Statement) {
        // let mut func = Function::new(locals)
        // let mut func = Function::new();
        // func.name(name);
        // func.signature().with_params(params);
        // func.signature().with_results(results);
        // func.body().with_instructions(instructions);
        // self.module.function(func);
    }
}