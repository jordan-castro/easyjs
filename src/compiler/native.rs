// Native compiler. Used within transpiler.rs

use std::{collections::HashMap, vec};

use wasm_encoder::{ConstExpr, Function, Instruction, ValType};

use crate::{
    emitter::{
        instruction_generator::EasyInstructions,
        signatures::{create_type_section, EasyNativeFN, EasyNativeVar, FunctionSignature},
        utils::{
            expression_is_ident, get_param_type_by_string, get_val_type_from_strong, StrongValType,
        },
    },
    parser::ast::{Expression, Statement},
};

/// easyjs native is a bare bones language feature. Think of it as C for the web.
/// To use easyjs native features, wrap your easyjs code in a native block like so:
/// ```
/// native { // native wrapper
///     raw = @use_mod("raw") // import raw instructions
///
///     fn main() {
///         // While not required, this is where you would apply advanced logic to global variables
///         // for example, if you have a global variable of type int, you could set it to a function call.
///         // You can not do that automatically in global scope. i.e. native {} without a fn scope.
///         // no need to return anything. This is a void fn
///     }
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
///
/// It should be mentioned that `native` is only implemented to a very basic degree. The reason for this is because currently,
/// easyjs does not make me any money of any kind.
///
/// But with the basic implementation a person could essentially program every type of logic they want. They could even add arrays, dicts, sets, etc.
///
/// `native` only supports functions, variables [global and local], math, and strings [only raw and concat]. To add to any of those you would have to
/// use the builtin raw instructions.
pub fn compile_native(stmts: &Vec<Statement>) -> Result<Vec<u8>, String> {
    // Setup context
    let mut ctx = NativeContext::new();

    for stmt in stmts {
        match stmt {
            Statement::ExportStatement(_, stmt) => {
                ctx.compile_statement(stmt, true);
            }
            _ => {
                ctx.compile_statement(stmt, false);
            }
        }
        // ctx.compile_statement(stmt, is_pub);
    }

    if ctx.errors.len() > 0 {
        // ruh roh, we have an error
        // only return the first error
        return Err(ctx.errors[0].clone());
    }

    // Setup WASM modules
    let mut module = wasm_encoder::Module::new();

    // Type Section (Defines function signatures)
    let fn_signatures = ctx.functions.iter().map(|f| f.signature.clone()).collect();
    // create function section here to add types correctly.
    let mut function_section = wasm_encoder::FunctionSection::new();
    module.section(&create_type_section(fn_signatures, &mut function_section));

    // Import Section (Imports from the host environment)

    // Function Section (Declares function indices)
    module.section(&function_section);

    // Table Section (For indirect function calls)

    // Memory Section (Defines memory for the module)

    // Global Section (Declares global variables)
    let mut global_section = wasm_encoder::GlobalSection::new();
    // for var in ctx.variable_scope[0].iter() {
    //     let mut global = wasm_encoder::
    // }
    module.section(&global_section);

    // Export Section (Exports functions, memory, globals, etc.)
    let mut export_section = wasm_encoder::ExportSection::new();
    for fun in ctx.functions.iter() {
        if fun.is_public {
            export_section.export(&fun.name, wasm_encoder::ExportKind::Func, fun.idx as u32);
        }
    }
    module.section(&export_section);

    // Start Section (Defines an optional function to run at startup)

    // Element Section (For function table initialization)

    // Code Section (Contains function bodies)
    let mut code_section = wasm_encoder::CodeSection::new();
    for fun in ctx.functions.iter_mut() {
        for instruction in ctx.instructions.get(&fun.idx).unwrap() {
            fun.function.instruction(instruction);
        }
        code_section.function(&fun.function);
    }

    module.section(&code_section);

    // Data Section (For initializing memory)

    Ok(module.finish())
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
    ///
    /// Important to note that global variables are in the first scope and will never be popped.
    /// All other scopes are used internally for compiling the functions.
    variable_scope: Vec<Vec<EasyNativeVar>>,

    /// Whether or not this context is valid.
    /// False when we have an error, invalid statements, unsopported, etc...
    errors: Vec<String>,

    /// Is the context currently global.
    is_currently_global: bool,

    /// The next idx for variables.
    next_var_idx: u32,

    /// The next idx for functions.
    next_fn_idx: u32,

    /// The generated instructions
    ///
    /// The way this works is that the instructions are matched to a idx.
    ///
    /// It is up to the compiler to match instructions with the function idx.
    instructions: HashMap<u32, EasyInstructions>,

    /// Is currently in a public scope?
    is_pub: bool,
}

