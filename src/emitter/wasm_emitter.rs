use std::collections::HashMap;
use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, Function, FunctionSection, Instruction, Module,
    TypeSection, ValType, GlobalSection, GlobalType
};

use crate::parser::ast::{self, Expression, Statement};

use super::{
    fn_builder::FNBuilder,
    signatures::{FunctionSignature, TypeRegistry},
    utils::get_param_type,
};

/// EasyWasm is tasked with compiling easyjs into WebAssembly.
pub struct EasyWasm {
    /// The wasmer module
    module: Module,

    /// The type registry
    type_registry: TypeRegistry,

    /// Function section
    function_section: FunctionSection,

    /// Export section
    export_section: ExportSection,

    /// Code section
    code_section: CodeSection,

    /// Track function names
    function_names: HashMap<String, String>,

    // /// Global section
    // pub global_section: GlobalSection,
}

/// Emits a WebAssembly module.
pub fn emit_wasm(stmts: Vec<Statement>) -> Vec<u8> {
    // create a web assembly module
    let mut module = Module::new();

    let mut easy_wasm = EasyWasm {
        module,
        type_registry: TypeRegistry::new(),
        function_section: FunctionSection::new(),
        export_section: ExportSection::new(),
        code_section: CodeSection::new(),
        function_names: HashMap::new(),
    };

    // iterate over the statements and add them to the module
    for stmt in stmts {
        match stmt.clone() {
            Statement::ExportStatement(_, stmt) => {
                easy_wasm.add_statement(stmt.as_ref().to_owned(), true);
            }
            _ => {
                easy_wasm.add_statement(stmt.clone(), false);
            }
        }
    }
    easy_wasm.type_registry.emit(&mut easy_wasm.module);
    easy_wasm.module.section(&easy_wasm.function_section);
    easy_wasm.module.section(&easy_wasm.export_section);
    easy_wasm.module.section(&easy_wasm.code_section);

    easy_wasm.module.finish()
}

impl EasyWasm {
    fn add_statement(&mut self, stmt: Statement, is_public: bool) {
        match stmt {
            // Statement::VariableStatement(tk, var, var_type, value) => {
            //     let var_type = var_type.expect("Type must exist in ").as_ref().to_owned();
            //     self.add_variable(var.as_ref().to_owned(), var_type, value.as_ref().to_owned())
            // }
            // Statement::ReturnStatement(tk, expr) => {
            //     self.add_return_stmt(expr.as_ref().to_owned());
            // }
            Statement::ExpressionStatement(tk, expr) => match expr.as_ref().to_owned() {
                Expression::FunctionLiteral(tk, name, params, var_type, body) => {
                    // let var_type = var_type.expect("Type must exist in ").as_ref().to_owned();
                    self.add_function(
                        name.as_ref().to_owned(),
                        params.as_ref().to_owned(),
                        var_type,
                        body.as_ref().to_owned(),
                        is_public,
                    );
                }
                _ => {
                    panic!("Unsupported expression statement");
                }
            },
            _ => {
                // panic becuase this is not supported
                panic!("Unsupported native statement");
            }
        }
    }

    /// Get the function idx from the function name.
    pub fn get_function_idx(&self, name: &str) -> u32 {
        let func_name = self.function_names.get(name).expect("Function name not found");
        let func_idx = func_name.split_at(1).1;
        let func_idx = func_idx.parse::<u32>().expect("Function index not found");
        func_idx
    }

    fn add_function(
        &mut self,
        name: Expression,
        params: Vec<Expression>,
        return_type: Option<Box<Expression>>,
        body: Statement,
        is_public: bool,
    ) {
        // Encode the types section
        let wasm_params: Vec<ValType> = params.iter().map(|param| get_param_type(param.clone())).collect();
        let results = {
            if let Some(return_type) = return_type {
                vec![get_param_type(return_type.as_ref().to_owned())]
            } else {
                vec![]
            }
        };

        let sig = FunctionSignature {
            params: wasm_params,
            results,
        };

        // get id and add to type registry and function section
        let type_idx = self.type_registry.add(sig);

        self.function_section.function(type_idx);

        // function names
        match name {
            Expression::Identifier(tk, name) => {
                let func_idx = self.function_names.len() as u32;
                self.function_names
                    .insert(name.clone(), format!("f{}", func_idx));
                // add to export section if this is a public function.
                if is_public {
                    self.export_section
                        .export(&name, ExportKind::Func, func_idx);
                }
            }
            _ => {
                panic!("Unsupported function name");
            }
        }

        // actually create the function
        let mut fn_builder = FNBuilder::new(&self);
        for param in params {
            match param.clone() {
                Expression::IdentifierWithType(tk, name, var_type) => {
                    fn_builder.add_param(name, get_param_type(var_type.as_ref().to_owned()));
                }
                _ => {
                    panic!("Unsupported function parameter");
                }
            }
        }
        fn_builder.compile_statement(&body);

        let (locals, instructions) = fn_builder.finish();
        let mut function = Function::new(locals);

        for instruction in instructions {
            function.instruction(&instruction);
        }

        self.code_section.function(&function);
    }
}
