use wasm_encoder::{BlockType, Function, Instruction, MemArg, TypeSection, ValType};

use crate::{
    emitter::builtins::{
        ALLOCATE_STRING_IDX, ALLOCATE_STRING_NAME, GLOBAL_STRING_IDX, STORE_STRING_LENGTH_IDX,
        STORE_STRING_LENGTH_NAME, STR_CHAR_CODE_AT_IDX, STR_CHAR_CODE_AT_NAME, STR_CONCAT_IDX,
        STR_CONCAT_NAME, STR_GET_LEN_IDX, STR_GET_LEN_NAME, STR_INDEX_IDX, STR_INDEX_NAME,
        STR_STORE_BYTE_IDX, STR_STORE_BYTE_NAME,
    },
    new_function_with_instructions, set_string_byte_in_loop,
};

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
        Instruction::End,
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
        Instruction::End, // return the loaded value
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

/// Generate a native function for concating 2 strings together.
pub fn native_str_concat() -> EasyNativeFN {
    // params
    let n1: u32 = 0; // string ptr 1
    let n2: u32 = 1; // string ptr 2

    // locals
    let ptr: u32 = 2;
    let n1_length: u32 = 3;
    let n2_length: u32 = 4;
    let n_length: u32 = 5;
    let loop_index: u32 = 6;
    let position: u32 = 7;
    let byte: u32 = 8;

    // pseudo code
    /**
     * n1_length = len(n1)
     * n2_length = len(n2)
     *
     * n_length = i32_add(n1_length, n2_length)
     *
     * ptr = alloc(n_length)
     *
     * for i in n_length
     *      if i < n1_length
     *          position = 4 + i
     *          byte = i_load(n1 + position)
     *          __str_store_byte(ptr, position, byte)
     *      if i > n1_length
     *          position = 4 + i
     *          byte_position = position - n1_length
     *          byte = i_load(n2 + byte_position)
     *          __str_store_byte(ptr, position, byte)
     * return ptr
     */
    let locals = vec![(7, ValType::I32)];

    let mut instructions = vec![];

    instructions.append(&mut vec![
        // Get length of n1
        Instruction::LocalGet(n1),
        Instruction::Call(STR_GET_LEN_IDX),
        Instruction::LocalSet(n1_length),
        // Get length of n2
        Instruction::LocalGet(n2),
        Instruction::Call(STR_GET_LEN_IDX),
        Instruction::LocalSet(n2_length),
        Instruction::LocalGet(n1_length),
        Instruction::LocalGet(n2_length),
        // Add together
        Instruction::I32Add,
        Instruction::LocalSet(n_length),
        Instruction::LocalGet(n_length),
        // take in to account the length diff
        Instruction::I32Const(8),
        Instruction::I32Add,
        // call allocate
        Instruction::Call(ALLOCATE_STRING_IDX),
        // set to ptr
        Instruction::LocalSet(ptr),
        // store string length
        Instruction::LocalGet(ptr),
        Instruction::LocalGet(n_length),
        Instruction::Call(STORE_STRING_LENGTH_IDX),
        // Add bytes
        // set loop_index to -1
        Instruction::I32Const(-1),
        Instruction::LocalSet(loop_index),
        // start loop
        Instruction::Loop(BlockType::Empty),
        // add to index
        Instruction::LocalGet(loop_index),
        Instruction::I32Const(1),
        Instruction::I32Add,
        Instruction::LocalSet(loop_index),
        // Check which ptr we are loading
        Instruction::LocalGet(loop_index), // i < n1_length
        Instruction::LocalGet(n1_length),
        Instruction::I32LtS,
        // If block for loading from n1 ptr
        Instruction::If(BlockType::Empty),
        // setup position
        Instruction::LocalGet(loop_index),
        Instruction::I32Const(4),
        Instruction::I32Add,
        Instruction::LocalSet(position),
        // Set up for byte
        Instruction::LocalGet(position),
        Instruction::LocalGet(n1),
        Instruction::I32Add,
        // Get byte
        Instruction::I32Load(MemArg {
            offset: 0,
            align: 0,
            memory_index: 0,
        }),
        // set local byte
        Instruction::LocalSet(byte),
        // setup for __str_store_byte
        Instruction::LocalGet(ptr),
        Instruction::LocalGet(position),
        Instruction::LocalGet(byte),
        // call __str_store_byte
        Instruction::Call(STR_STORE_BYTE_IDX),
        Instruction::Else,
        // we are loading n2 ptr
        // setup position
        Instruction::LocalGet(loop_index),
        Instruction::I32Const(4),
        Instruction::I32Add,
        Instruction::LocalSet(position),
        // Set up for byte
        Instruction::LocalGet(position),
        Instruction::LocalGet(n1_length),
        // subtract n1_length to get correct index in n2
        Instruction::I32Sub,
        Instruction::LocalGet(n2),
        Instruction::I32Add,
        // Get byte
        Instruction::I32Load(MemArg {
            offset: 0,
            align: 0,
            memory_index: 0,
        }),
        // set local byte
        Instruction::LocalSet(byte),
        // setup for __str_store_byte
        Instruction::LocalGet(ptr),
        Instruction::LocalGet(position),
        Instruction::LocalGet(byte),
        // call __str_store_byte
        Instruction::Call(STR_STORE_BYTE_IDX),
        Instruction::End, // close if/else stmt
        // check size
        Instruction::LocalGet(loop_index),
        Instruction::LocalGet(n_length),
        Instruction::I32LtS,
        Instruction::BrIf(0),
        Instruction::End,
        Instruction::LocalGet(ptr),
        Instruction::End, // function
    ]);

    let function = new_function_with_instructions!(locals, instructions);

    EasyNativeFN {
        signature: FunctionSignature {
            params: vec![ValType::I32, ValType::I32],
            results: vec![ValType::I32],
            param_strong: vec![StrongValType::String, StrongValType::String],
            results_strong: vec![StrongValType::String],
        },
        function,
        name: STR_CONCAT_NAME.to_string(),
        idx: STR_CONCAT_IDX,
        is_public: true,
    }
}

