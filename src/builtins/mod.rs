use crate::{compiler::transpile::Transpiler, lexer::lex, parser::par, std, utils::reader::read_file};

pub const INT_RANGE: &str = "int_range";

/// EasyJS builtin include
/// 
/// Includes a easyjs file that becomed modulirazed as 
/// const file_name = {
///     file module...
/// }
/// 
pub fn include(file_path_param: &str) -> String {
    // parse the file path
    let binding = file_path_param.replace("'", "").replace("\"", "");

    let mut contents : String;
    let mut file_path : String;

    // check if binding includes a ":"
    if binding.contains(":") {
        // this could be a core module
        let core_module = binding.split(":").collect::<Vec<_>>()[1];
        // file path
        file_path = core_module.to_string();
        // check with std
        contents = std::load_std(core_module);
    } else {
        file_path = binding;
        // read the file
        contents = read_file(&file_path);
    }

    // get an AST from the contents
    // create a lexer
    let lexer = lex::Lex::new_with_file(contents, file_path.to_string());
    // create a parser
    let mut parser = par::Parser::new(lexer);
    // parse the program
    let program = parser.parse_program();

    Transpiler::transpile_module(program)
}