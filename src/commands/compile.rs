use std::error::Error;

use crate::lexer::lex;
use crate::parser::par;
use crate::compiler::transpile::Transpiler;
use crate::utils::version;

/// Compile a string of EasyJS into JS code.
/// 
/// `place_watermark:bool` Does the watermark 'compiled by easjs...' go on?
/// `file_name: &str` The name of the file.
/// 
/// return `String`
fn compile(input: String, place_watermark: bool, file_name: &str) -> Result<(String, Transpiler), Box<dyn Error>> {
    let lexer = lex::Lex::new_with_file(input, file_name.to_owned());
    let mut parser = par::Parser::new(lexer);
    let program = parser.parse_program();

    if parser.errors.len() > 0 {
        for e in parser.errors {
            println!("{}", e);
        }
        return Err("Failed to parse input".into());
    }

    let mut transpiler = Transpiler::new();

    let js = transpiler.transpile(program);
    let watermark = if place_watermark {
        format!("// Compiled by EasyJS version {}\n", version::VERSION_CODE)
    } else {
        "".to_string()
    };
    Ok((format!("{}{}", watermark, js), transpiler))
}

/// Compile the main source of the program or the original file.
pub fn compile_main(input: String, file_name: &str) -> String {
    compile(input, true, file_name).expect("Could not compile").0
}

/// Compile a imported easyjs module.
pub fn compile_module(input: String, module_name:&str) -> (String, Transpiler) {
    compile(input, false, module_name).expect("Could not compile")
}

/// Compile for repl
pub fn compile_for_repl(input: String) -> String {
    compile(input, false, "").expect("Could not compile0").0
}