impl NativeContext {
    fn new() -> Self {
        NativeContext {
            functions: Vec::new(),
            variable_scope: vec![vec![]],
            errors: Vec::new(),
            is_currently_global: true,
            next_var_idx: 0,
            next_fn_idx: 0,
            instructions: HashMap::new(),
            is_pub: false,
        }
    }

    /// Add a new variable scope.
    fn add_scope(&mut self) {
        self.variable_scope.push(vec![]);
    }

    /// Pop current variable scope
    fn pop_scope(&mut self) {
        self.variable_scope.pop();
    }

    /// Compile a native statement.
    fn compile_statement(&mut self, stmt: &Statement, is_pub: bool) {
        self.is_pub = is_pub;
        match stmt {
            Statement::VariableStatement(_, name, val_type, value, _) => {
                self.compile_variable_stmt(name, value, true);
            }
            Statement::ConstVariableStatement(_, name, val_type, value, _) => {
                self.compile_variable_stmt(name, value, false);
            }
            Statement::ExpressionStatement(_, expr) => {
                // add instructinos to current function
                let instructions = self.compile_expression(expr);
                if let Some(current_fn_ins) = self.instructions.get_mut(&self.next_fn_idx) {
                    current_fn_ins.append(&mut instructions.clone());
                }
            }
            Statement::ReturnStatement(_, expr) => {
                let mut instructions = self.compile_expression(expr);
                instructions.push(Instruction::Return);
                if let Some(current_fn_ins) = self.instructions.get_mut(&self.next_fn_idx) {
                    current_fn_ins.append(&mut instructions);
                }
            }
            Statement::BlockStatement(_, stmts) => {
                for stmt in stmts.as_ref() {
                    self.compile_statement(stmt, is_pub);
                }
            }
            _ => {
                // This stmt is not supported in native blocks (yet)
                self.add_error("Unsupported statement");
            }
        }
    }

    /// Compile a native variable.
    ///
    /// It is important to note whether or not the variable is in the global scope.
    fn compile_variable_stmt(&mut self, name: &Expression, value: &Expression, is_mut: bool) {
        // get variable name as raw expression
        let var_name = self.compile_raw_expression(name);
        let strong_val_type = self.get_val_type_from_expression(&value);

        if self.is_currently_global {
            let parsed = self.compile_global_variable_stmt(value);
            // add to variable
            self.variable_scope[0].push(EasyNativeVar {
                name: var_name,
                idx: self.next_var_idx,
                is_global: true,
                value: parsed,
                val_type: strong_val_type,
                is_mut,
            });
        } else {
            // add to variable scope
            self.variable_scope.last_mut().unwrap().push(EasyNativeVar {
                name: var_name,
                idx: self.next_var_idx,
                is_global: false,
                value: ConstExpr::empty(),
                val_type: strong_val_type,
                is_mut,
            });

            // add to instructions
            if let Some(current_fn_ins) = self.instructions.get_mut(&self.next_fn_idx) {
                current_fn_ins.push(Instruction::LocalSet(self.next_var_idx));
            } else {
                self.add_error("No current function");
            }
        }

        // Add to the next var idx
        self.next_var_idx += 1;
    }

