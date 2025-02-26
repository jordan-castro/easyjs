use std::collections::HashMap;

use wasm_encoder::{Instruction, ValType};

use crate::parser::ast::{Expression, Statement};

use super::{
    utils::{ get_param_type_by_named_expression, get_param_type_by_string, infer_variable_type, make_instruction_for_value, StrongValType},
    variables::{WasmVariable, WasmVariables},
    wasm_emitter::EasyWasm,
};

/// Handle building functions in easyjs to WebAssembly.
pub struct FNBuilder<'a> {
    /// Local Function variables. (includes paramaters)
    pub variables: WasmVariables,

    /// The locals
    locals: Vec<ValType>,
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
    pub fn add_param(&mut self, name: String, param: ValType) {
        self.variables.add_variable(name, param);
    }

    /// Add a local to the function.
    fn add_local(&mut self, name: String, local: ValType) {
        self.variables.add_variable(name, local);
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

    fn get_val_type(&self, expr: &'a Expression) -> ValType {
        match expr {
            Expression::IntegerLiteral(_, _) => ValType::I32,
            Expression::FloatLiteral(_, _) => ValType::F32,
            Expression::Identifier(_, name) => {
                // it's a variable...
                self.get_variable(name.to_owned()).0.ty
            },
            Expression::Type(_, ty) => {
                let t = get_param_type_by_string(ty.to_owned());
                match t {
                    Some(v) => v,
                    None => unimplemented!()
                }
            }
            _ => {
                unimplemented!();
            }
        }
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

                let ty = self.get_val_type(left.as_ref());

                match (ty, op.as_str()) {
                    (ValType::I32, "+") => self.add_instruction(Instruction::I32Add),
                    (ValType::I32, "-") => self.add_instruction(Instruction::I32Sub),
                    (ValType::I32, "*") => self.add_instruction(Instruction::I32Mul),
                    (ValType::I32, "/") => self.add_instruction(Instruction::I32DivU),

                    (ValType::I64, "+") => self.add_instruction(Instruction::I64Add),
                    (ValType::I64, "-") => self.add_instruction(Instruction::I64Sub),
                    (ValType::I64, "*") => self.add_instruction(Instruction::I64Mul),
                    (ValType::I64, "/") => self.add_instruction(Instruction::I64DivU),

                    (ValType::F32, "+") => self.add_instruction(Instruction::F32Add),
                    (ValType::F32, "-") => self.add_instruction(Instruction::F32Sub),
                    (ValType::F32, "*") => self.add_instruction(Instruction::F32Mul),
                    (ValType::F32, "/") => self.add_instruction(Instruction::F32Div),

                    (ValType::F64, "+") => self.add_instruction(Instruction::F64Add),
                    (ValType::F64, "-") => self.add_instruction(Instruction::F64Sub),
                    (ValType::F64, "*") => self.add_instruction(Instruction::F64Mul),
                    (ValType::F64, "/") => self.add_instruction(Instruction::F64Div),
                    _ => {
                        self.add_instruction(Instruction::Nop);
                    }
                }
            }
            Expression::CallExpression(_, name, args) => {
                let name = self.read_identifier(name.as_ref());

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

                let var_type = {
                    if let Some(var_type) = var_type {
                        get_param_type_by_named_expression(var_type.as_ref().to_owned())
                    } else {
                        if !infer_type {
                            unimplemented!();
                        } else {
                            infer_variable_type(value, &self.variables, &self.easy_wasm.type_registry_ref())
                        }
                    }
                };

                let var_type = match var_type {
                    StrongValType::Some(v) => {
                        v
                    }
                    _ => {
                        unimplemented!();
                    }
                };

                self.add_local(name.clone(), var_type);

                let wasm_var = self
                    .variables
                    .get_variable_by_name(&name)
                    .expect("Variable not found");

                // Ensure the expression emits a value before storing it in the local
                self.compile_expression(value.as_ref());

                // Set the local variable
                self.add_instruction(Instruction::LocalSet(wasm_var.idx));
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
            res.push((1, *local));
        }
        
        // The instructions stay as they are
        let instructions = self.instructions.clone();

        // Clear the locals and instructions for reuse in future function builds
        self.locals.clear();
        self.instructions.clear();

        (res, instructions)
    }
}
