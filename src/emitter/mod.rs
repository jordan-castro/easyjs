// This module is used to help emit WASM module from Native code.
// It is used by native.rs

pub mod wasm_emitter;
pub mod signatures;
mod fn_builder;
pub mod utils;
mod variables;
mod strings;
pub mod instruction_generator;