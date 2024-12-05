use std::error::Error;

use crate::lexer::lex;
use crate::parser::par;
use crate::compiler::transpile::Transpiler;
use crate::utils::version;

/// Compile a string of EasyJS into JS code.
/// 
/// `pretty:bool` do we make it look pretty?
/// `place_watermark:bool` Does the watermark 'compiled by easjs...' go on?
/// 
/// return `String`
fn compile(input: String, pretty: bool, place_watermark: bool) -> Result<(String, Transpiler), Box<dyn Error>> {
    let lexer = lex::Lex::new(input);
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
pub fn compile_main(input: String) -> String {
    compile(input, false, true).expect("Could not compile").0
}

/// Compile a imported easyjs module.
pub fn compile_module(input: String) -> (String, Transpiler) {
    compile(input,false, false).expect("Could not compile")
}