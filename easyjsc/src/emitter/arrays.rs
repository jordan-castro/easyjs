use wasm_encoder::{BlockType, Function, Instruction, MemArg, ValType};

use crate::{
    emitter::{
        builtins::{
            ARR_ALLOCATE_IDX, ARR_ALLOCATE_NAME, ARR_GET_CAP_IDX, ARR_GET_CAP_NAME, ARR_GET_ITEM_IDX, ARR_GET_ITEM_NAME, ARR_GET_LEN_IDX, ARR_GET_LEN_NAME, ARR_PUSH_ARRAY_IDX, ARR_PUSH_ARRAY_NAME, ARR_PUSH_FLOAT_IDX, ARR_PUSH_FLOAT_NAME, ARR_PUSH_INT_IDX, ARR_PUSH_INT_NAME, ARR_PUSH_STRING_IDX, ARR_PUSH_STRING_NAME, ARR_REALLOCATE_IDX, ARR_REALLOCATE_NAME, ARR_STORE_CAPACITY_IDX, ARR_STORE_CAPACITY_NAME, ARR_STORE_LENGTH_IDX, ARR_STORE_LENGTH_NAME, GLOBAL_HEAP_IDX
        },
        instruction_generator::{call, i32_load, i32_store, set_local_to_global, EasyInstructions},
        signatures::{EasyNativeFN, FunctionSignature},
    },
    new_function_with_instructions,
    typechecker::{StrongValType, ARRAY_TYPE_IDX, F32_TYPE_IDX, I32_TYPE_IDX, STRING_TYPE_IDX},
};

const DYNAMIC_ARRAY_ITEM_BYTE_SIZE: i32 = 8;

/// A Native builtin function to allocate space for an array.
/// Creates the space for the array and sets in memory.
/// Returns a pointer to the original position.
pub fn native_allocate_array() -> EasyNativeFN {
    // Paramaters are: capacity
    let locals = vec![(1, ValType::I32)]; // the ptr
    let mut instructions = vec![];

    // Logic:
    // 1. ptr = GLOBAL_HEAP_IDX
    // 2. GLOBAL_HEAP_IDX += capacity * item_size + 8
    // 8 bytes for length, capacity
    // item_size = 8 bytes, 4 for type, 4 for value (i32, f32, string ptr, object ptr, array ptr)

    // 3. return pointer

    instructions.append(&mut set_local_to_global(1, GLOBAL_HEAP_IDX));
    instructions.append(&mut vec![
        Instruction::LocalGet(0), // capacity
        Instruction::I32Const(DYNAMIC_ARRAY_ITEM_BYTE_SIZE),
        Instruction::I32Mul,
        Instruction::I32Const(8), // array needed size for length,capacity
        Instruction::I32Add,      // add it to multiplied result
        Instruction::GlobalGet(GLOBAL_HEAP_IDX), // get global
        Instruction::I32Add,
        Instruction::GlobalSet(GLOBAL_HEAP_IDX), // set global
        Instruction::LocalGet(1),                // return ptr
        Instruction::End,
    ]);

    EasyNativeFN {
        signature: FunctionSignature {
            params: vec![ValType::I32],  // the initial capacity of the array
            results: vec![ValType::I32], // the ptr
            param_strong: vec![StrongValType::Int],
            results_strong: vec![StrongValType::Int],
        },
        function: new_function_with_instructions!(locals, instructions),
        name: ARR_ALLOCATE_NAME.to_string(),
        idx: ARR_ALLOCATE_IDX,
        is_public: true,
    }
}

/// Native function to store the length of a array.
pub fn native_arr_store_length() -> EasyNativeFN {
    // paramaters are: ptr, length
    let locals = vec![]; // no locals necessary
    let mut instructions = vec![];

    let ptr_idx = 0;
    let length_idx = 1;

    // Logic
    // Length is first 4 bytes.
    // 1. ptr, length i32.store to store length at ptr i.e. first 4 bytes.
    // 2. nothing else

    instructions.append(&mut vec![
        Instruction::LocalGet(ptr_idx),
        Instruction::LocalGet(length_idx),
    ]);
    instructions.append(&mut i32_store(0, 0, 0));
    instructions.push(Instruction::End);

    EasyNativeFN {
        signature: FunctionSignature {
            params: vec![ValType::I32, ValType::I32], // ptr, length
            results: vec![],
            param_strong: vec![StrongValType::Int, StrongValType::Int],
            results_strong: vec![], // no results
        },
        function: new_function_with_instructions!(locals, instructions),
        name: ARR_STORE_LENGTH_NAME.to_string(),
        idx: ARR_STORE_LENGTH_IDX,
        is_public: true,
    }
}