/// Generate a command
pub fn native_str_index() -> EasyNativeFN {
    // params
    let ptr: u32 = 0;
    let index: u32 = 1;

    // locals
    let n_ptr: u32 = 2;
    let n_byte: u32 = 3;
    let byte_position: u32 = 4;

    // locals
    let locals = vec![(3, ValType::I32)];

    /**
     * pseudo code
     * 
     * // check index
     * if index < 0 {
     *      byte_position = __str_get_len(ptr) + index
     * } else {
     *      byte_position = index
     * }
     * 
     * byte_position = ptr + 4
     * 
     * i32_load(byte_position)
     * 
     * // the rest which should work fine...
     */

    let instructions = vec![
        // set byte pos to 0
        // // get ptr
        // Instruction::LocalGet(ptr),
        // // add 4
        // Instruction::I32Const(4),
        // Instruction::I32Add,
        // // Set byte position
        // Instruction::LocalSet(byte_position),
        
        // Check if index < 0
        Instruction::LocalGet(index),
        Instruction::I32Const(0),
        Instruction::I32LtS,
        Instruction::If(BlockType::Empty),

        // update byte position based on length + (-index)
        Instruction::LocalGet(ptr),
        Instruction::Call(STR_GET_LEN_IDX),
        Instruction::LocalGet(index),
        Instruction::I32Add,
        Instruction::LocalSet(byte_position),

        Instruction::Else,
        Instruction::LocalGet(index),
        Instruction::LocalSet(byte_position),
        
        Instruction::End,
        Instruction::LocalGet(byte_position),
        Instruction::LocalGet(ptr),
        Instruction::I32Add,
        Instruction::I32Const(4),
        Instruction::I32Add,
        // Instruction::I32Add,
        // load memory
        Instruction::I32Load(MemArg {
            offset: 0,
            align: 0,
            memory_index: 0,
        }),
        // set it to a local byte
        Instruction::LocalSet(n_byte),
        // size of new string
        Instruction::I32Const(1),
        // allocate
        Instruction::Call(ALLOCATE_STRING_IDX),
        Instruction::LocalSet(n_ptr),
        // store length
        Instruction::LocalGet(n_ptr),
        Instruction::I32Const(1),
        Instruction::Call(STORE_STRING_LENGTH_IDX),
        // store byte
        Instruction::LocalGet(n_ptr),
        Instruction::I32Const(4),
        Instruction::LocalGet(n_byte),
        Instruction::Call(STR_STORE_BYTE_IDX),
        // return pointer
        Instruction::LocalGet(n_ptr),
        // Create a new
        Instruction::End,
    ];

    EasyNativeFN {
        signature: FunctionSignature {
            params: vec![ValType::I32, ValType::I32],
            results: vec![ValType::I32],
            param_strong: vec![StrongValType::String, StrongValType::Int],
            results_strong: vec![StrongValType::String],
        },
        function: new_function_with_instructions!(locals, instructions),
        name: STR_INDEX_NAME.to_string(),
        idx: STR_INDEX_IDX,
        is_public: true,
    }
}

/// Generate a Native function for getting the charCodeAt on a string
pub fn native_str_char_code_at() -> EasyNativeFN {
    let ptr: u32 = 0;
    let index: u32 = 1;

    let locals = vec![];

    let instructions = vec![
        // i32load(ptr + offset i.e. 4 + index)
        Instruction::LocalGet(ptr),
        Instruction::I32Const(4),
        Instruction::I32Add,
        Instruction::LocalGet(index),
        Instruction::I32Add,
        Instruction::I32Load8U(MemArg {
            offset: 0,
            align: 0,
            memory_index: 0,
        }),
        Instruction::End,
    ];

    EasyNativeFN {
        signature: FunctionSignature {
            params: vec![ValType::I32, ValType::I32],
            results: vec![ValType::I32],
            param_strong: vec![StrongValType::String, StrongValType::Int],
            results_strong: vec![StrongValType::Int],
        },
        function: new_function_with_instructions!(locals, instructions),
        name: STR_CHAR_CODE_AT_NAME.to_string(),
        idx: STR_CHAR_CODE_AT_IDX,
        is_public: true,
    }
}
