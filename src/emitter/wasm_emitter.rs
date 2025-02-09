use std::collections::HashMap;
use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, Function, FunctionSection, Instruction, Module,
    TypeSection, ValType,
};

use crate::parser::ast::{self, Expression, Statement};

use super::{
    fn_builder::FNBuilder,
    signatures::{FunctionSignature, TypeRegistry},
    utils::get_param_type,
};

/// EasyWasm is tasked with compiling easyjs into WebAssembly.
struct EasyWasm {
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
                    let var_type = var_type.expect("Type must exist in ").as_ref().to_owned();
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

    fn add_function(
        &mut self,
        name: Expression,
        params: Vec<Expression>,
        return_type: Expression,
        body: Statement,
        is_public: bool,
    ) {
        // Encode the types section
        // let mut types = TypeSection::new();
        let wasm_params: Vec<ValType> = params.iter().map(|param| ValType::I32).collect();
        let results = vec![get_param_type(return_type)];

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
                self.function_names
                    .insert(name.clone(), format!("f{}", type_idx));
                // add to export section if this is a public function.
                if is_public {
                    self.export_section
                        .export(&name, ExportKind::Func, type_idx);
                }
            }
            _ => {
                panic!("Unsupported function name");
            }
        }

        // actually create the function
        let mut fn_builder = FNBuilder::new();
        for param in params {
            match param.clone() {
                Expression::Identifier(tk, name) => {
                    fn_builder.add_param(name, ValType::I32);
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