/// Native function to store the capacity of the array.
pub fn native_arr_store_capacity() -> EasyNativeFN {
    // Paramaters are ptr, capacity
    let locals = vec![]; // No locals necessary
    let mut instructions = vec![];

    // Logic
    // Capcity is after lengh i.e. ptr + 4
    // 1. ptr + 4, capacity i32.store
    // 2. end

    instructions.append(&mut vec![
        Instruction::LocalGet(0),
        Instruction::I32Const(4),
        Instruction::I32Add,
        Instruction::LocalGet(1),
    ]);
    instructions.append(&mut i32_store(0, 0, 0));
    instructions.push(Instruction::End);

    EasyNativeFN {
        signature: FunctionSignature {
            params: vec![ValType::I32, ValType::I32],
            results: vec![],
            param_strong: vec![StrongValType::Int, StrongValType::Int],
            results_strong: vec![],
        },
        function: new_function_with_instructions!(locals, instructions),
        name: ARR_STORE_CAPACITY_NAME.to_string(),
        idx: ARR_STORE_CAPACITY_IDX,
        is_public: true,
    }
}

/// Native function to get the length of the array.
pub fn native_arr_get_len() -> EasyNativeFN {
    // paramaters are ptr
    let locals = vec![]; // None necessary
    let mut instructions = vec![];

    // Logic
    // 1. ptr, i32_load
    // 2. return

    instructions.push(Instruction::LocalGet(0));
    instructions.append(&mut i32_load(0, 0, 0));
    instructions.push(Instruction::End);

    EasyNativeFN {
        signature: FunctionSignature {
            params: vec![ValType::I32],  // ptr
            results: vec![ValType::I32], // length
            param_strong: vec![StrongValType::Int],
            results_strong: vec![StrongValType::Int],
        },
        function: new_function_with_instructions!(locals, instructions),
        name: ARR_GET_LEN_NAME.to_string(),
        idx: ARR_GET_LEN_IDX,
        is_public: true,
    }
}

/// Native function to get the capacity of the array.
pub fn native_arr_get_cap() -> EasyNativeFN {
    // Paramaters are ptr
    let locals = vec![]; // none necassary
    let mut instructions = vec![];

    // Logic
    // 1. ptr + 4, i32_load
    // 2. return

    instructions.append(&mut vec![
        Instruction::LocalGet(0), // ptr
        Instruction::I32Const(4),
        Instruction::I32Add,
    ]);
    instructions.append(&mut i32_load(0, 0, 0));
    instructions.push(Instruction::End);

    EasyNativeFN {
        signature: FunctionSignature {
            params: vec![ValType::I32],  // ptr
            results: vec![ValType::I32], // length
            param_strong: vec![StrongValType::Int],
            results_strong: vec![StrongValType::Int],
        },
        function: new_function_with_instructions!(locals, instructions),
        name: ARR_GET_CAP_NAME.to_string(),
        idx: ARR_GET_CAP_IDX,
        is_public: true,
    }
}