    /// Compile a global variable.
    ///
    /// This is used to get a ConstExpr.
    /// It is only used for global variables in the global scope. Global variables in the local scope act as normal.
    fn compile_global_variable_stmt(&mut self, value: &Expression) -> ConstExpr {
        // Let's parse and get the value type
        match value {
            Expression::IntegerLiteral(_, val) => ConstExpr::i32_const(*val as i32),
            Expression::FloatLiteral(_, val) => ConstExpr::f32_const(*val as f32),
            Expression::Boolean(_, val) => ConstExpr::i32_const(*val as i32),
            Expression::Identifier(_, name) => {
                // check if null
                if name != "null" {
                    self.add_error("Unsupported expression as value for global variable");
                }
                ConstExpr::empty()
            }
            _ => {
                // not supported
                self.add_error("Unsupported expression as value for global variable");
                ConstExpr::empty()
            }
        }
    }

    /// Compile a native expression (to be used only within NativeContext logic.)
    ///
    /// This returns a list of Instructions that are then used for compilation.
    fn compile_expression(&mut self, expr: &Expression) -> EasyInstructions {
        match expr {
            Expression::Identifier(_, name) => {
                // Ok this one is a little interesting... we need to check if the variable is in the global scope or the local scope
                // of if the name is a function i.e. call

                // local
                for var in self.variable_scope.get(1).unwrap() {
                    if var.name == *name {
                        return vec![Instruction::LocalGet(var.idx)];
                    }
                }
                // global
                for var in self.variable_scope.get(0).unwrap() {
                    if var.name == *name {
                        return vec![Instruction::GlobalGet(var.idx)];
                    }
                }

                // check functions
                if let Some(fun_idx) = self.get_fun_idx_from_name(name) {
                    return vec![Instruction::Call(fun_idx)];
                }
                // some error
                self.add_error(format!("native, error compiling identifier: {} ", name).as_str());
                vec![]
            }
            Expression::FunctionLiteral(_, name, params, val_type, body) => {
                self.compile_function_literal(name, params, val_type.as_ref().unwrap(), body);
                self.instructions.iter().last().unwrap().1.clone()
            }
            Expression::IntegerLiteral(_, val) => vec![Instruction::I32Const(*val as i32)],
            Expression::FloatLiteral(_, val) => vec![Instruction::F32Const(*val as f32)],
            Expression::InfixExpression(_, left, op, right) => {
                let left = self.compile_expression(left.as_ref());
                let right = self.compile_expression(right.as_ref());

                let left_type = self.get_val_type_from_instruction(left.last().unwrap());
                let right_type = self.get_val_type_from_instruction(right.last().unwrap());

                // if left is int and right is float: use f32Add
                // hierarchy: f64 > f32 > i64 > i32
                let instruction_type = match (&left_type, &right_type) {
                    (StrongValType::Int, StrongValType::Int) => "int",
                    (StrongValType::Int, StrongValType::Float) => "f32",
                    (StrongValType::Float, StrongValType::Int) => "f32",
                    (_, _) => {
                        self.add_error(format!("native, Unsupported operation: {:#?} {} {:#?}", left_type, op, right_type).as_str());
                        return vec![];
                    }
                };

                match op.as_str() {
                    "+" => {
                        let mut result = vec![];
                        result.append(&mut left.clone());
                        result.append(&mut right.clone());
                        match instruction_type {
                            "int" => result.push(Instruction::I32Add),
                            "f32" => result.push(Instruction::F32Add),
                            _ => {}
                        }
                        result
                    }
                    "-" => {
                        let mut result = vec![];
                        result.append(&mut left.clone());
                        result.append(&mut right.clone());
                        match instruction_type {
                            "int" => result.push(Instruction::I32Sub),
                            "f32" => result.push(Instruction::F32Sub),
                            _ => {}
                        }
                        result
                    }
                    "*" => {
                        let mut result = vec![];
                        result.append(&mut left.clone());
                        result.append(&mut right.clone());
                        match instruction_type {
                            "int" => result.push(Instruction::I32Mul),
                            "f32" => result.push(Instruction::F32Mul),
                            _ => {}
                        }
                        result
                    }
                    "/" => {
                        let mut result = vec![];
                        result.append(&mut left.clone());
                        result.append(&mut right.clone());
                        match instruction_type {
                            "int" => result.push(Instruction::I32DivS),
                            "f32" => result.push(Instruction::F32Div),
                            _ => {}
                        }
                        result
                    }
                    "%" => {
                        let mut result = vec![];
                        result.append(&mut left.clone());
                        result.append(&mut right.clone());
                        match instruction_type {
                            "int" => result.push(Instruction::I32RemS),
                            "f32" => result.push(Instruction::I32RemS),
                            _ => {}
                        }
                        result
                    }
                    _ => {
                        self.add_error(format!("native, Unsupported operator: {}", op).as_str());
                        vec![]
                    }
                }
            }
            Expression::CallExpression(_, name, arguments) => {
                let name = self.compile_raw_expression(name);
                
                let fun_idx = self.get_fun_idx_from_name(&name);
                if fun_idx.is_none() {
                    self.add_error(format!("native, could not parse function: {:#?}", name).as_str());
                    return vec![];
                }
                let fun_idx = fun_idx.unwrap();

                let mut parsed_arguments = vec![];

                for arg in arguments.as_ref() {
                    let mut instructions = self.compile_expression(arg);
                    parsed_arguments.append(instructions.as_mut());
                }

                let mut result = vec![];
                result.append(&mut parsed_arguments);
                result.push(Instruction::Call(fun_idx));

                result
            }
            _ => {
                vec![]
            }
        }
    }

