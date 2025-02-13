use std::collections::HashMap;
use wasm_encoder::{
    CodeSection, ConstExpr, ExportKind, ExportSection, Function, FunctionSection, GlobalSection,
    GlobalType, Instruction, Module, TypeSection, ValType,
};

use crate::parser::ast::{self, Expression, Statement};

use super::{
    fn_builder::FNBuilder,
    signatures::{FunctionSignature, TypeRegistry},
    utils::{get_param_type, make_instruction_for_value, StrongValType},
    variables::WasmVariables,
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

    /// Global section
    global_section: GlobalSection,

    /// The global variables
    pub variables: WasmVariables,
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
        global_section: GlobalSection::new(),
        variables: WasmVariables::new(),
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

    easy_wasm.finish()
}

impl EasyWasm {
    fn finish(&mut self) -> Vec<u8> {
        // Type Section (Defines function signatures)
        self.type_registry.emit(&mut self.module);
        // Import Section (Imports from the host environment)
        // Function Section (Declares function indices)
        self.module.section(&self.function_section);
        // Table Section (For indirect function calls)
        // Memory Section (Defines memory for the module)
        // Global Section (Declares global variables) ✅
        self.module.section(&self.global_section);
        // Export Section (Exports functions, memory, globals, etc.)
        self.module.section(&self.export_section);
        // Start Section (Defines an optional function to run at startup)
        // Element Section (For function table initialization)
        // Code Section (Contains function bodies)
        self.module.section(&self.code_section);
        // Data Section (For initializing memory)

        self.module.clone().finish()
    }

    /// Add a global variable. mut and non mut.
    fn add_variable(&mut self, var: &Expression, ty: &Expression, val: &Expression, mutable: bool) {
    }

    fn add_statement(&mut self, stmt: Statement, is_public: bool) {
        match stmt {
            Statement::VariableStatement(tk, var, var_type, value) => {
                // get var type
                let var_type = var_type
                    .expect("Type must exist in native blocks")
                    .as_ref()
                    .to_owned();
                let var_type = get_param_type(var_type);
                let mut val_type: ValType;
                match var_type {
                    StrongValType::None => {
                        // TODO: set an error
                        unimplemented!();
                        // return;
                    }
                    StrongValType::NotSupported => {
                        // TODO: set an error
                        unimplemented!();
                        // return;
                    }
                    StrongValType::Some(ty) => {
                        val_type = ty;
                    }
                }

                // get var name
                let var = var.as_ref().to_owned();
                let var = match var {
                    Expression::Identifier(tk, name) => name,
                    _ => panic!("Unsupported variable name"),
                };

                // get value
                let value = value.as_ref().to_owned();
                // setup the const expression
                let const_expr = match value {
                    Expression::IntegerLiteral(tk, v) => {
                        if val_type == ValType::I32 {
                            ConstExpr::i32_const(v as i32)
                        } else {
                            ConstExpr::i64_const(v)
                        }
                    }
                    Expression::FloatLiteral(tk, v) => {
                        if val_type == ValType::F32 {
                            ConstExpr::f32_const(v as f32)
                        } else {
                            ConstExpr::f64_const(v)
                        }
                    }
                    _ => {
                        // TODO: set an error
                        unimplemented!();
                        // return;
                    }
                };
                self.global_section.global(
                    GlobalType {
                        val_type: val_type,
                        mutable: true,
                        shared: false,
                    },
                    &const_expr,
                );
                self.variables.add_variable(var, val_type);
            }
            Statement::ExpressionStatement(tk, expr) => match expr.as_ref().to_owned() {
                Expression::FunctionLiteral(tk, name, params, var_type, body) => {
                    self.add_function(
                        name.as_ref().to_owned(),
                        params.as_ref().to_owned(),
                        var_type,
                        body.as_ref().to_owned(),
                        is_public,
                    );
                }
                _ => {
                    // TODO: set an error
                    unimplemented!();
                    // return;
                }
            },
            _ => {
                // TODO: set an error
                unimplemented!();
                // return;
            }
        }
    }

    /// Get the function idx from the function name.
    pub fn get_function_idx(&self, name: &str) -> u32 {
        let func_name = self
            .function_names
            .get(name)
            .expect("Function name not found");
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
        let wasm_params: Vec<StrongValType> = params
            .iter()
            .map(|param| get_param_type(param.clone()))
            .collect();
        let results = {
            if let Some(return_type) = return_type {
                vec![get_param_type(return_type.as_ref().to_owned())]
            } else {
                vec![]
            }
        };

        // parse the strong val types
        let parsed_wasm_params = {
            let mut vals = vec![];
            for param in wasm_params.iter() {
                match param {
                    StrongValType::Some(ty) => vals.push(ty.clone()),
                    _ => {
                        // TODO: ERROR
                        unimplemented!();
                    }
                }
            }
            vals
        };
        let parsed_results = {
            let mut vals = vec![];
            for param in results.iter() {
                match param {
                    StrongValType::Some(ty) => vals.push(ty.clone()),
                    _ => {
                        // TODO: ERROR
                        unimplemented!();
                    }
                }
            }
            vals
        };

        let sig = FunctionSignature {
            params: parsed_wasm_params,
            results: parsed_results,
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
                    let strong_type = get_param_type(var_type.as_ref().to_owned());
                    match strong_type {
                        StrongValType::Some(ty) => {
                            fn_builder.add_param(name, ty);
                        }
                        _ => {
                            // we will never get here!
                            unimplemented!();
                        }
                    }
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