/// Native function to reallocate an array.
pub fn native_arr_reallocate() -> EasyNativeFN {
    // Paramaters are: ptr
    let locals = vec![(7, ValType::I32)];
    let mut instructions = vec![];

    /**

       // like this
       fn __arr_reallocate(ptr:int) {
           // Load length
           length = __i32_load(ptr)
           // Load capacity
           capacity = __i32_load(ptr + 4)
           // Double capcity
           capacity *= 2

           // allocate new array
           new_ptr = __arr_allocate(capacity)
           // set length
           __arr_set_length(new_ptr, length)
           // set capacity
           __arr_set_capacity(new_ptr, capacity)

           // Loop through old array
           loop_index = -1
           for true {
               loop_index += 1
               // Get type of current item
               type = __i32_load(ptr + 8 + loop_index)
               // Check type
               if type == I32 {
                  // store type
                  __i32_store(new_ptr + 8 + loop_index, type)
                  // store value
                  __i32_store(new_ptr + 12 + loop_index, __i32_load(ptr + 12 + loop_index))
               }
               if type == F32 {
                   // .....
               }
           }
       }

    */
    let old_ptr = 0;
    let new_ptr = 1;
    let length = 2;
    let capacity = 3;
    let loop_index = 4;
    let item_type = 5;
    let old_item_position = 6;
    let new_item_position = 7;

    // Get and set length
    instructions.append(&mut vec![
        Instruction::LocalGet(old_ptr),
        Instruction::Call(ARR_GET_LEN_IDX),
        Instruction::LocalSet(length),
    ]);

    // Get and set capacity
    instructions.append(&mut vec![
        Instruction::LocalGet(old_ptr),
        Instruction::Call(ARR_GET_CAP_IDX),
        Instruction::LocalSet(capacity),
    ]);

    // Allocate new array, set length, set capacity
    instructions.append(&mut vec![
        Instruction::LocalGet(capacity),
        Instruction::Call(ARR_ALLOCATE_IDX),
        Instruction::LocalSet(new_ptr),
        // set length
        Instruction::LocalGet(new_ptr),
        Instruction::LocalGet(length),
        Instruction::Call(ARR_STORE_LENGTH_IDX),
        // store capacity
        Instruction::LocalGet(new_ptr),
        Instruction::LocalGet(capacity),
        Instruction::Call(ARR_STORE_CAPACITY_IDX),
    ]);

    // Loop through old array and set values in new array position.
    instructions.append(&mut vec![
        // Start loop
        Instruction::I32Const(-1),
        Instruction::LocalSet(loop_index),
        Instruction::Loop(BlockType::Empty),
        // Update index
        Instruction::LocalGet(loop_index),
        Instruction::I32Const(1),
        Instruction::I32Add,
        Instruction::LocalSet(loop_index),
        // Get type
        // Math is: ptr + 8[length:capacity] + loop_index
        Instruction::LocalGet(old_ptr),
        Instruction::I32Const(8),
        Instruction::I32Add,
        Instruction::LocalGet(loop_index),
        Instruction::I32Const(DYNAMIC_ARRAY_ITEM_BYTE_SIZE),
        Instruction::I32Mul,
        Instruction::I32Add,
        Instruction::I32Load(MemArg {
            offset: 0,
            align: 0,
            memory_index: 0,
        }),
        Instruction::LocalSet(item_type),
        // Save type at new position
        // Math is: new_ptr + 8[length:capacity] + loop_index
        Instruction::LocalGet(new_ptr),
        Instruction::I32Const(8),
        Instruction::I32Add,
        Instruction::LocalGet(loop_index),
        Instruction::I32Const(DYNAMIC_ARRAY_ITEM_BYTE_SIZE),
        Instruction::I32Mul,
        Instruction::I32Add,
        Instruction::LocalGet(item_type),
        Instruction::I32Store(MemArg {
            offset: 0,
            align: 0,
            memory_index: 0,
        }),
        // Set old item position
        // Math is: ptr + 8[length:capacity] + loop_index + 4
        Instruction::LocalGet(old_ptr),
        Instruction::I32Const(8),
        Instruction::I32Add,
        Instruction::LocalGet(loop_index),
        Instruction::I32Const(DYNAMIC_ARRAY_ITEM_BYTE_SIZE),
        Instruction::I32Mul,
        Instruction::I32Add,
        Instruction::I32Const(4),
        Instruction::I32Add,
        Instruction::LocalSet(old_item_position),
        // Set new item position
        // Math is: new_ptr + 8[l:c] + loop_index + 4
        Instruction::LocalGet(new_item_position),
        Instruction::I32Const(8),
        Instruction::I32Add,
        Instruction::LocalGet(loop_index),
        Instruction::I32Const(DYNAMIC_ARRAY_ITEM_BYTE_SIZE),
        Instruction::I32Mul,
        Instruction::I32Add,
        Instruction::I32Const(4),
        Instruction::I32Add,
        Instruction::LocalSet(new_item_position),
        // Check type is INT, or STRING, or ARRAY
        Instruction::LocalGet(item_type),
        Instruction::I32Const(I32_TYPE_IDX),
        Instruction::I32Eq,
        Instruction::LocalGet(item_type),
        Instruction::I32Const(STRING_TYPE_IDX),
        Instruction::I32Eq,
        Instruction::I32Or,
        Instruction::LocalGet(item_type),
        Instruction::I32Const(ARRAY_TYPE_IDX),
        Instruction::I32Eq,
        Instruction::I32Or,
        Instruction::If(BlockType::Empty),
        // Is int, Get position
        Instruction::LocalGet(new_item_position),
        // Get old value
        Instruction::LocalGet(old_item_position),
        Instruction::I32Load(MemArg {
            offset: 0,
            align: 0,
            memory_index: 0,
        }),
        // Set new
        Instruction::I32Store(MemArg {
            offset: 0,
            align: 0,
            memory_index: 0,
        }),
        // End if statement
        Instruction::End,
        // New if statement for F32
        // First get type
        Instruction::LocalGet(item_type),
        Instruction::I32Const(F32_TYPE_IDX),
        Instruction::I32Eq,
        Instruction::If(BlockType::Empty),
        // If statement for Floats
        // Get position
        Instruction::LocalGet(new_item_position),
        // Get old value
        Instruction::LocalGet(old_item_position),
        Instruction::F32Load(MemArg {
            offset: 0,
            align: 0,
            memory_index: 0,
        }),
        // set new
        Instruction::F32Store(MemArg {
            offset: 0,
            align: 0,
            memory_index: 0,
        }),
        // End if
        Instruction::End,
        // Check loop_index
        Instruction::LocalGet(loop_index),
        Instruction::LocalGet(length),
        Instruction::I32LtS,
        Instruction::BrIf(0),
        // end loop
        Instruction::End,
    ]);
    // Return new pointer
    instructions.append(&mut vec![Instruction::LocalGet(new_ptr), Instruction::End]);

    EasyNativeFN {
        signature: FunctionSignature {
            params: vec![ValType::I32],  // ptr
            results: vec![ValType::I32], // new_ptr
            param_strong: vec![StrongValType::Int],
            results_strong: vec![StrongValType::Int],
        },
        function: new_function_with_instructions!(locals, instructions),
        name: ARR_REALLOCATE_NAME.to_string(),
        idx: ARR_REALLOCATE_IDX,
        is_public: true,
    }
}

