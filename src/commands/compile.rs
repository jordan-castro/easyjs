use crate::lexer::lex;
use crate::parser::par;
use crate::compiler::transpile::Transpiler;
use crate::utils::version;

pub fn compile(input: String, pretty: bool) -> String {
    let lexer = lex::Lex::new(input);
    let mut parser = par::Parser::new(lexer);
    let program = parser.parse_program();

    if parser.errors.len() > 0 {
        for e in parser.errors {
            println!("{}", e);
        }
        return String::new();
    }

    let mut transpiler = Transpiler::new();

    let js = transpiler.transpile(program, pretty);
    format!("// Compiled by EasyJS version {}\n{}", version::VERSION_CODE, js)
}