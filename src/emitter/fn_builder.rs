use std::collections::HashMap;

use wasm_encoder::{Instruction, ValType};

use crate::parser::ast::{Expression, Statement};

use super::{
    instruction_generator::{call, call_instruction, is_wasm_core, set_local_string}, strings::ALLOCATE_STRING_IDX, utils::{
        get_param_type_by_named_expression, get_param_type_by_string, get_val_type_from_strong, StrongValType
    }, variables::{WasmVariable, WasmVariables}
};

/// Handle building functions in easyjs to WebAssembly.
pub struct FNBuilder<'a> {
    /// Local Function variables. (includes paramaters)
    pub variables: WasmVariables,

    /// The locals
    locals: Vec<StrongValType>,
    /// The instructions
    instructions: Vec<Instruction<'a>>,

    // /// Map of variable names to local indices
    // locals_map: HashMap<String, (u32, bool)>,
    /// A reference to the EasyWasm struct
    easy_wasm: &'a EasyWasm,
}

impl<'a> FNBuilder<'a> {
    /// Create a new function builder.
    pub fn new(easy_wasm_ref: &'a EasyWasm) -> Self {
        FNBuilder {
            variables: WasmVariables::new(),
            instructions: Vec::new(),
            easy_wasm: easy_wasm_ref,
            locals: Vec::new(),
        }
    }

    /// Add a parameter to the function.
    pub fn add_param(&mut self, name: String, param: StrongValType) {
        self.variables.add_variable(name, param);
    }

    /// Add a local to the function.
    fn add_local(&mut self, name: String, local: StrongValType) {
        self.variables.add_variable(name, local.clone());
        self.locals.push(local);
    }

    /// Add an instruction to the function.
    fn add_instruction(&mut self, instruction: Instruction<'a>) {
        self.instructions.push(instruction);
    }

    /// Get a string representation of identifier expressions.
    ///
    /// Identifier.
    fn read_identifier(&mut self, expr: &'a Expression) -> String {
        match expr {
            Expression::Identifier(tk, ident) => ident.to_owned(),
            _ => {
                unimplemented!();
            }
        }
    }

    /// This method also sets the instruction for you.
    fn get_variable(&self, name: String) -> (WasmVariable, bool) {
        let pos_local = self.variables.get_variable_by_name(&name);
        // check local
        if let Some(pos_local) = pos_local {
            (pos_local, true)
        } else {
            // local not found, check global
            let pos_global = self.easy_wasm.variables.get_variable_by_name(&name);
            if let Some(pos_global) = pos_global {
                (pos_global, false)
            } else {
                // TODO: set an error
                unimplemented!();
            }
        }
    }

    fn get_val_type(&self, expr: &'a Expression) -> StrongValType {
        infer_variable_type(expr, &self.variables, &self.easy_wasm.type_registry_ref(), Some(&self.easy_wasm.variables))
    }