/// Instructions for pushing to a array. These instructions vary very slightly depending on what you are pushing.
/// This function simply returns the instructions based on a type.
fn instructions_for_pushing_to_array(item_type_idx: i32, function_idx: u32) -> EasyInstructions {
    let old_ptr = 0;
    let item = 1;
    // locals
    let new_ptr = 2;
    let length = 3;
    let item_position = 4;
    let byte_length = 5;

    let mut instructions = vec![];

    instructions.append(&mut vec![
        // Automatically set the new ptr to the old ptr
        Instruction::LocalGet(old_ptr),
        Instruction::LocalSet(new_ptr),
        // Get length
        Instruction::LocalGet(old_ptr),
        Instruction::Call(ARR_GET_LEN_IDX),
        // Set length
        Instruction::LocalSet(length),
        // Multiply length by item size
        Instruction::LocalGet(length),
        Instruction::I32Const(DYNAMIC_ARRAY_ITEM_BYTE_SIZE),
        Instruction::I32Mul,
        Instruction::LocalSet(byte_length),
        Instruction::LocalGet(length),
        // Get capacity
        Instruction::LocalGet(old_ptr),
        Instruction::Call(ARR_GET_CAP_IDX),
        // Check left less than right
        Instruction::I32LtU,
        Instruction::If(BlockType::Empty),
        // We can push
        // Get position, Math is: ptr + 8[l:c] + byte_length
        Instruction::LocalGet(old_ptr),
        Instruction::I32Const(8),
        Instruction::I32Add,
        Instruction::LocalGet(byte_length),
        Instruction::I32Add,
        // set item position
        Instruction::LocalSet(item_position),
        // Save type
        Instruction::LocalGet(item_position),
        Instruction::I32Const(item_type_idx),
        Instruction::I32Store(MemArg {
            offset: 0,
            align: 0,
            memory_index: 0,
        }),
        // Save value
        Instruction::LocalGet(item_position),
        Instruction::I32Const(4),
        Instruction::I32Add,
        Instruction::LocalGet(item),
    ]);

    // Int, String, and Array are all I32
    if item_type_idx == I32_TYPE_IDX
        || item_type_idx == STRING_TYPE_IDX
        || item_type_idx == ARRAY_TYPE_IDX
    {
        instructions.push(Instruction::I32Store(MemArg {
            offset: 0,
            align: 0,
            memory_index: 0,
        }));
    } else if item_type_idx == F32_TYPE_IDX {
        instructions.push(Instruction::F32Store(MemArg {
            offset: 0,
            align: 0,
            memory_index: 0,
        }));
    }

    instructions.append(&mut vec![
        // Update new length
        Instruction::LocalGet(old_ptr),
        Instruction::LocalGet(length),
        Instruction::I32Const(1),
        Instruction::I32Add,
        Instruction::Call(ARR_STORE_LENGTH_IDX),
        // Else!
        Instruction::Else,
        // This means we have to reallocate the array
        Instruction::LocalGet(old_ptr),
        Instruction::Call(ARR_REALLOCATE_IDX),
        Instruction::LocalSet(new_ptr),
        // Recall this function
        Instruction::LocalGet(new_ptr),
        Instruction::LocalGet(item),
        Instruction::Call(function_idx),
        Instruction::LocalSet(new_ptr),
        Instruction::End,
        // Return new_ptr
        Instruction::LocalGet(new_ptr),
        Instruction::End,
    ]);

    instructions
}

