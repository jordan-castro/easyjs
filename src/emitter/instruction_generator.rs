use wasm_encoder::{Instruction, MemArg, ValType};

use crate::{emitter::strings::ALLOCATE_STRING_IDX, parser::ast::Expression};

use super::strings::STORE_STRING_LENGTH_IDX;

pub type EasyInstructions = Vec<Instruction<'static>>;

/// Is a function a core wasm function?
pub fn is_wasm_core(fn_name: &str) -> bool {
    match fn_name {
        "__new_ptr" => true,
        "__i32_store" => true,
        "__i32_store_16" => true,
        "__i32_store_8" => true,
        "__get_local" => true,
        "__set_local" => true,
        "__add_i32" => true,
        "__i32_add" => true,
        _ => false,
    }
}

/// Call the core wasm function.
pub fn call_instruction(name: &str, arguments: &Vec<Expression>) -> EasyInstructions {
    match name {
        "__new_ptr" => {
            let at = match arguments[0] {
                Expression::IntegerLiteral(_, at) => at as i32,
                _ => panic!("Expected number as argument for __new_ptr"),
            };
            new_ptr(at)
        }
        // i32_store(align: u32, offset: u64, memory_index: u32)
        "__i32_store" => {
            let mut args = vec![];
            for arg in arguments {
                match arg {
                    Expression::IntegerLiteral(_, value) => {
                        args.push(*value);
                    }
                    _ => panic!("Expected number as argument for __i32_store"),
                }
            }

            i32_store(args[0] as u32, args[1] as u64, args[2] as u32)
        }
        // i32_store_16(align: u32, offset: u64, memory_index: u32)
        "__i32_store_16" => {
            let mut args = vec![];
            for arg in arguments {
                match arg {
                    Expression::IntegerLiteral(_, value) => {
                        args.push(*value);
                    }
                    _ => panic!("Expected number as argument for __i32_store_16"),
                }
            }

            i32_store_16(args[0] as u32, args[1] as u64, args[2] as u32)
        }
        // i32_store_8(align: u32, offset: u64, memory_index: u32)
        "__i32_store_8" => {
            let mut args = vec![];
            for arg in arguments {
                match arg {
                    Expression::IntegerLiteral(_, value) => {
                        args.push(*value);
                    }
                    _ => panic!("Expected number as argument for __i32_store_8"),
                }
            }

            i32_store_8(args[0] as u32, args[1] as u64, args[2] as u32)
        }
        "__get_local" => {
            let idx = match arguments[0] {
                Expression::IntegerLiteral(_, idx) => idx as u32,
                _ => panic!("Expected number as argument for __get_local"),
            };
            get_local(idx)
        }
        "__set_local" => {
            let idx = match arguments[0] {
                Expression::IntegerLiteral(_, idx) => idx as u32,
                _ => panic!("Expected number as argument for __set_local"),
            };
            set_local(idx)
        }
        "__add_i32" => {
            let numbers = arguments.iter().map(|arg| match arg {
                Expression::IntegerLiteral(_, value) => *value as i32,
                _ => panic!("Expected number as argument for __add_i32"),
            }).collect();
            add_i32(numbers)
        }
        // sometimes basic instructions need to be called
        "__i32_add" => {
            vec![Instruction::I32Add]
        }
        "__f32_add" => {
            vec![Instruction::F32Add]
        }
        _ => {
            vec![Instruction::Unreachable]
        }
    }
}

pub fn new_ptr(at: i32) -> EasyInstructions {
    vec![Instruction::I32Const(at)]
}

pub fn set_local_string(idx: u32, string: String) -> EasyInstructions {
    let str_length = string.len() as i32;
    let str_bytes = string.as_bytes();

    let mut instructions = vec![
        // Step 1. Allocate memory for string...
        // const string length
        Instruction::I32Const(str_length),
        // allocate memory
        call(ALLOCATE_STRING_IDX)[0].to_owned(),
        // set local variable to pointer
        set_local(idx)[0].to_owned(),
        // Step 2. Store string length at the start of the allocated memory.
        get_local(idx)[0].to_owned(),
        Instruction::I32Const(str_length),
        call(STORE_STRING_LENGTH_IDX)[0].to_owned(),
    ];

    // Step 3. loop through all bytes and stor them. Offset them by 4
    let offset = 4;

    for (i, byte) in str_bytes.iter().enumerate() {
        // get local pointer
        instructions.push(get_local(idx)[0].to_owned());
        // add offset
        instructions.push(Instruction::I32Const(i as i32 + offset as i32));
        instructions.push(Instruction::I32Add);
        // set byte
        instructions.push(Instruction::I32Const(*byte as i32));
        // store byte
        instructions.push(i32_store_8(0, 0, 0)[0].to_owned());
    }

    instructions
}