    /// This compiles an expression and sets instructions.
    fn compile_expression(&mut self, expr: &'a Expression) {
        match expr {
            Expression::IntegerLiteral(_, i) => {
                self.add_instruction(Instruction::I32Const(*i as i32));
            }
            Expression::FloatLiteral(_, f) => {
                self.add_instruction(Instruction::F32Const(*f as f32));
            }
            Expression::Identifier(_, name) => {
                let var = self.get_variable(name.to_owned());
                if var.1 {
                    self.add_instruction(Instruction::LocalGet(var.0.idx));
                } else {
                    self.add_instruction(Instruction::GlobalGet(var.0.idx));
                }
            }
            Expression::InfixExpression(_, left, op, right) => {
                self.compile_expression(left.as_ref());
                self.compile_expression(right.as_ref());

                // Depending on the strong type and the opperation this runs differently.
                let strong_type = self.get_val_type(left.as_ref());
                match strong_type {
                    (StrongValType::Int | StrongValType::Float) => {
                        let ty = get_val_type_from_strong(&strong_type).unwrap();
                        match (ty, op.as_str()) {
                            (ValType::I32, "+") => self.add_instruction(Instruction::I32Add),
                            (ValType::I32, "-") => self.add_instruction(Instruction::I32Sub),
                            (ValType::I32, "*") => self.add_instruction(Instruction::I32Mul),
                            (ValType::I32, "/") => self.add_instruction(Instruction::I32DivU),
                            (ValType::F32, "+") => self.add_instruction(Instruction::F32Add),
                            (ValType::F32, "-") => self.add_instruction(Instruction::F32Sub),
                            (ValType::F32, "*") => self.add_instruction(Instruction::F32Mul),
                            (ValType::F32, "/") => self.add_instruction(Instruction::F32Div),
                            _ => unimplemented!(),
                        }
                    }
                    StrongValType::Bool => {
                        // you can't add a boolean
                        unimplemented!("TODO: error for not being able to add a boolean");
                        // THROW_ERROR();
                    }
                    // StrongValType::String => {
                    //     // use concat for strings
                    //     self.add_instruction(call(CONCAT_STRING_IDX)[0].clone());
                    // }
                    _ => {
                        unimplemented!("TODO: error for unsupported type");
                    }
                }
            }
            Expression::CallExpression(_, name, args) => {
                let name = self.read_identifier(name.as_ref());

                // TODO: if the name is a native function but it exists in the code (use that instead). (Give a warning though)
                // check name is a native library function
                if is_wasm_core(&name) {
                    let instructions = call_instruction(&name, args.as_ref());
                    for instruction in instructions {
                        self.add_instruction(instruction);
                        return;
                    }
                }

                // add the arguments as instructions
                for arg in args.as_ref() {
                    self.compile_expression(arg);
                }
                // call the function
                self.add_instruction(Instruction::Call(self.easy_wasm.get_function_idx(&name)));
            }
            Expression::AssignExpression(_, left, right) => {
                let left = self.read_identifier(left.as_ref());
                self.compile_expression(right.as_ref());

                let var = self.get_variable(left);
                if var.1 {
                    self.add_instruction(Instruction::LocalSet(var.0.idx));
                } else {
                    self.add_instruction(Instruction::GlobalSet(var.0.idx));
                }
            }
            _ => {
                let instructions = make_instruction_for_value(expr);
                for instruction in instructions {
                    self.add_instruction(instruction);
                }
            }
        }
    }

    /// Compile a statement into a function.
    pub fn compile_statement(&mut self, stmt: &'a Statement) {
        match stmt {
            Statement::BlockStatement(_, stmts) => {
                for stmt in stmts.as_ref() {
                    self.compile_statement(stmt);
                }
            }
            Statement::VariableStatement(_, name, var_type, value, infer_type) => {
                let name = self.read_identifier(name.as_ref());
                let mut is_string = false;

                let var_type = {
                    if let Some(var_type) = var_type {
                        get_param_type_by_named_expression(var_type.as_ref().to_owned())
                    } else {
                        if !infer_type {
                            unimplemented!();
                        } else {
                            infer_variable_type(
                                value,
                                &self.variables,
                                &self.easy_wasm.type_registry_ref(),
                                Some(&self.easy_wasm.variables),
                            )
                        }
                    }
                };

                self.add_local(name.clone(), var_type);

                let wasm_var = self
                    .variables
                    .get_variable_by_name(&name)
                    .expect("Variable not found");

                if !is_string {
                    // Ensure the expression emits a value before storing it in the local
                    self.compile_expression(value.as_ref());

                    // Set the local variable
                    self.add_instruction(Instruction::LocalSet(wasm_var.idx));
                } else {
                    // a little special here...
                    // TODO: maybe this will be used for all memory types? [strings, arrays, structs, dictionaries, etc...]
                    match value.as_ref().to_owned() {
                        Expression::StringLiteral(_, v) => {
                            let instructions = set_local_string(wasm_var.idx, v);
                            for instruction in instructions {
                                self.add_instruction(instruction);
                            }
                        }
                        _ => {
                            unimplemented!();
                        }
                    }
                    // set_local_string(wasm_var.idx, string);
                }
            }
            Statement::ReturnStatement(_, expr) => {
                self.compile_expression(expr.as_ref());
                self.add_instruction(Instruction::Return);
            }
            Statement::ExpressionStatement(_, expr) => {
                self.compile_expression(expr);
            }
            _ => {
                self.add_instruction(Instruction::Nop);
            }
        }
    }

    /// Finish the function.
    /// This will return the locals and instructions.
    pub fn finish(&mut self) -> (Vec<(u32, ValType)>, Vec<Instruction<'a>>) {
        self.add_instruction(Instruction::End);
        let mut res = Vec::new();

        for local in self.locals.iter() {
            // Instead of grouping, store locals as they are declared
            res.push((1, get_val_type_from_strong(local).unwrap()));
        }

        // The instructions stay as they are
        let instructions = self.instructions.clone();

        // Clear the locals and instructions for reuse in future function builds
        self.locals.clear();
        self.instructions.clear();

        (res, instructions)
    }
}