/// Native function to push a INT to a array.
/// Will allocate a new array if beyond our capacity
pub fn native_arr_push_int() -> EasyNativeFN {
    // paramaters: ptr, item(int)
    let locals = vec![(4, ValType::I32)];
    let instructions = instructions_for_pushing_to_array(I32_TYPE_IDX, ARR_PUSH_INT_IDX);

    // Logic
    // 1. Get length.
    // 2. Get capacity
    // 3. if length < capacity push item and set new length
    // 4. otherwise reallocate a new array
    // 5. push item and set new length
    // 6. return ptr

    EasyNativeFN {
        signature: FunctionSignature {
            params: vec![ValType::I32, ValType::I32], // ptr, item
            results: vec![ValType::I32],              // new_ptr
            param_strong: vec![StrongValType::Int, StrongValType::Int],
            results_strong: vec![StrongValType::Int],
        },
        function: new_function_with_instructions!(locals, instructions),
        name: ARR_PUSH_INT_NAME.to_string(),
        idx: ARR_PUSH_INT_IDX,
        is_public: true,
    }
}

/// Native function to push a float to a array.
/// Will allocate new array if reached max capacity.
pub fn native_arr_push_float() -> EasyNativeFN {
    // paramaters are ptr, item(float)
    let locals = vec![(4, ValType::I32)];
    let instructions = instructions_for_pushing_to_array(F32_TYPE_IDX, ARR_PUSH_FLOAT_IDX);

    EasyNativeFN {
        signature: FunctionSignature {
            params: vec![ValType::I32, ValType::F32],
            results: vec![ValType::I32],
            param_strong: vec![StrongValType::Int, StrongValType::Float],
            results_strong: vec![StrongValType::Int],
        },
        function: new_function_with_instructions!(locals, instructions),
        name: ARR_PUSH_FLOAT_NAME.to_string(),
        idx: ARR_PUSH_FLOAT_IDX,
        is_public: true,
    }
}

/// Native function to push a string to a array.
/// Will allocate new array if needed.
pub fn native_arr_push_string() -> EasyNativeFN {
    let locals = vec![(4, ValType::I32)];
    let instructions = instructions_for_pushing_to_array(STRING_TYPE_IDX, ARR_PUSH_STRING_IDX);

    EasyNativeFN {
        signature: FunctionSignature {
            params: vec![ValType::I32, ValType::I32],
            results: vec![ValType::I32],
            param_strong: vec![StrongValType::Int, StrongValType::String],
            results_strong: vec![StrongValType::Int],
        },
        function: new_function_with_instructions!(locals, instructions),
        name: ARR_PUSH_STRING_NAME.to_string(),
        idx: ARR_PUSH_STRING_IDX,
        is_public: true,
    }
}

/// Native function to push a array to a array.
/// Will allocate new array if needed
pub fn native_arr_push_array() -> EasyNativeFN {
    let locals = vec![(4, ValType::I32)];
    let instructions = instructions_for_pushing_to_array(ARRAY_TYPE_IDX, ARR_PUSH_ARRAY_IDX);
    
    EasyNativeFN {
        signature: FunctionSignature {
            params: vec![ValType::I32, ValType::I32],
            results: vec![ValType::I32],
            param_strong: vec![StrongValType::Int, StrongValType::Array],
            results_strong: vec![StrongValType::Int],
        },
        function: new_function_with_instructions!(locals, instructions),
        name: ARR_PUSH_ARRAY_NAME.to_string(),
        idx: ARR_PUSH_ARRAY_IDX,
        is_public: true,
    }
}

