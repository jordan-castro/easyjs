use std::collections::HashMap;

use wasm_encoder::{Instruction, ValType};

use crate::parser::ast::{Expression, Statement};

use super::utils::get_param_type;

/// Handle building functions in easyjs to WebAssembly.
pub struct FNBuilder<'a> {
    /// Parameters
    params: Vec<ValType>,

    /// The locals
    locals: Vec<ValType>,
    /// The instructions
    instructions: Vec<Instruction<'a>>,

    /// Map of variable names to local indices
    locals_map: HashMap<String, u32>,
}

impl<'a> FNBuilder<'a> {
    /// Create a new function builder.
    pub fn new() -> Self {
        FNBuilder {
            params: Vec::new(),
            locals: Vec::new(),
            instructions: Vec::new(),
            locals_map: HashMap::new(),
        }
    }
    /// Add a parameter to the function.
    pub fn add_param(&mut self, name:String, param: ValType) {
        self.params.push(param);

        // Add the parameter to the locals map
        let idx = self.locals_map.len() as u32;
        self.locals_map.insert(name, idx);
    }

    /// Add a local to the function.
    fn add_local(&mut self, name:String, local: ValType) {
        self.locals.push(local);

        // Add the local to the locals map
        let idx = self.locals_map.len() as u32;
        self.locals_map.insert(name, idx);
    }

    /// Add an instruction to the function.
    fn add_instruction(&mut self, instruction: Instruction<'a>) {
        self.instructions.push(instruction);
    }

    fn read_expression(&mut self, expr: &'a Expression) -> String {
        match expr {
            Expression::Identifier(tk, ident) => {
                ident.to_owned()
            }
            _ => {
                unimplemented!();
            }
        }
    }

    /// This compiles an expression and sets instructions.
    fn compile_expression(&mut self, expr: &'a Expression) {
        match expr {
            Expression::InfixExpression(_, left, op, right) => {
                let left = self.read_expression(left.as_ref());
                let right = self.read_expression(right.as_ref());

                // get local
                let left = self.locals_map.get(&left).expect("Local not found").to_owned();
                let right = self.locals_map.get(&right).expect("Local not found").to_owned();

                self.add_instruction(Instruction::LocalGet(left));
                self.add_instruction(Instruction::LocalGet(right));

                // check valtype
                let val_type = {
                    if let Some(val_type) = self.locals.get(left as usize) {
                        val_type
                    } else {
                        panic!("Local not found");
                    }   
                };

                match (val_type, op.as_str()) {
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
                        unimplemented!();
                    }
                }

            }
            _ => {
                unimplemented!();
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
            Statement::VariableStatement(_, name, var_type, value) => {
                let name = self.read_expression(name.as_ref());

                let var_type = var_type.clone().expect("Variable type not found").as_ref().to_owned();
                self.add_local(name, get_param_type(var_type));

                // instruction to set the local
                self.compile_expression(value.as_ref());
            }
            Statement::ReturnStatement(_, expr) => {
                // self.compile_expression(expr.as_ref());
                self.add_instruction(Instruction::Return);
            }
            _ => {
                unimplemented!();
            }
        }
    }

    /// Finish the function.
    /// This will return the locals and instructions.
    pub fn finish(&mut self) -> (Vec<(u32, ValType)>, Vec<Instruction<'a>>) {
        self.add_instruction(Instruction::End);
        let mut res = Vec::new();

        // Pair locals with the amount of each type.
        for local in self.locals.iter() {
            let count = res.iter_mut().find(|(count, ty)| ty == local);
            if let Some((count, _)) = count {
                *count += 1;
            } else {
                res.push((1, *local));
            }
        }

        // The instructions stay as they are
        let instructions = self.instructions.clone();
    
        // Clear the locals and instructions for reuse in future function builds
        self.locals.clear();
        self.instructions.clear();
    
        (res, instructions)
    }
}