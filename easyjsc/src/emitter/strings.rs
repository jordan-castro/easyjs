use wasm_encoder::{BlockType, Function, Instruction, MemArg, TypeSection, ValType};

use crate::{emitter::builtins::{
    ALLOCATE_STRING_IDX, ALLOCATE_STRING_NAME, GLOBAL_STRING_IDX, STORE_STRING_LENGTH_IDX, STORE_STRING_LENGTH_NAME, STR_GET_LEN_IDX, STR_GET_LEN_NAME, STR_STORE_BYTE_IDX, STR_STORE_BYTE_NAME
}, new_function_with_instructions};

use super::{
    instruction_generator::{
        call, get_global, get_local, i32_store, i32_store_8, set_global, set_local,
        set_local_to_global,
    },
    signatures::{EasyNativeFN, FunctionSignature},
    utils::StrongValType,
};

// /// Generate a Native function for concating 2 strings together.
// pub fn naive_str_concat() -> EasyNativeFN {

// }

/// Create a function for allocationg strings in memory via easyjs native.
///
/// This function allocates the string in memory and returns a pointer to it.
pub fn allocate_string() -> EasyNativeFN {
    // params are: size(0) "the size of the string"
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
    instructions.append(&mut get_global(0));
    instructions.push(Instruction::I32Add);

    // set global
    instructions.append(&mut set_global(0));

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
            results_strong: vec![StrongValType::Int],
        },
        function,
        name: ALLOCATE_STRING_NAME.to_string(),
        idx: ALLOCATE_STRING_IDX,
        is_public: true,
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
    instructions.append(&mut i32_store(0, 0, 0));

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
            results_strong: vec![],
        },
        function,
        name: STORE_STRING_LENGTH_NAME.to_string(),
        idx: STORE_STRING_LENGTH_IDX,
        is_public: true,
    }
}
/// Generate a Native function for storing a byte to a string.
pub fn native_str_store_byte() -> EasyNativeFN {
    // Paramaters are ptr(0), current position(1), byte(2)
    // No local necessary
    let locals = vec![];

    let mut instructions = vec![];

    // get ptr
    instructions.append(&mut get_local(0));
    // get position
    instructions.append(&mut get_local(1));
    instructions.append(&mut vec![
        Instruction::I32Add,      // add to get ptr + position
        Instruction::LocalGet(2), // get byte
        Instruction::I32Store8(MemArg {
            offset: 0,
            align: 0,
            memory_index: 0,
        }), // store byte
        Instruction::End
    ]);

    let function = new_function_with_instructions!(locals, instructions);

    EasyNativeFN {
        signature: FunctionSignature {
            params: vec![ValType::I32, ValType::I32, ValType::I32],
            results: vec![],
            param_strong: vec![StrongValType::Int, StrongValType::Int, StrongValType::Int],
            results_strong: vec![],
        },
        function: function,
        name: STR_STORE_BYTE_NAME.to_string(),
        idx: STR_STORE_BYTE_IDX,
        is_public: true,
    }
}

/// Generate a Native function for getting the string length.
pub fn native_str_get_len() -> EasyNativeFN {
    // Paramater variables: ptr(0) "Pointer to string"
    // Local variables None
    let locals = vec![];
    let mut instructions = vec![];

    // Get ptr
    instructions.append(&mut get_local(0));
    instructions.append(&mut vec![
        Instruction::I32Load(MemArg {
            offset: 0,
            align: 0,
            memory_index: 0,
        }), // load memory
        Instruction::End,         // return the loaded value
    ]);

    let function = new_function_with_instructions!(locals, instructions); 

    EasyNativeFN {
        signature: FunctionSignature {
            params: vec![ValType::I32],  // this is the ptr
            results: vec![ValType::I32], // string length which is the result of I32Load
            param_strong: vec![StrongValType::Int],
            results_strong: vec![StrongValType::Int],
        },
        function,
        name: STR_GET_LEN_NAME.to_string(),
        idx: STR_GET_LEN_IDX,
        is_public: true,
    }
}

/// Generate a native function that will instance a new string.
///
/// This handles everything from allocating, adding bytes, etc.
pub fn native_str_new() -> EasyNativeFN {
    unimplemented!("This still needs to be implemented")
}