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
    fn_builder::FNBuilder, instruction_generator::EasyInstructions, signatures::{FunctionSignature, TypeRegistry}, strings::{allocate_string, store_string_length}, utils::{
        expression_is_ident, get_param_type_by_named_expression, get_val_type_from_strong, infer_variable_type, make_instruction_for_value, parse_strong_from_expression, StrongValType
    }, variables::WasmVariables
};

/// Represents a variable.
/// Holds a name, type, and idx.
struct Variable {
    pub name: String,
    pub val_type: StrongValType,
    pub idx: u32
}

/// Represents a function.
/// Holds a name, type, and idx.
struct NativeFunction {
    pub name: String,
    pub val_type: StrongValType,
    pub idx: u32,
}

/// EasyWasm is tasked with compiling easyjs native code blocks into WebAssembly.
/// Easyjs native is a bare bones language feature. Think of it as C for the web.
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

    /// Variables by scope
    variable_scopes: Vec<Vec<Variable>>,
    
    /// Functions by scope
    native_function_scopes: Vec<Vec<NativeFunction>>,

    /// Errors
    pub errors: Vec<EasyError>,
}

/// Emits a WebAssembly module as Bytes.
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
        native_function_scopes: vec![vec![]],
        variable_scopes: vec![vec![]],
        // variables: WasmVariables::new(),
        errors: vec![],
    };

    easy_wasm.setup();

    // iterate over the statements and add them to the module
    for stmt in stmts {
        match &stmt {
            Statement::ExportStatement(_, stmt) => {
                easy_wasm.compile_statement(stmt.as_ref(), true);
            }
            _ => {
                easy_wasm.compile_statement(&stmt, false);
            }
        }
    }

    easy_wasm.finish()
}

impl EasyWasm {
    /// Add a new variable scope
    fn new_variable_scope(&mut self) {
        self.variable_scopes.push(vec![]);
    }

    /// Pop the most recent variable scope
    fn pop_variable_scope(&mut self) {
        self.variable_scopes.pop();
    }

    /// Add a new function scope
    fn new_function_scope(&mut self) {
        self.native_function_scopes.push(vec![]);
    }

    /// Pop the most recent function scope
    fn pop_function_scope(&mut self) {
        self.native_function_scopes.pop();
    }

    /// Create a new scope for both variables and functions.
    fn new_scope(&mut self) {
        self.new_variable_scope();
        self.new_function_scope();
    }

    /// Pop the most recent scope for both variables and functions.
    fn pop_scope(&mut self) {
        self.pop_variable_scope();
        self.pop_function_scope();
    }

    /// Setup the EasyWasm module.
    ///
    /// This includes builtin functions for strings, arrays, dictionaries, structs, classes.
    fn setup(&mut self) {
        // This scope never gets popped!
        self.new_scope();

        // TOOD: add default instructions
        // crate::std::load_std("strings");
        // crate::std::load_std("arrays");
        // crate::std::load_std("dicts");
        // crate::std::load_std("tuples");
        // Parse default functions into instructions
        // let mut easy_wasm = EasyWasm::new();

        // // add memory (just 1 page for now)
        // self.memory_section.memory(MemoryType {
        //     minimum: 1,
        //     maximum: None,
        //     memory64: false,
        //     shared: false,
        //     page_size_log2: None,
        // });

        // // export it
        // self.export_section.export("memory", ExportKind::Memory, 0);

        // add global variables
        // let global_variables = [1024];
        // for global_var in global_variables {
        //     self.global_section.global(
        //         GlobalType {
        //             val_type: ValType::I32,
        //             mutable: true,
        //             shared: false,
        //         },
        //         &ConstExpr::i32_const(global_var as i32),
        //     );
        // }

        // // add easy_native_fns in order!
        // let easy_native_fns = [allocate_string(), store_string_length()];
        // for fn_def in easy_native_fns {
        //     // add to type registry
        //     let type_index = self
        //         .type_registry
        //         .add(fn_def.signature.clone(), fn_def.name.clone());
        //     // add to function names.
        //     self.function_names
        //         .insert(fn_def.name.clone(), format!("f{}", fn_def.idx));
        //     // add to function section
        //     self.function_section.function(type_index);
        //     // add to codes section
        //     self.code_section.function(&fn_def.function);

        //     // if it is public, export it
        //     if fn_def.is_public {
        //         self.export_section
        //             .export(&fn_def.name, ExportKind::Func, fn_def.idx);
        //     }
        // }
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
                    infer_variable_type(val, &self.variables, &self.type_registry, None)
                }
            }
        };

        let val_type = get_val_type_from_strong(&ty).expect("Could not get val type.");

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
        self.variables.add_variable(var_name.to_owned(), ty.clone());
    }

    /// Add a native statement.
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

    /// Add a function.
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

        let sig = FunctionSignature {
            params: wasm_params
                .iter()
                .map(|p| get_val_type_from_strong(p).expect("Could not parse strong val type"))
                .collect(),
            results: results
                .iter()
                .map(|p| get_val_type_from_strong(p).expect("Could not parse strong val type"))
                .collect(),
            param_strong: wasm_params,
            results_strong: results,
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
                    fn_builder.add_param(name, strong_type);
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

    /// Compile a Statement.
    /// 
    /// This actually appends the instructions in a logical manner.
    fn compile_statement(&mut self, stmt: &Statement, is_public: bool) {
        match stmt {
            Statement::VariableStatement(_, name, val_type, value, should_infer) => {
                self.compile_variable_stmt(name.as_ref(), val_type, value.as_ref(), should_infer);
            }
            _ => {}
        }
    }

    fn compile_variable_stmt(&mut self, name: &Expression, val_type: &Option<Box<Expression>>, value: &Expression, should_infer: &bool) {
        let value_type : StrongValType = {
            // check infer...
            if val_type.is_none() {
                // check if we are dealing with a identifier
                if !expression_is_ident(value) {
                    // easy peasy dog
                    parse_strong_from_expression(value)
                } else {
                    // Find variable in scope
                    for scope in self.variable_scopes.iter().rev() {
                        for variable in scope.iter() {
                            if variable.name == name {
                                return variable.val_type.clone();
                            }
                        }
                    }

                    for scope in self.native_function_scopes.iter().rev() {
                        for function in scope.iter() {
                            if function.name == name {
                                return function.val_type.clone();
                            }
                        }
                    }
                }
                // if expression_is_ident(value) {
                //     // Look for Variables or functions in scope.
                //     for variable in self.variable_scopes.iter().rev() {
                //         if variable.iter().find_map(|v| v.name == value)
                //     }
                // }
            } else {
                parse_strong_from_expression(val_type.unwrap().as_ref())
            }
        };

        if *should_infer {}
    }

    /// Compile a Expression.
    fn compile_expression(&mut self, expr: &Expression) -> EasyInstructions {
        match expr {
            Expression::FunctionLiteral(_, name, params, val_type, body) => {

            }   
            _ => {}
        }
        
        vec![]
    }
}