/// Add any number of i32s together.
pub fn add_i32(numbers: Vec<i32>) -> EasyInstructions {
    let mut instructions = Vec::new();
    for number in numbers {
        instructions.push(Instruction::I32Const(number));
    }

    instructions.push(Instruction::I32Add);

    instructions
}

/// Add any number of f32s together.
pub fn add_f32(numbers: Vec<f32>) -> EasyInstructions {
    let mut instructions = Vec::new();
    for number in numbers {
        instructions.push(Instruction::F32Const(number));
    }

    instructions.push(Instruction::F32Add);

    instructions
}

/// Get a global variable by index
pub fn get_global(idx: u32) -> EasyInstructions {
    vec![Instruction::GlobalGet(idx)]
}

pub fn set_global(idx: u32) -> EasyInstructions {
    vec![Instruction::GlobalSet(idx)]
}

pub fn get_local(idx: u32) -> EasyInstructions {
    vec![Instruction::LocalGet(idx)]
}

pub fn set_local(idx: u32) -> EasyInstructions {
    vec![Instruction::LocalSet(idx)]
}

pub fn set_local_to_local(idx: u32, value: u32) -> EasyInstructions {
    vec![get_local(value)[0].to_owned(), set_local(idx)[0].to_owned()]
}

pub fn set_local_to_global(local_idx: u32, global_idx: u32) -> EasyInstructions {
    vec![
        get_global(global_idx)[0].to_owned(),
        set_local(local_idx)[0].to_owned(),
    ]
}

pub fn set_local_to_i32(idx: u32, value: i32) -> EasyInstructions {
    vec![Instruction::I32Const(value), set_local(idx)[0].to_owned()]
}

pub fn set_local_to_f32(idx: u32, value: f32) -> EasyInstructions {
    vec![Instruction::F32Const(value), set_local(idx)[0].to_owned()]
}

pub fn set_global_to_global(idx: u32, value: u32) -> EasyInstructions {
    vec![
        get_global(value)[0].to_owned(),
        set_global(idx)[0].to_owned(),
    ]
}

pub fn set_global_to_local(idx: u32, value: u32) -> EasyInstructions {
    vec![
        get_local(value)[0].to_owned(),
        set_global(idx)[0].to_owned(),
    ]
}

pub fn set_global_to_i32(idx: u32, value: i32) -> EasyInstructions {
    vec![Instruction::I32Const(value), set_global(idx)[0].to_owned()]
}

pub fn set_global_to_f32(idx: u32, value: f32) -> EasyInstructions {
    vec![Instruction::F32Const(value), set_global(idx)[0].to_owned()]
}

pub fn call(idx: u32) -> EasyInstructions {
    vec![Instruction::Call(idx)]
}

pub fn i32_store(align: u32, offset: u64, memory_index: u32) -> EasyInstructions {
    vec![Instruction::I32Store(MemArg {
        align,
        offset,
        memory_index,
    })]
}

pub fn i32_store_16(align: u32, offset: u64, memory_index: u32) -> EasyInstructions {
    vec![Instruction::I32Store16(MemArg {
        align,
        offset,
        memory_index,
    })]
}

pub fn i32_store_8(align: u32, offset: u64, memory_index: u32) -> EasyInstructions {
    vec![Instruction::I32Store8(MemArg {
        align,
        offset,
        memory_index,
    })]
}

pub fn i32_load_8u(align: u32, offset: u64, memory_index: u32) -> EasyInstructions {
    vec![Instruction::I32Load8U(MemArg {
        align,
        offset,
        memory_index,
    })]
}