    /// Compile a FunctionLiteral.
    ///
    /// Returns a list of instructions
    fn compile_function_literal(
        &mut self,
        name: &Expression,
        params: &Vec<Expression>,
        val_type: &Expression,
        body: &Statement,
    ) {
        // get name and params first...
        let name = self.compile_raw_expression(name);
        let param_names = params
            .iter()
            .map(|p| self.compile_raw_expression(p))
            .collect::<Vec<String>>();
        let param_types = params
            .iter()
            .map(|p| self.get_val_type_from_expression(p))
            .collect::<Vec<StrongValType>>();
        let param_val_types = param_types
            .iter()
            .filter_map(|v| get_val_type_from_strong(v))
            .collect::<Vec<ValType>>();
        // lets get the type too
        let return_type = self.get_val_type_from_expression(val_type);

        // add empty instructions
        self.instructions.insert(self.next_fn_idx, vec![]);

        // add a new scope
        self.add_scope();

        // add params to scope
        for i in 0..params.len() {
            self.variable_scope.get_mut(1).unwrap().push(EasyNativeVar {
                name: param_names[i].clone(),
                idx: i as u32,
                is_global: false,
                value: ConstExpr::empty(),
                val_type: param_types[i].clone(),
                is_mut: true,
            });
        }

        // all instructions have been added correctly.
        self.compile_statement(body, self.is_pub);

        // remove the scoped param variables
        for i in 0..params.len() {
            self.variable_scope.get_mut(1).unwrap().remove(0);
        }

        self.instructions
            .get_mut(&self.next_fn_idx)
            .unwrap()
            .push(Instruction::End);

        // set the function
        // get variables of current scope.
        let variables = self.variable_scope.last().unwrap();
        let variable_types = variables
            .iter()
            .map(|v| v.val_type.clone())
            .collect::<Vec<StrongValType>>();
        let variable_val_types = variable_types
            .iter()
            .filter_map(|v| get_val_type_from_strong(v))
            .collect::<Vec<ValType>>();
        let return_val_type = get_val_type_from_strong(&return_type).unwrap();

        // setup function signature.
        let signature = FunctionSignature {
            param_strong: param_types,
            params: param_val_types,
            results: vec![return_val_type],
            results_strong: vec![return_type],
        };

        // count amount of variable types.
        let mut locals = vec![];
        for i in 0..variable_val_types.len() {
            locals.push((1, variable_val_types[i]));
        }

        self.functions.push(EasyNativeFN {
            signature,
            name,
            function: Function::new(locals),
            idx: self.next_fn_idx,
            is_public: self.is_pub,
        });

        // update idx
        self.next_fn_idx += 1;

        // pop current variable scope
        self.pop_scope();
    }

