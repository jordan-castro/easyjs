// Native compiler. Used within transpiler.rs

use std::{collections::HashMap, string, vec};

use crate::{
    emitter::{
        builtins::{
            ALLOCATE_STRING_IDX, STORE_STRING_LENGTH_IDX, STR_CONCAT_IDX, STR_GET_LEN_IDX,
            STR_INDEX_IDX, STR_STORE_BYTE_IDX,
        },
        instruction_generator::{
            EasyInstructions, call_wasm_core_function, is_wasm_core, set_local_string,
        },
        signatures::{
            EasyNativeBlock, EasyNativeFN, EasyNativeVar, FunctionSignature, create_type_section,
        },
        strings::{
            allocate_string, native_str_char_code_at, native_str_concat, native_str_get_len,
            native_str_index, native_str_store_byte, store_string_length,
        },
        utils::{
            StrongValType, expression_is_ident, get_param_type_by_string, get_val_type_from_strong,
        },
    },
    errors::{
        native_can_not_compile_raw_expression, native_can_not_get_value_from_expression,
        native_could_not_parse_function, native_error_compiling_identifier,
        native_if_expression_must_go_within_functions,
        native_no_function_provided_for_variable_scope,
        native_return_value_does_not_match_function, native_unsupported_builtin_call,
        native_unsupported_expression, native_unsupported_expression_as_value_for_global_variable,
        native_unsupported_index_expression, native_unsupported_operation,
        native_unsupported_operator, native_unsupported_prefix_expression,
        native_unsupported_statement,
    },
    lexer::token::{self, Token},
    parser::ast::{Expression, Statement},
};
use wasm_encoder::{
    BlockType, ConstExpr, ExportKind, Function, FunctionSection, GlobalType, Instruction,
    MemorySection, MemoryType, Module, TypeSection, ValType,
};

/// Get the left side idx from a InfixExpression.
///
/// - `left:Expression` The left side expression
macro_rules! get_left_side_idx {
    ($left:expr) => {{
        // Get last instruction from left side
        let last_instruction = $left.last().unwrap();
        let mut is_global = false;
        let mut vidx: i32;
        match last_instruction {
            Instruction::LocalGet(idx) => {
                vidx = idx.clone() as i32;
            }
            Instruction::GlobalGet(idx) => {
                is_global = true;
                vidx = idx.clone() as i32;
            }
            _ => {
                vidx = -1;
            }
        }

        (is_global, vidx)
    }};
}

