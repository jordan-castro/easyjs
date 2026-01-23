use wasm_encoder::{Instruction, MemArg, ValType};

// use crate::{
//     emitter::builtins::{ALLOCATE_STRING_IDX, STORE_STRING_LENGTH_IDX, STR_STORE_BYTE_IDX},
//     errors::make_native_error,
//     parser::ast::Expression,
// };

// /// Macro for creating a new wasm function with instructions.
// #[macro_export]
// macro_rules! new_function_with_instructions {
//     ($locals:expr, $instructions: expr) => {{
//         let mut function = Function::new($locals);
//         for instruction in $instructions {
//             function.instruction(&instruction);
//         }
//         function
//     }};
// }

// /// Instructions for setting a string byte within a loop
// ///
// /// `loop_index: u32` This is the idx of the loop index.
// ///
// /// `position: u32` This is the idx of the position variable. This will be set and get.
// ///
// /// `from_string_ptr: u32` This is the idx of the ptr of the string from which we are loading the byte.
// ///
// /// `byte: u32` The idx of the byte variable. This will be get and set.
// ///
// /// `to_string_ptr: u32` The idx of the ptr of the strign to which we are setting the byte.
// #[macro_export]
// macro_rules! set_string_byte_in_loop {
//     ($loop_index: expr, $position: expr, $from_string_ptr: expr, $byte: expr, $to_string_ptr: expr) => {
//         vec![
//             Instruction::LocalGet($loop_index),
//             Instruction::I32Const(4),
//             Instruction::I32Add,
//             Instruction::LocalSet($position),
//             // Set up for byte
//             Instruction::LocalGet($position),
//             Instruction::LocalGet($from_string_ptr),
//             Instruction::I32Add,
//             // Get byte
//             Instruction::I32Load(MemArg {
//                 offset: 0,
//                 align: 0,
//                 memory_index: 0,
//             }),
//             // set local byte
//             Instruction::LocalSet($byte),
//             // setup for __str_store_byte
//             Instruction::LocalGet($to_string_ptr),
//             Instruction::LocalGet($position),
//             Instruction::LocalGet($byte),
//             // call __str_store_byte
//             Instruction::Call(STR_STORE_BYTE_IDX),
//         ]
//     };
// }

// /// Get arguments from when a wasm_core function is called.
// ///
// /// `arguments:&Vec<Expression>` a vector of expressions as arguments
// ///
// /// `core_fn_name:&str` the name of the core function.
// ///
// /// `error:&str` A possible error message.
// ///
// /// `errors:Vec<String>` mutable vector to ad the error to.
// ///
// /// returns: `args: Vec<u32>` The generated arguments.
// macro_rules! wasm_core_args {
//     ($arguments:expr, $core_fn_name:expr, $error:expr, $errors:expr) => {{
//         let mut args = vec![];
//         for arg in $arguments {
//             match arg {
//                 Expression::IntegerLiteral(_, value) => {
//                     args.push(*value as u32);
//                 }
//                 _ => $errors.push(make_native_error(arg.get_token(), $error)),
//             }
//         }

//         args
//     }};
// }

pub type EasyInstructions = Vec<Instruction<'static>>;