/// Native function to read from an array. (__arr_get_item)
pub fn native_arr_get_item() -> EasyNativeFN {
    // params: ptr, index
    let locals = vec![(6, ValType::I32), (1, ValType::F32)];
    let mut instructions = vec![];

    // Logic
    // 1. Get byte_position of item
    // 2. Read and save type
    // 3. Based on type either do a I32Load or F32Load.
    // 4. return result
    let ptr = 0;
    let index = 1;
    
    let byte_position = 2;
    let int_value = 3;
    let value_position = 4;
    let string_value = 5;
    let array_value = 6;
    let item_type = 7;
    let float_value = 8;

    instructions.append(&mut vec![
        // Set defaults
        Instruction::I32Const(0),
        Instruction::LocalSet(int_value),
        Instruction::F32Const(0.0),
        Instruction::LocalSet(float_value),
        Instruction::I32Const(0),
        Instruction::LocalSet(string_value),
        Instruction::I32Const(0),
        Instruction::LocalSet(array_value),

        // Get byte position
        Instruction::LocalGet(index),
        Instruction::I32Const(DYNAMIC_ARRAY_ITEM_BYTE_SIZE),
        Instruction::I32Mul,
        Instruction::LocalGet(ptr),
        Instruction::I32Add,
        Instruction::I32Const(8),
        Instruction::I32Add,
        Instruction::LocalSet(byte_position),
        
        // Set value_position
        Instruction::LocalGet(byte_position),
        Instruction::I32Const(4),
        Instruction::I32Add,
        Instruction::LocalSet(value_position),

        // Read and save type
        Instruction::LocalGet(byte_position),
        Instruction::I32Load(MemArg { offset: 0, align: 0, memory_index: 0 }),
        Instruction::LocalSet(item_type),

        // Check if type is INT
        Instruction::LocalGet(item_type),
        Instruction::I32Const(I32_TYPE_IDX),
        Instruction::I32Eq,
        Instruction::If(BlockType::Empty),
        // Get int value
        Instruction::LocalGet(value_position),
        Instruction::I32Load(MemArg { offset: 0, align: 0, memory_index: 0 }),
        Instruction::LocalSet(int_value),
        Instruction::End,
        
        // Check if type is FLOAT
        Instruction::LocalGet(item_type),
        Instruction::I32Const(F32_TYPE_IDX),
        Instruction::I32Eq,
        Instruction::If(BlockType::Empty),
        // Get float value
        Instruction::LocalGet(value_position),
        Instruction::F32Load(MemArg { offset: 0, align: 0, memory_index: 0 }),
        Instruction::LocalSet(float_value),
        Instruction::End,

        // Check if type is STRING
        Instruction::LocalGet(item_type),
        Instruction::I32Const(STRING_TYPE_IDX),
        Instruction::I32Eq,
        Instruction::If(BlockType::Empty),
        // Get string ptr
        Instruction::LocalGet(value_position),
        Instruction::I32Load(MemArg { offset: 0, align: 0, memory_index: 0 }),
        Instruction::LocalSet(string_value),
        Instruction::End,

        // Check if type is ARRAY
        Instruction::LocalGet(item_type),
        Instruction::I32Const(ARRAY_TYPE_IDX),
        Instruction::I32Eq,
        Instruction::If(BlockType::Empty),
        // Get Array ptr
        Instruction::LocalGet(value_position),
        Instruction::I32Load(MemArg { offset: 0, align: 0, memory_index: 0 }),
        Instruction::LocalSet(array_value),
        Instruction::End,
        
        // Return the values!
        Instruction::LocalGet(item_type),
        Instruction::LocalGet(int_value),
        Instruction::LocalGet(float_value),
        Instruction::LocalGet(string_value),
        Instruction::LocalGet(array_value),

        Instruction::End
    ]);

    EasyNativeFN {
        signature: FunctionSignature {
            params: vec![ValType::I32, ValType::I32],
            results: vec![ValType::I32, ValType::I32, ValType::F32, ValType::I32, ValType::I32],
            param_strong: vec![StrongValType::Int, StrongValType::Int],
            results_strong: vec![StrongValType::Int, StrongValType::Int, StrongValType::Float, StrongValType::String, StrongValType::Array],
        },
        function: new_function_with_instructions!(locals, instructions),
        name: ARR_GET_ITEM_NAME.to_string(),
        idx: ARR_GET_ITEM_IDX,
        is_public: true,
    }
}