    /// Compile the raw expression into a String
    ///
    /// Does not work for all expressions.
    ///
    /// Works for:
    /// - identifiers
    /// - identifiers with type
    /// - string literals
    /// - function calls (the name)
    /// - function calls (the arguments)
    fn compile_raw_expression(&mut self, expr: &Expression) -> String {
        match expr {
            Expression::Identifier(_, name) => name.clone(),
            Expression::IdentifierWithType(_, name, _) => name.clone(),
            Expression::StringLiteral(_, lit) => lit.clone(),
            Expression::FunctionLiteral(_, name, _, _, _) => {
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
    /// - identifiers without types (will try to infer the type)
    /// - literals
    /// - function calls (the return type)
    /// - function calls (the arguments with types)
    fn get_val_type_from_expression(&mut self, expr: &Expression) -> StrongValType {
        match expr {
            Expression::Identifier(_, name) => {
                let res = get_param_type_by_string(name);
                if res == StrongValType::NotSupported {
                    // Try to infer from variables
                    for scope in self.variable_scope.iter().rev() {
                        for variable in scope {
                            if &variable.name == name {
                                return variable.val_type.clone();
                            }
                        }
                    }
                    // Try to infer from functions
                    for function in self.functions.iter() {
                        if &function.name == name {
                            return function.signature.results_strong[0].clone();
                        }
                    }

                    // If we get this far we could not infer
                    self.add_error("Can not get value from expression");
                    StrongValType::NotSupported
                } else {
                    res
                }
            }
            Expression::Type(_, val_type) => get_param_type_by_string(val_type),
            Expression::IdentifierWithType(_, name, val_type) => {
                self.get_val_type_from_expression(val_type.as_ref())
            }
            Expression::FunctionLiteral(_, _, _, val_type, _) => {
                // There is no way to infer the return type of a function (not yet)
                if val_type.is_none() {
                    self.add_error("Can not get value from expression.");
                    // self.not_valid_reason = Some("Can not get value from expression".to_string());
                    StrongValType::NotSupported
                } else {
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

    /// Get the type of a variable / literal from a instruction
    fn get_val_type_from_instruction(&self, instruction: &Instruction) -> StrongValType {
        match instruction {
            Instruction::I32Const(_) => StrongValType::Int,
            Instruction::LocalGet(idx) => {
                // find the variable type in scopes
                for variable in self.variable_scope.get(1).unwrap() {
                    if variable.idx == *idx {
                        return variable.val_type.clone();
                    }
                }
                StrongValType::NotSupported
            }
            Instruction::GlobalGet(idx) => {
                for variable in self.variable_scope.get(0).unwrap() {
                    if variable.idx == *idx {
                        return variable.val_type.clone();
                    }
                }

                StrongValType::NotSupported
            }
            Instruction::Call(idx) => {
                // find the function type
                for function in self.functions.iter() {
                    if function.idx == *idx {
                        return function.signature.results_strong[0].clone();
                    }
                }
                StrongValType::NotSupported
            }
            // Instruction::I64Const(_) => StrongValType::Long,
            Instruction::F32Const(_) => StrongValType::Float,
            // Instruction::F64Const(_) => StrongValType::Double,
            _ => StrongValType::NotSupported,
        }
    }

    /// Get function idx from name.
    fn get_fun_idx_from_name(&self, name: &str) -> Option<u32> {
        for fun in self.functions.iter() {
            if fun.name == *name {
                return Some(fun.idx);
            }
        }
        None
    }
}
