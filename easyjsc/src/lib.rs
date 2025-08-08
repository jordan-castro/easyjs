pub mod builtins;
pub mod compiler;
pub mod lexer;
pub mod std;
pub mod parser;
pub mod emitter;
pub mod errors;
pub mod typechecker;

use compiler::transpile::Transpiler;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compile_easy_js(input: String) -> String {
    let mut transpiler = Transpiler::new();
    transpiler.transpile_from_string(input)
}