/// For generating instructions for operators:
/// 
/// - += 
/// - -=
/// - *=
/// - /*
/// 
/// Params:
/// - `left:Expression` The left side expression
/// - `instruction:Instruction` The instruction to generate. i.e. I32Add, F32Add, etc
macro_rules! generate_n_assign_instructions {
    ($left:expr, $instruction:expr) => {{
        let (is_global, idx) = get_left_side_idx!(&$left);
        if is_global {
            vec![
                $instruction,
                Instruction::GlobalSet(idx as u32)
            ]
        } else {
            vec![
                $instruction,
                Instruction::LocalSet(idx as u32)
            ]
        }
    }};
}
                        // "int" => {
                        //     let (is_global, left_idx) = get_left_side_idx!(left);
                        //     if is_global {
                        //         instructions.append(&mut vec![
                        //             Instruction::I32Add,
                        //             Instruction::GlobalSet(left_idx as u32)
                        //         ]);
                        //     } else {
                        //         instructions.append(&mut vec![
                        //             Instruction::I32Add,
                        //             Instruction::LocalSet(left_idx as u32)
                        //         ]);
                        //     }
                        // }

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

    // Builtin functions
    let mut builtin_fns = vec![
        // STRING METHODS
        allocate_string(),
        store_string_length(),
        native_str_store_byte(),
        native_str_get_len(),
        native_str_concat(),
        native_str_index(),
        native_str_char_code_at(),
    ];
    // add to native context.
    ctx.functions.append(&mut builtin_fns);
    // update function_idx
    ctx.next_fn_idx = ctx.functions.len() as u32;

    for stmt in stmts {
        // At the start of each statement we are global
        ctx.is_currently_global = true;
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

    // create function section here to add types correctly.
    let mut function_section = wasm_encoder::FunctionSection::new();
    // Type Section (Defines function signatures)
    let fn_signatures = ctx.functions.iter().map(|f| f.signature.clone()).collect();
    let mut type_section = create_type_section(fn_signatures, &mut function_section);
    module.section(&type_section);

    // Import Section (Imports from the host environment)

    // Function Section (Declares function indices)
    module.section(&function_section);

    // Table Section (For indirect function calls)

    // Memory Section (Defines memory for the module)
    let mut memory_section = wasm_encoder::MemorySection::new();
    memory_section.memory(MemoryType {
        minimum: 1,
        maximum: None,
        memory64: false,
        shared: false,
        page_size_log2: None,
    });
    module.section(&memory_section);

    // Global Section (Declares global variables)
    let mut global_section = wasm_encoder::GlobalSection::new();
    // TODO: dynamic global setting
    global_section.global(
        GlobalType {
            mutable: true,
            shared: false,
            val_type: ValType::I32,
        },
        &ConstExpr::i32_const(0),
    );
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
    export_section.export("memory", ExportKind::Memory, 0);
    module.section(&export_section);

    // Start Section (Defines an optional function to run at startup)

    // Element Section (For function table initialization)

    // Code Section (Contains function bodies)
    let mut code_section = wasm_encoder::CodeSection::new();
    for fun in ctx.functions.iter_mut() {
        // NativeContext.functions do not hold instructions.
        if let Some(instructions) = ctx.instructions.get(&fun.idx) {
            for instruction in instructions {
                fun.function.instruction(instruction);
            }
        } // but builtin functions DO!
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

    /// Block scope
    block_scope: Vec<Vec<EasyNativeBlock>>,
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
            block_scope: Vec::new(),
        }
    }

    /// Add a new variable scope.
    fn add_var_scope(&mut self) {
        self.variable_scope.push(vec![]);
    }

    /// Pop current variable scope
    fn pop_var_scope(&mut self) {
        self.variable_scope.pop();
        self.next_var_idx = 0;
    }

    /// Add a new block scope
    fn add_block_scope(&mut self) {
        self.block_scope.push(vec![]);
    }

    /// Pop current block scope
    fn poop_block_scope(&mut self) {
        self.block_scope.pop();
    }

    /// Compile a native statement.
    fn compile_statement(&mut self, stmt: &Statement, is_pub: bool) {
        self.is_pub = is_pub;
        match stmt {
            Statement::VariableStatement(_, name, val_type, value, _) => {
                self.compile_variable_stmt(name, value, true);
            }
            // Statement::ConstVariableStatement(_, name, val_type, value, _) => {
            //     self.compile_variable_stmt(name, value, false);
            // }
            Statement::ExpressionStatement(_, expr) => {
                // add instructinos to current function
                let instructions = self.compile_expression(expr);
                if let Some(current_fn_ins) = self.instructions.get_mut(&self.next_fn_idx) {
                    current_fn_ins.append(&mut instructions.clone());
                }
            }
            Statement::ReturnStatement(token, expr) => {
                let mut instructions = self.compile_expression(expr);
                if let Some(current_fn_ins) = self.instructions.get_mut(&self.next_fn_idx) {
                    current_fn_ins.append(&mut instructions);
                    current_fn_ins.push(Instruction::Return);
                }
            }
            Statement::BlockStatement(_, stmts) => {
                self.is_currently_global = false;
                for stmt in stmts.as_ref() {
                    self.compile_statement(stmt, is_pub);
                }
            }
            _ => {
                // This stmt is not supported in native blocks (yet)
                self.errors
                    .push(native_unsupported_statement(stmt.get_token()));
                // self.add_error("Unsupported statement", stmt.get_token());
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

        // check if variable already exists in scope
        let mut easy_native_var: Option<&EasyNativeVar> = None;
        for var in self.variable_scope.iter().rev() {
            for v in var.iter() {
                if v.name == var_name {
                    easy_native_var = Some(v);
                    break;
                }
            }

            if easy_native_var.is_some() {
                break;
            }
        }

        // This variable already exists.
        if let Some(easy_native_var) = easy_native_var {
            // clone to appease the borrow checker
            let easy_native_var = easy_native_var.clone();
            // We should simply get and set the variable
            // First let's compiile the value
            let mut instructions = self.compile_expression(value);

            // Let's check if we have a function context.
            if let Some(current_fn_ins) = self.instructions.get_mut(&self.next_fn_idx) {
                current_fn_ins.append(&mut instructions);
                if easy_native_var.is_global {
                    current_fn_ins.push(Instruction::GlobalSet(easy_native_var.idx));
                } else {
                    current_fn_ins.push(Instruction::LocalSet(easy_native_var.idx));
                }
            } else {
                self.errors
                    .push(native_no_function_provided_for_variable_scope(
                        name.get_token(),
                    ));
            }

            return;
        }

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

            // Instance the instructions
            let mut value_instructions = self.compile_expression(value);

            // add to instructions
            if let Some(current_fn_ins) = self.instructions.get_mut(&self.next_fn_idx) {
                current_fn_ins.append(&mut value_instructions);
                current_fn_ins.push(Instruction::LocalSet(self.next_var_idx));
            } else {
                // panic!("Not sure what is supposed to happen here...");
                self.errors
                    .push(native_no_function_provided_for_variable_scope(
                        name.get_token(),
                    ));
                return;
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
                    self.errors
                        .push(native_unsupported_expression_as_value_for_global_variable(
                            value.get_token(),
                        ));
                }
                ConstExpr::empty()
            }
            _ => {
                // not supported
                self.errors
                    .push(native_unsupported_expression_as_value_for_global_variable(
                        value.get_token(),
                    ));
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
                self.errors
                    .push(native_error_compiling_identifier(expr.get_token(), name));
                vec![]
            }
            Expression::FunctionLiteral(_, name, params, val_type, body) => {
                self.compile_function_literal(name, params, val_type.as_ref().unwrap(), body);
                vec![]
                // self.instructions.iter().last().unwrap().1.clone()
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
                    (StrongValType::Int, StrongValType::Float) => "int",
                    (StrongValType::Float, StrongValType::Float) => "float",
                    (StrongValType::Float, StrongValType::Int) => "float",
                    (StrongValType::String, StrongValType::String) => "string",
                    (StrongValType::Bool, StrongValType::Bool) => "int",
                    (StrongValType::Int, StrongValType::Bool) => "int",
                    (StrongValType::Bool, StrongValType::Int) => "int",
                    (_, _) => {
                        self.errors.push(native_unsupported_operation(
                            expr.get_token(),
                            format!("{:#?}", left_type).as_str(),
                            op,
                            format!("{:#?}", right_type).as_str(),
                        ));
                        return vec![];
                    }
                };

                let mut instructions = vec![];
                instructions.append(&mut left.clone());
                instructions.append(&mut right.clone());

                match op.as_str() {
                    token::PLUS => match instruction_type {
                        "int" => instructions.push(Instruction::I32Add),
                        "float" => instructions.push(Instruction::F32Add),
                        "string" => instructions.push(Instruction::Call(STR_CONCAT_IDX)),
                        _ => {}
                    },
                    token::MINUS => match instruction_type {
                        "int" => instructions.push(Instruction::I32Sub),
                        "float" => instructions.push(Instruction::F32Sub),
                        _ => {}
                    },
                    token::ASTERISK => match instruction_type {
                        "int" => instructions.push(Instruction::I32Mul),
                        "float" => instructions.push(Instruction::F32Mul),
                        _ => {}
                    },
                    token::SLASH => match instruction_type {
                        "int" => instructions.push(Instruction::I32DivS),
                        "float" => instructions.push(Instruction::F32Div),
                        _ => {}
                    },
                    token::MODULUS => match instruction_type {
                        "int" => instructions.push(Instruction::I32RemS),
                        "float" => instructions.push(Instruction::I32RemS),
                        _ => {}
                    },
                    token::EQ => match instruction_type {
                        "int" => instructions.push(Instruction::I32Eq),
                        "float" => instructions.push(Instruction::F32Eq),
                        _ => {}
                    },
                    token::LT => match instruction_type {
                        "int" => instructions.push(Instruction::I32LtS),
                        "float" => instructions.push(Instruction::F32Lt),
                        _ => {}
                    },
                    token::LT_OR_EQ => match instruction_type {
                        "int" => instructions.push(Instruction::I32LeS),
                        "float" => instructions.push(Instruction::F32Le),
                        _ => {}
                    },
                    token::GT => match instruction_type {
                        "int" => instructions.push(Instruction::I32GtS),
                        "float" => instructions.push(Instruction::F32Gt),
                        _ => {}
                    },
                    token::GT_OR_EQ => match instruction_type {
                        "int" => instructions.push(Instruction::I32GeS),
                        "float" => instructions.push(Instruction::F32Ge),
                        _ => {}
                    },
                    token::PLUS_EQUALS => match instruction_type {
                        "int" => {
                            let mut n_assign = generate_n_assign_instructions!(left, Instruction::I32Add);
                            instructions.append(&mut n_assign);
                        }
                        "float" => {
                            let mut n_assign = generate_n_assign_instructions!(left, Instruction::F32Add);
                            instructions.append(&mut n_assign);
                        }
                        "string" => {
                            let mut n_assign = generate_n_assign_instructions!(left, Instruction::Call(STR_CONCAT_IDX));
                            instructions.append(&mut n_assign);
                        }
                        _ => {}
                    },
                    _ => {
                        self.errors
                            .push(native_unsupported_operator(expr.get_token(), op));
                        // self.add_error(
                        //     format!("native, Unsupported operator: {}", op).as_str(),
                        //     expr.get_token(),
                        // );
                        return vec![];
                    }
                }

                instructions
            }
            Expression::CallExpression(_, name, arguments) => {
                let name = self.compile_raw_expression(name);

                let fun_idx = self.get_fun_idx_from_name(&name);
                if fun_idx.is_none() {
                    // Check if this is a wasm core function
                    if !is_wasm_core(name.as_str()) {
                        self.errors
                            .push(native_could_not_parse_function(expr.get_token(), &name));
                        return vec![];
                    } else {
                        // We have a core function. Call it and pass back the instructions
                        return call_wasm_core_function(&mut self.errors, name.as_str(), arguments);
                    }
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
            Expression::DotExpression(tk, left, right) => {
                let mut instructions = vec![];

                // compile left side
                instructions.append(&mut self.compile_expression(left.as_ref()));
                // compile right side.
                instructions.append(&mut self.compile_expression(right.as_ref()));

                instructions
            }
            Expression::StringLiteral(_, literal) => {
                // add to the variable scope
                let string_var_idx = self.next_var_idx;
                // update idx
                self.next_var_idx += 1;
                self.variable_scope.get_mut(1).unwrap().push(EasyNativeVar {
                    name: format!("{}{}", literal, self.next_var_idx),
                    idx: string_var_idx,
                    is_global: false,
                    value: ConstExpr::empty(),
                    val_type: StrongValType::String,
                    is_mut: true,
                });
                set_local_string(string_var_idx, literal.to_owned())
            }
            Expression::IndexExpression(tk, left, index) => {
                let mut instructions = vec![];

                // get left type
                let left_type = self.get_val_type_from_expression(left.as_ref());
                // get index type
                let index_type = self.get_val_type_from_expression(index.as_ref());

                // compile left side
                instructions.append(&mut self.compile_expression(left));

                match left_type {
                    StrongValType::String => {
                        instructions.append(&mut self.compile_expression(index));
                        match index_type {
                            StrongValType::Int => {
                                // Call __str_index
                                instructions.push(Instruction::Call(STR_INDEX_IDX));
                            }
                            _ => self
                                .errors
                                .push(native_unsupported_index_expression(index.get_token())),
                        }
                    }
                    _ => self
                        .errors
                        .push(native_unsupported_index_expression(left.get_token())),
                }

                instructions
            }
            Expression::PrefixExpression(tk, prefix, right) => {
                // TODO: this and in get_val_type
                let mut instructions = vec![];

                // check prefix, it's gonna be eiter a BANG or a MINUS
                match prefix.as_str() {
                    // token::BANG => {}
                    token::MINUS => {
                        // Multiple the expression by -1 to convert the number into a negative.
                        instructions.append(&mut self.compile_expression(&right));
                        instructions
                            .append(&mut vec![Instruction::I32Const(-1), Instruction::I32Mul]);
                    }
                    _ => self
                        .errors
                        .push(native_unsupported_prefix_expression(tk, prefix)),
                }

                instructions
            }
            Expression::Boolean(tk, value) => {
                if *value {
                    vec![Instruction::I32Const(1)]
                } else {
                    vec![Instruction::I32Const(0)]
                }
            }
            Expression::IfExpression(token, condition, consequence, elif, else_block) => {
                // The index of the If instruction. Used for when doing If as expressions
                let mut current_instruction_index = 0;
                // If Expression requires the context to access the direct instructions for the current function.
                let mut compiled_condition = self.compile_expression(&condition);
                {
                    if let Some(instructions) = self.instructions.get_mut(&self.next_fn_idx) {
                        // add compiled condition
                        instructions.append(&mut compiled_condition);
                        current_instruction_index += instructions.len();
                        instructions.push(Instruction::If(BlockType::Empty));
                    } else {
                        self.errors
                            .push(native_if_expression_must_go_within_functions(token));
                        return vec![];
                    }
                }
                // compile consequence
                self.compile_statement(&consequence, self.is_pub);

                // Check for a elif
                if !elif.is_empty() {
                    // Let's add a else and compile the elif expression
                    if let Some(instructions) = self.instructions.get_mut(&self.next_fn_idx) {
                        // add ELSE instruction
                        instructions.push(Instruction::Else);
                    }
                    // Let's compile the elif expression
                    self.compile_expression(&elif);
                }

                // Check for a else
                if !else_block.is_empty() {
                    // Let's add a else block and compile the statement
                    if let Some(instructions) = self.instructions.get_mut(&self.next_fn_idx) {
                        instructions.push(Instruction::Else);
                    }
                    // Compile else statement
                    self.compile_statement(&else_block, self.is_pub);
                }
                // Add a end instruction
                if let Some(instructions) = self.instructions.get_mut(&self.next_fn_idx) {
                    instructions.push(Instruction::End);
                }

                // TODO: implement iife
                // if self.block_scope.len() > 0 {
                //     // check if and else block types
                //     let if_block_type = self.get_return_val_type_of_block(&consequence);
                //     let else_block_type = self.get_return_val_type_of_block(&else_block);

                //     // Check if they exist first of all
                // }

                // // Make sure they match the function return type
                // if (if_block_type.is_some() && if_block_type != self.current_fn_block.block_type)
                //     || (else_block_type.is_some()
                //         && else_block_type != self.current_fn_block.block_type)
                // {
                //     self.errors
                //         .push(native_return_value_does_not_match_function(token));
                //     return vec![];
                // }

                // // Check if the types are none or not
                // if let Some(block_type) = if_block_type {
                //     // We already have it
                //     self.insert_instruction_at(
                //         current_instruction_index,
                //         self.next_fn_idx,
                //         Instruction::If(BlockType::Result(block_type)),
                //     );
                // } else {
                //     // Let's check else_block
                //     if let Some(block_type) = else_block_type {
                //         // We have a block type
                //         self.insert_instruction_at(
                //             current_instruction_index,
                //             self.next_fn_idx,
                //             Instruction::If(BlockType::Result(block_type)),
                //         );
                //     }
                // }

                vec![]
            }
            Expression::IIFE(token, body) => {
                // IIFE contains a stmt so just call compile_stmt
                // But we do need to add a block scope
                self.add_block_scope();
                self.compile_statement(body, self.is_pub);
                // Now we can remove the block scope
                self.poop_block_scope();
                vec![]
            }
            Expression::EmptyExpression => {
                vec![]
            }
            _ => {
                self.errors.push(native_unsupported_expression(expr));
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
        // Get the block type
        // let block = {
        //     if let Some(return_val_type) = get_val_type_from_strong(&return_type) {
        //         BlockType::Result(return_val_type)
        //     } else {
        //         BlockType::Empty
        //     }
        // };
        self.instructions.insert(self.next_fn_idx, vec![]);
        // create current_fn_block
        // self.current_fn_block = EasyNativeBlock {
        //     idx: 0,
        //     block_type: get_val_type_from_strong(&return_type),
        //     strong_block_type: return_type.clone(),
        // };

        // add a new scope
        self.add_var_scope();

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
        self.next_var_idx += params.len() as u32;

        // all instructions have been added correctly.
        self.compile_statement(body, self.is_pub);

        // remove the scoped param variables
        for i in 0..params.len() {
            self.variable_scope.get_mut(1).unwrap().remove(0);
        }

        self.instructions
            .get_mut(&self.next_fn_idx)
            .unwrap()
            .append(&mut vec![
                Instruction::Unreachable, // To make sure if works
                Instruction::End,
            ]);

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
        self.pop_var_scope();
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
            Expression::StringLiteral(_, literal) => literal.to_owned(),
            _ => {
                // add error
                self.errors
                    .push(native_can_not_compile_raw_expression(expr.get_token()));
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
    /// - prefix expressions
    ///
    fn get_val_type_from_expression(&mut self, expr: &Expression) -> StrongValType {
        match expr {
            Expression::Identifier(tk, name) => {
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
                    self.errors
                        .push(native_can_not_get_value_from_expression(tk));
                    StrongValType::NotSupported
                } else {
                    res
                }
            }
            Expression::Type(tk, val_type) => get_param_type_by_string(val_type),
            Expression::IdentifierWithType(tk, name, val_type) => {
                self.get_val_type_from_expression(val_type.as_ref())
            }
            Expression::FunctionLiteral(tk, _, _, val_type, _) => {
                // There is no way to infer the return type of a function (not yet)
                // TODO: infer return type of function. also in JS
                if val_type.is_none() {
                    self.errors
                        .push(native_can_not_get_value_from_expression(tk));
                    StrongValType::NotSupported
                } else {
                    self.get_val_type_from_expression(val_type.clone().unwrap().as_ref())
                }
            }
            Expression::StringLiteral(tk, literal) => StrongValType::String,
            Expression::IntegerLiteral(tk, literal) => StrongValType::Int,
            Expression::PrefixExpression(tk, prefix, expression) => {
                self.get_val_type_from_expression(expression)
            }
            Expression::IndexExpression(tk, source, index) => {
                self.get_val_type_from_expression(source)
            }
            Expression::Boolean(_, _) => StrongValType::Bool,
            Expression::InfixExpression(tk, left, infix, right) => {
                self.get_val_type_from_expression(left)
            }
            Expression::FloatLiteral(tk, literal) => {
                StrongValType::Float                
            }
            _ => {
                // add error
                self.errors
                    .push(native_can_not_get_value_from_expression(expr.get_token()));
                StrongValType::NotSupported
            }
        }
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

    // /// Get variable idx from name
    // fn get_var_idx_from_name(&self, name: &str) -> Option<u32> {
    //     for scope in self.variable_scope.iter() {
    //         for var in scope.iter() {
    //             if var.name == *name {
    //                 return Some(var.idx);
    //             }
    //         }
    //     }
    //     None
    // }

    /// Insert a instruction at a specific in a specific function.
    ///
    /// This is used in:
    ///
    /// - if expressions
    fn insert_instruction_at(
        &mut self,
        index: usize,
        fn_index: u32,
        instruction: Instruction<'static>,
    ) {
        if let Some(instructions) = self.instructions.get_mut(&fn_index) {
            if index >= instructions.len() {
                return;
            }

            // Update
            instructions[index] = instruction;
        }
    }

    /// Get the StrongValType of a block.
    ///
    /// Only works for return statements.
    fn get_return_val_type_of_block(&mut self, block: &Statement) -> Option<ValType> {
        // Let's update our block type
        let final_stmt = block.get_final_stmt();
        let mut new_block_type: Option<ValType> = None;
        match final_stmt {
            Statement::ReturnStatement(_, expr) => {
                let strong_val_type = self.get_val_type_from_expression(expr);
                match strong_val_type {
                    StrongValType::Bool => {
                        new_block_type = Some(ValType::I32);
                    }
                    StrongValType::Float => {
                        new_block_type = Some(ValType::F32);
                    }
                    StrongValType::Int => {
                        new_block_type = Some(ValType::I32);
                    }
                    StrongValType::String => {
                        new_block_type = Some(ValType::I32);
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        new_block_type
    }
}
