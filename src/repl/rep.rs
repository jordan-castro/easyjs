use std::io::Write;

use crate::utils::version;
use crate::lexer::lex;
use crate::parser::par;
use crate::compiler::transpile::Transpiler;
use crate::repl::runtime::Runtime;

/// Repl for EasyJS
const PROMPT: &str = ">> ";

const EASY_JS_ASCII:&str = "    ___       ___       ___       ___            ___       ___   
   /\\  \\     /\\  \\     /\\  \\     /\\__\\          /\\  \\     /\\  \\  
  /::\\  \\   /::\\  \\   /::\\  \\   |::L__L        _\\:\\  \\   /::\\  \\ 
 /::\\:\\__\\ /::\\:\\__\\ /\\:\\:\\__\\  |:::\\__\\      /\\/::\\__\\ /\\:\\:\\__\\
 \\:\\:\\/  / \\/\\::/  / \\:\\:\\/__/  /:;;/__/      \\::/\\/__/ \\:\\:\\/__/
  \\:\\/  /    /:/  /   \\::/  /   \\/__/          \\/__/     \\::/  / 
   \\/__/     \\/__/     \\/__/                              \\/__/  ";


pub fn start(runtime_option: &str, crash_on_error: bool, debug:bool) {
    let mut runtime = Runtime::new(runtime_option, crash_on_error);
    let mut transpiler = Transpiler::new();
    println!("{}", EASY_JS_ASCII);
    println!("EasyJS {}", version::VERSION_CODE);

    let builtins = transpiler.builtins.to_owned();
    // run the builtin scripts
    for builtin in builtins {
        runtime.send_command(&builtin);
    }

    loop {
        transpiler.reset();
        print!("{}", PROMPT);
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "quit" {
            break;
        }

        let lexer = lex::Lex::new(input.to_string());
        let mut parser = par::Parser::new(lexer);
        let program = parser.parse_program();

        if parser.errors.len() > 0 {
            for e in parser.errors {
                println!("{}", e);
            }
            continue;
        }

        let js = transpiler.transpile(program, false);

        if debug {
            println!("{}", js);
        }

        let output: Vec<String> = runtime.send_command(&js);

        for line in output {
            if line.starts_with(">") {
                println!("{}", line.strip_prefix("> ").unwrap());
            } else {
                println!("{}", line);
            }
        }
    }

    runtime.close();
}