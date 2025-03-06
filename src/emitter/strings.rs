use wasm_encoder::{BlockType, Function, Instruction, MemArg, TypeSection, ValType};

use super::{instruction_generator::{get_global, get_local, i32_store, i32_store_8, set_global, set_local_to_global}, signatures::{EasyNativeFN, FunctionSignature}, utils::StrongValType};

pub const GLOBAL_STRING_IDX: u32 = 0;

pub const ALLOCATE_STRING_IDX: u32 = 0;
pub const STORE_STRING_BYTE_IDX: u32 = 1;
pub const GET_STRING_LENGTH_IDX: u32 = 2;
pub const STORE_STRING_LENGTH_IDX: u32 = 3;
pub const CONCAT_STRING_IDX: u32 = 4;
const FREE_STRING_IDX: u32 = 5;

/// Create a function for allocationg strings in memory via easyjs native.
///
/// This function allocates the string in memory and returns a pointer to it.
pub fn allocate_string() -> EasyNativeFN {
    let locals = vec![(1, ValType::I32)]; // Local variable: the ptr
    let mut instructions = vec![];

    // set ptr to GLOBAL_STRING_IDX
    instructions.append(&mut set_local_to_global(1, GLOBAL_STRING_IDX));

    // get length of string
    instructions.append(&mut get_local(0));
    // add 4 to keep string length
    instructions.push(Instruction::I32Const(4));

    // add together
    instructions.push(Instruction::I32Add);

    // add current global
    instructions.append(&mut get_global(GLOBAL_STRING_IDX));
    instructions.push(Instruction::I32Add);

    // set global
    instructions.append(&mut set_global(GLOBAL_STRING_IDX));

    // return pointer
    instructions.append(&mut get_local(1));

    // Return original pointer (before allocation)
    instructions.push(Instruction::End);

    let mut function = Function::new(locals);
    for instruction in instructions {
        function.instruction(&instruction);
    }

    EasyNativeFN {
        signature: FunctionSignature {
            params: vec![ValType::I32], // the size of the string
            results: vec![ValType::I32],
            param_strong: vec![StrongValType::Int],
            results_strong: vec![StrongValType::Int]
        },
        function,
        name: "__str_alloc".to_string(),
        idx: ALLOCATE_STRING_IDX,
        is_public: true
    }
}

/// Create a function for storing the length of a string
pub fn store_string_length() -> EasyNativeFN {
    let locals = vec![];

    let mut instructions = vec![];

    // Get position and length
    instructions.append(&mut get_local(0));
    instructions.append(&mut get_local(1));

    // store it
    instructions.append(&mut i32_store(0,0,0));

    instructions.push(Instruction::End);

    let mut function = Function::new(locals);
    for instruction in instructions {
        function.instruction(&instruction);
    }

    EasyNativeFN {
        signature: FunctionSignature {
            params: vec![ValType::I32, ValType::I32], // position in memory size should go (ptr), size
            results: vec![],
            param_strong: vec![StrongValType::Int, StrongValType::Int],
            results_strong: vec![]
        },
        function,
        name:"__str_store_len".to_string(),
        idx: STORE_STRING_LENGTH_IDX,
        is_public: true
    }
}

/// Create a function for storing a byte in a string
pub fn store_string_byte() -> EasyNativeFN {
    let locals = vec![];

    let mut instructions = vec![];

    // Get position and byte
    instructions.append(&mut get_local(0));
    instructions.push(Instruction::I32Const(4)); // add 4 for length
    instructions.push(Instruction::I32Add);
    
    instructions.append(&mut get_local(1));

    // store it
    instructions.append(&mut i32_store_8(0,0,0));

    instructions.push(Instruction::End);

    let mut function = Function::new(locals);
    for instruction in instructions {
        function.instruction(&instruction);
    }

    EasyNativeFN {
        signature: FunctionSignature {
            params: vec![ValType::I32, ValType::I32], // position in memory byte should go, byte
            results: vec![],
            param_strong: vec![StrongValType::Int, StrongValType::Int],
            results_strong: vec![]
        },
        function,
        name:"__str_store_byte".to_string(),
        idx: STORE_STRING_BYTE_IDX,
        is_public: true
    }
}

/// Create a function for getting the length of a string
pub fn get_length_string() -> EasyNativeFN {
    let locals = vec![];
    let mut function = Function::new(locals);

    function.instruction(&Instruction::LocalGet(0)); // get the ptr
    function.instruction(&Instruction::I32Load8U(MemArg {
        align: 0,
        offset: 0,
        memory_index: 0,
    }));

    function.instruction(&Instruction::End);

    EasyNativeFN {
        signature: FunctionSignature {
            params: vec![ValType::I32],  // pointer to string
            results: vec![ValType::I32], // length
            param_strong: vec![StrongValType::String],
            results_strong: vec![StrongValType::Int]
        },
        function,
        name: "__str_len".to_string(),
        idx: GET_STRING_LENGTH_IDX,
        is_public: true
    }
}

pub fn concat_strings() -> EasyNativeFN {
    let locals = vec![(2, ValType::I32)]; // ptr1, ptr2

    EasyNativeFN {
        signature: FunctionSignature {
            params: vec![ValType::I32, ValType::I32], // ptr1, ptr2
            results: vec![ValType::I32], // ptr
            param_strong: vec![StrongValType::String, StrongValType::String],
            results_strong: vec![StrongValType::String]
        },
        function: Function::new(locals),
        name: "__str_concat".to_string(),
        idx: CONCAT_STRING_IDX,
        is_public: true
    }
}
pub fn free_string() {}