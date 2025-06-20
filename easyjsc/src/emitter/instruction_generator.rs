use wasm_encoder::{Instruction, MemArg, ValType};

use crate::{
    emitter::builtins::{ALLOCATE_STRING_IDX, STORE_STRING_LENGTH_IDX, STR_STORE_BYTE_IDX},
    parser::ast::Expression,
};

/// Macro for creating a new wasm function with instructions.
#[macro_export]
macro_rules! new_function_with_instructions {
    ($locals:expr, $instructions: expr) => {{
        let mut function = Function::new($locals);
        for instruction in $instructions {
            function.instruction(&instruction);
        }
        function
    }};
}

/// Instructions for setting a string byte within a loop
/// 
/// `loop_index: u32` This is the idx of the loop index.
/// 
/// `position: u32` This is the idx of the position variable. This will be set and get.
/// 
/// `from_string_ptr: u32` This is the idx of the ptr of the string from which we are loading the byte.
/// 
/// `byte: u32` The idx of the byte variable. This will be get and set.
/// 
/// `to_string_ptr: u32` The idx of the ptr of the strign to which we are setting the byte.
#[macro_export]
macro_rules! set_string_byte_in_loop {
    ($loop_index: expr, $position: expr, $from_string_ptr: expr, $byte: expr, $to_string_ptr: expr) => {
        vec![
        Instruction::LocalGet($loop_index),
        Instruction::I32Const(4),
        Instruction::I32Add,
        Instruction::LocalSet($position),
        // Set up for byte
        Instruction::LocalGet($position),
        Instruction::LocalGet($from_string_ptr),
        Instruction::I32Add,
        // Get byte
        Instruction::I32Load(MemArg {
            offset: 0,
            align: 0,
            memory_index: 0,
        }),
        // set local byte
        Instruction::LocalSet($byte),
        // setup for __str_store_byte
        Instruction::LocalGet($to_string_ptr),
        Instruction::LocalGet($position),
        Instruction::LocalGet($byte),
        // call __str_store_byte
        Instruction::Call(STR_STORE_BYTE_IDX),
        ]
    };
} 


pub type EasyInstructions = Vec<Instruction<'static>>;

/// Is a function a core wasm function?
pub fn is_wasm_core(fn_name: &str) -> bool {
    match fn_name {
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
            let numbers = arguments
                .iter()
                .map(|arg| match arg {
                    Expression::IntegerLiteral(_, value) => *value as i32,
                    _ => panic!("Expected number as argument for __add_i32"),
                })
                .collect();
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

/// Instructions for setting strings localy (i.e. AOT)
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

    instructions.push(Instruction::LocalGet(idx));
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

/// Set whatever came before to global IDX
pub fn set_global(idx: u32) -> EasyInstructions {
    vec![Instruction::GlobalSet(idx)]
}

/// Get a local by idx
pub fn get_local(idx: u32) -> EasyInstructions {
    vec![Instruction::LocalGet(idx)]
}

/// Set whatever came before to the following local idx
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

pub fn i32_load(align: u32, offset: u64, memory_index: u32) -> EasyInstructions {
    vec![Instruction::I32Load(MemArg {
        offset,
        align,
        memory_index,
    })]
}
