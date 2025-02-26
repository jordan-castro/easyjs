use wasm_encoder::{BlockType, Function, Instruction, MemArg, TypeSection, ValType};

use super::signatures::{EasyNativeFN, FunctionSignature};

pub const GLOBAL_STRING_OFFSET: u32 = 1024;
const GLOBAL_STRING_IDX: u32 = 0;

pub const ALLOCATE_STRING_IDX: u32 = 0;
// pub const STORE_STRING_IDX: u32 = 1;
pub const GET_STRING_LENGTH_IDX: u32 = 1;
const GET_STRING_IDX: u32 = 2;
const CONCAT_STRING_IDX: u32 = 3;
const FREE_STRING_IDX: u32 = 4;

/// Create a function for allocationg strings in memory via easyjs native.
///
/// This function allocates the string in memory and returns a pointer to it.
pub fn allocate_string() -> EasyNativeFN {
    let locals = vec![];
    let mut function = Function::new(locals);

    // Get current GLOBAL_STRING_OFFSET (initial pointer)
    function.instruction(&Instruction::GlobalGet(GLOBAL_STRING_IDX));

    // Update GLOBAL_STRING_OFFSET after allocation
    function.instruction(&Instruction::GlobalGet(GLOBAL_STRING_IDX));
    function.instruction(&Instruction::LocalGet(0));
    function.instruction(&Instruction::I32Add);
    function.instruction(&Instruction::GlobalSet(GLOBAL_STRING_IDX));

    // Return original pointer (before allocation)
    function.instruction(&Instruction::End);

    EasyNativeFN {
        signature: FunctionSignature {
            params: vec![ValType::I32],
            results: vec![ValType::I32],
        },
        function,
        name: "allocate_string".to_string(),
        idx: ALLOCATE_STRING_IDX,
    }
}

// /// Create a function for storing strings in memory via easyjs native.
// pub fn store_string_byte() -> EasyNativeFN {
//     let locals = vec![];
//     let mut function = Function::new(locals);
// }

// pub fn store_string() -> EasyNativeFN {
//     let locals = vec![(2, ValType::I32)]; // Local variables: ptr, i for loop

//     // The length of the string
//     let str_length_local: u32 = 0;
//     // the bytes of the string
//     let str_bytes: u32 = 1;
//     // The ptr that will be set from allocate_string
//     let ptr_local: u32 = 2;
//     // For the loop
//     let i_local: u32 = 3;

//     let mut function = Function::new(locals);

//     // 1. Allocate memory... (string length + data)
//     // string length
//     function.instruction(&Instruction::LocalGet(str_length_local));
//     function.instruction(&Instruction::I32Const(4)); // ADD 4 bytes for a easy way to determine string length later on.
//     function.instruction(&Instruction::I32Add);
//     function.instruction(&Instruction::Call(ALLOCATE_STRING_IDX));
//     // store pointer
//     function.instruction(&Instruction::LocalSet(ptr_local));

//     // 2. Store length at the start
//     function.instruction(&Instruction::LocalGet(str_length_local));
//     function.instruction(&Instruction::LocalGet(ptr_local));
//     function.instruction(&Instruction::I32Store(MemArg {
//         align: 2,
//         offset: 0,
//         memory_index: 0,
//     }));

//     // 3. loop through bytes and store them AFTER the length
//     function.instruction(&Instruction::I32Const(0)); // i = 0
//     function.instruction(&Instruction::LocalSet(i_local));

//     let loop_label = 0;

//     function.instruction(&Instruction::Block(BlockType::Empty)); // start block
//     function.instruction(&Instruction::Block(BlockType::Empty)); // start loop

//     // Break condition if i >= length
//     function.instruction(&Instruction::LocalGet(i_local)); // load i
//     function.instruction(&Instruction::LocalGet(str_length_local)); // get length of string
//     function.instruction(&Instruction::I32GeU);
//     function.instruction(&Instruction::BrIf(1)); // break if i >= length

//     // store characters at index ptr + 4 + i
//     function.instruction(&Instruction::LocalGet(ptr_local));
//     function.instruction(&Instruction::I32Const(4));
//     function.instruction(&Instruction::LocalGet(i_local));
//     function.instruction(&Instruction::I32Add);

//     function.instruction(&Instruction::LocalGet(str_bytes)); // bytes ptr
//     function.instruction(&Instruction::LocalGet(i_local)); // i
//     function.instruction(&Instruction::I32Add); // bytes ptr + i
//     function.instruction(&Instruction::I32Load8U(MemArg {
//         align: 0,
//         offset: 0,
//         memory_index: 0,
//     })); // load byte from input string

//     // store the bytes
//     function.instruction(&Instruction::I32Store8(MemArg {
//         align: 0,
//         offset: 0,
//         memory_index: 0,
//     }));

//     // increment i
//     function.instruction(&Instruction::LocalGet(i_local));
//     function.instruction(&Instruction::I32Const(1));
//     function.instruction(&Instruction::I32Add);
//     function.instruction(&Instruction::LocalSet(i_local));

//     // repeat loop
//     function.instruction(&Instruction::Br(loop_label));

//     // end loop
//     function.instruction(&Instruction::End);
//     function.instruction(&Instruction::End); // block

//     // 4. return pointer
//     function.instruction(&Instruction::LocalGet(ptr_local));
//     function.instruction(&Instruction::End);

//     EasyNativeFN {
//         signature: FunctionSignature {
//             params: vec![ValType::I32, ValType::I32], // string length, string bytes
//             results: vec![ValType::I32],              // ptr to string...
//         },
//         function,
//         name: "store_string".to_string(),
//         idx: STORE_STRING_IDX,
//     }
// }

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
        },
        function,
        name: "get_length_string".to_string(),
        idx: GET_STRING_LENGTH_IDX,
    }
}

pub fn get_string() {}
pub fn concat_strings() {}
pub fn free_string() {}
