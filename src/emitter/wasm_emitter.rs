use std::collections::HashMap;
use wasm_encoder::{
    CodeSection, ConstExpr, DataSection, ExportKind, ExportSection, Function, FunctionSection,
    GlobalSection, GlobalType, Instruction, MemorySection, MemoryType, Module, TypeSection,
    ValType,
};

use crate::{
    errors::EasyError,
    parser::ast::{self, Expression, Statement},
};

use super::{
    fn_builder::FNBuilder,
    signatures::{FunctionSignature, TypeRegistry},
    strings::{allocate_string, get_length_string, GLOBAL_STRING_IDX, store_string_byte, store_string_length},
    utils::{
        get_param_type_by_named_expression, infer_variable_type, make_instruction_for_value,
        StrongValType,
    },
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

    /// Memory section
    memory_section: MemorySection,

    /// The global variables
    pub variables: WasmVariables,

    /// Errors
    pub errors: Vec<EasyError>,
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
        memory_section: MemorySection::new(),
        variables: WasmVariables::new(),
        errors: vec![],
    };

    easy_wasm.setup();

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
    /// Setup the EasyWasm module.
    ///
    /// This includes builtin functions for strings, arrays, dictionaries, structs, classes.
    fn setup(&mut self) {
        // add memory (just 1 page for now)
        self.memory_section.memory(MemoryType {
            minimum: 1,
            maximum: None,
            memory64: false,
            shared: false,
            page_size_log2: None,
        });

        // export it
        self.export_section.export("memory", ExportKind::Memory, 0);

        // add global variables
        let global_variables = [GLOBAL_STRING_IDX];
        for global_var in global_variables {
            self.global_section.global(
                GlobalType {
                    val_type: ValType::I32,
                    mutable: true,
                    shared: false,
                },
                &ConstExpr::i32_const(global_var as i32),
            );
        }

        // add easy_native_fns in order!
        let easy_native_fns = [allocate_string(), store_string_byte(), get_length_string(), store_string_length()];
        for fn_def in easy_native_fns {
            // add to type registry
            let type_index = self
                .type_registry
                .add(fn_def.signature.clone(), fn_def.name.clone());
            // add to function names.
            self.function_names
                .insert(fn_def.name.clone(), format!("f{}", fn_def.idx));
            // add to function section
            self.function_section.function(type_index);
            // add to codes section
            self.code_section.function(&fn_def.function);

            // if it is public, export it
            if fn_def.is_public {
                self.export_section.export(&fn_def.name, ExportKind::Func, fn_def.idx);
            }
        }
    }

    fn finish(&mut self) -> Vec<u8> {
        // Type Section (Defines function signatures)
        self.type_registry.emit(&mut self.module);
        // Import Section (Imports from the host environment)
        // Function Section (Declares function indices)
        self.module.section(&self.function_section);
        // Table Section (For indirect function calls)
        // Memory Section (Defines memory for the module)
        self.module.section(&self.memory_section);
        // Global Section (Declares global variables)
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

    fn add_error(&mut self, error: EasyError) {
        self.errors.push(error);
    }

    pub fn type_registry_ref(&self) -> TypeRegistry {
        self.type_registry.to_owned()
    }

    /// Add a global variable. mut and non mut.
    fn add_variable(
        &mut self,
        var: &Expression,
        ty: Option<Box<Expression>>,
        val: &Expression,
        mutable: bool,
        infer_type: bool,
    ) {
        let ty = {
            if let Some(ty) = ty {
                get_param_type_by_named_expression(ty.as_ref().to_owned())
            } else {
                if !infer_type {
                    self.add_error(EasyError::UnsupportedType(
                        "Type not supported.".to_string(),
                    ));
                    return;
                } else {
                    infer_variable_type(val, &self.variables, &self.type_registry)
                }
            }
        };

        let mut val_type: ValType;
        match ty {
            StrongValType::None => {
                self.add_error(EasyError::UnsupportedType(
                    "Unknown variable type.".to_string(),
                ));
                return;
            }
            StrongValType::NotSupported => {
                self.add_error(EasyError::UnsupportedType(
                    "Unknown variable type.".to_string(),
                ));
                return;
            }
            StrongValType::Some(t) => {
                val_type = t;
            }
            StrongValType::String => {
                // TODO:...
                val_type = ValType::I32;
                // unimplemented!("Strings in wasm_emitter");
            }
        }

        // get var name
        let var_name = match var {
            Expression::Identifier(_, name) => name,
            _ => {
                self.add_error(EasyError::Expected(format!("Identifier, found {:?}", var)));
                return;
            }
        };

        // get value
        let const_expr = match val {
            Expression::IntegerLiteral(_, i) => ConstExpr::i32_const(*i as i32),
            Expression::FloatLiteral(_, f) => ConstExpr::f32_const(*f as f32),
            _ => {
                self.add_error(EasyError::Expected(format!(
                    "Integer, Float, found {:?}",
                    val
                )));
                return;
            }
        };

        // set the variable
        self.global_section.global(
            GlobalType {
                val_type,
                mutable,
                shared: false,
            },
            &const_expr,
        );
        self.variables.add_variable(var_name.to_owned(), val_type);
    }

    fn add_statement(&mut self, stmt: Statement, is_public: bool) {
        match stmt.clone() {
            Statement::VariableStatement(tk, var, var_type, value, infer_type) => {
                self.add_variable(var.as_ref(), var_type, value.as_ref(), true, infer_type);
            }
            Statement::ConstVariableStatement(_, var, ty, value, infer_type) => {
                self.add_variable(var.as_ref(), ty, value.as_ref(), false, infer_type);
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
                    self.add_error(EasyError::NotSupported(format!("{:?}", stmt)));
                    return;
                }
            },
            _ => {
                self.add_error(EasyError::NotSupported(format!("{:?}", stmt)));
                return;
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
            .map(|param| get_param_type_by_named_expression(param.clone()))
            .collect();
        let results = {
            if let Some(return_type) = return_type {
                vec![get_param_type_by_named_expression(
                    return_type.as_ref().to_owned(),
                )]
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
                    StrongValType::String => vals.push(ValType::I32),
                    _ => {
                        self.add_error(EasyError::UnsupportedType(
                            "Type is not supported".to_string(),
                        ));
                        return;
                        unimplemented!("Wasm paramter is not supported");
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
                    StrongValType::String => vals.push(ValType::I32),
                    _ => {
                        self.add_error(EasyError::UnsupportedType(
                            "Type is not supported".to_string(),
                        ));
                        return;
                    }
                }
            }
            vals
        };

        let sig = FunctionSignature {
            params: parsed_wasm_params,
            results: parsed_results,
        };

        // function name
        let mut func_name: String;
        match name {
            Expression::Identifier(tk, name) => {
                func_name = name.clone();
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
                self.add_error(EasyError::Expected(format!("Identifier, found {:?}", name)));
                return;
            }
        }

        // get id and add to type registry and function section
        let type_idx = self.type_registry.add(sig, func_name);

        self.function_section.function(type_idx);

        // actually create the function
        let mut fn_builder = FNBuilder::new(&self);
        for param in params {
            match param.clone() {
                Expression::IdentifierWithType(tk, name, var_type) => {
                    let strong_type =
                        get_param_type_by_named_expression(var_type.as_ref().to_owned());
                    match strong_type {
                        StrongValType::Some(ty) => {
                            fn_builder.add_param(name, ty);
                        }
                        StrongValType::String => {
                            fn_builder.add_param(name, ValType::I32);
                        }
                        _ => {
                            // we will never get here!
                            unimplemented!("Function paramater is not supported.");
                        }
                    }
                }
                _ => {
                    self.add_error(EasyError::Expected(format!(
                        "Identifier, found {:?}",
                        param
                    )));
                    return;
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
