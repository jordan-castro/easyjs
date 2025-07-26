use std::collections::HashMap;

use regex::Regex;

use super::macros::Macro;
use super::native::compile_native;
use crate::builtins;
use crate::compiler::runes::RuneParser;
// use crate::interpreter::{interpret_js, is_javascript_var_defined};
use crate::lexer::lex::{self, ALLOWED_IN_IDENT};
use crate::lexer::token;
use crate::parser::ast::{Expression, Statement};
use crate::parser::{ast, par};
use easy_utils::utils::{h::hash_string, js_helpers::is_javascript_keyword};

use super::import::import_file;

struct EasyType {
    /// The name of the type.
    type_name: String,
    // TODO: a type_schema.
    // type_schema: String
}

/// Variable data. (used mostly for scoping)
struct Variable {
    name: String,
    /// Is mutable
    is_mutable: bool,
    /// Variable type
    easy_type: Option<EasyType>,
}

/// Used only in transpiler and type checker.
/// Used to track native function calls.
///
/// i.e. convert native() to __easyjs_native_call("native", ["params"], ["returns"], ...args);
struct NativeFunction {
    params: Vec<String>,
    returns: Vec<String>,
    name: String,
}

/// Used only in transpiler and type checker.
/// Holds all native for project.
struct NativeContext {
    functions: Vec<NativeFunction>,
    variables: Vec<String>,
}

pub struct Transpiler {
    /// Stmt by Stmt
    scripts: Vec<String>,
    /// All EasyJS macros
    pub macros: HashMap<String, Macro>,
    /// All declared EasyJS functions
    pub functions: Vec<String>,
    /// All declared EasyJS structs.
    pub structs: Vec<String>,

    /// All declares structs within modules.
    pub structs_in_modules: Vec<String>,

    /// Keep a list of all scopes the EasyJS code has.
    scopes: Vec<Vec<Variable>>,

    /// Track all native statements
    native_stmts: Vec<Statement>,

    /// Track native variables and functions
    native_ctx: NativeContext,

    /// Are we in debug mode?
    pub debug_mode: bool,

    /// All imported modules (including stdlib)
    imported_modules: Vec<String>,

    /// Is this being compiled as a module
    is_module: bool,
}
/// Non Wasm specific (if running in non wasm enviroment, optionally save the wasm binary)
#[cfg(not(target_arch = "wasm32"))]
fn save_wasm_bin(wasm_bin: &Vec<u8>) {
    use std::io::Write;

    let mut file = std::fs::File::create("easyjs.wasm").unwrap();
    file.write(&wasm_bin).unwrap();
}
#[cfg(target_arch = "wasm32")]
fn save_wasm_bin(wasm_bin: &Vec<u8>) {
    // Empty body
}

impl Transpiler {
    pub fn new() -> Self {
        let mut t = Transpiler {
            scripts: vec![],
            functions: vec![],
            macros: HashMap::new(),
            // context: Context::default(),
            structs: vec![],
            scopes: vec![],
            structs_in_modules: vec![],
            native_stmts: vec![],
            native_ctx: NativeContext {
                functions: vec![],
                variables: vec![],
            },
            debug_mode: false,
            imported_modules: Vec::new(),
            is_module: false,
        };

        // Check the EASYJS_DEBUG variable
        let is_debug_mode_var = std::env::var("EASYJS_DEBUG");
        let is_debug_mode = if is_debug_mode_var.is_err() {
            false
        } else {
            is_debug_mode_var.unwrap() == "1"
        };

        t.debug_mode = is_debug_mode;

        // add the first scope. This scope will never be popped.
        t.add_scope();
        t
    }

    /// Add a statement to the native context.
    ///
    /// Only adds `pub` statements.
    ///
    /// pass in is_export to know if we are already inside a export statement.
    /// Otherwise any other stmt will fail if not inside an export.
    ///
    /// !Important: This is for converting a native function call into it's __easyjs_native_call equivalent.
    fn add_stmt_to_native_ctx(&mut self, stmt: &Statement, is_export: bool) {
        match stmt {
            Statement::ExportStatement(_, stmt) => {
                self.add_stmt_to_native_ctx(stmt, true);
            }
            Statement::ExpressionStatement(_, expr) => {
                if !is_export {
                    return;
                }

                self.add_expr_to_native_ctx(expr.as_ref());
            }
            _ => {
                return;
            }
        }
    }

    /// Add a expression to the native context.
    ///
    /// Currently only used for functions. Could potentially be used for global varaibles too, but not ideal atm.
    ///
    /// Advanced developers would not need to even use this. They could just call __easyjs_native_call directly.
    ///
    /// Or __easyjs_native_instance directly.
    fn add_expr_to_native_ctx(&mut self, expr: &Expression) {
        match expr {
            Expression::FunctionLiteral(_, name, params, result, _) => {
                // find out param types (as string...)
                let param_types = {
                    let mut param_types = vec![];
                    for param in params.as_ref().to_owned() {
                        match param {
                            Expression::IdentifierWithType(_, _, ty) => match ty.as_ref() {
                                Expression::Type(_, t) => {
                                    param_types.push(t.to_owned());
                                }
                                _ => {
                                    unimplemented!("TODO: how is this not a type?");
                                }
                            },
                            _ => {
                                unimplemented!("TODO: add some kind of error for no type.");
                            }
                        }
                    }

                    param_types
                };
                // find out return types (as string...)
                let return_types = {
                    if let Some(result) = result {
                        match result.as_ref() {
                            Expression::Type(_, ty) => ty.to_owned(),
                            _ => {
                                unimplemented!("TODO: add some kind of error for no type.");
                            }
                        }
                    } else {
                        unimplemented!("TODO: add some kind of error for no type.");
                    }
                };

                let fn_name = match name.as_ref() {
                    Expression::Identifier(_, n) => n.to_owned(),
                    _ => {
                        unimplemented!("TODO: add some kind of error for no fn_name.");
                    }
                };

                // add to ctx
                self.native_ctx.functions.push(NativeFunction {
                    params: param_types,
                    returns: vec![return_types],
                    name: fn_name,
                });
            }
            _ => {
                return;
            }
        }
    }

    pub fn reset(&mut self) {
        self.scripts = vec![];
    }

    /// Transpile a module.
    pub fn transpile_module(&mut self, p: ast::Program) -> String {
        let mut t = Transpiler::new();
        t.is_module = true;
        let js = t.transpile(p);
        self.native_ctx
            .functions
            .append(&mut t.native_ctx.functions);
        self.native_ctx
            .variables
            .append(&mut t.native_ctx.variables);
        // Have to add the imported native statements to the top first.
        let mut native_stmts = t.native_stmts.clone();
        native_stmts.append(&mut self.native_stmts);
        self.native_stmts = native_stmts;

        self.macros.extend(t.macros);
        js
    }

    /// Add a new scope
    fn add_scope(&mut self) {
        self.scopes.push(vec![]);
    }

    /// Remove last scope
    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    /// Convert transpiled JS into a string.
    fn to_string(&mut self) -> String {
        let mut res = String::new();

        // Only compile the native statments if we are not in a module and there are any.
        if !self.is_module && self.native_stmts.len() > 0 {
            // compiile native
            res.push_str(&self.transpile_native_stmts());
        }

        for script in self.scripts.iter() {
            res.push_str(&script);
        }

        res
    }

    /// Transpile easyjs code into JS from a string input.
    pub fn transpile_from_string(&mut self, p: String) -> String {
        let l = lex::Lex::new(p);
        let mut p = par::Parser::new(l);
        let program = p.parse_program();

        self.transpile_from(program)
    }

    /// Transpile easyjs code into JS from a ast program.
    pub fn transpile(&mut self, p: ast::Program) -> String {
        let code = self.transpile_from(p);
        code
    }

    /// Transpile easyjs code into JS from a ast program.
    fn transpile_from(&mut self, p: ast::Program) -> String {
        // seperate stmt types
        let mut native_stmts = p
            .statements
            .iter()
            .filter(|p| p.is_native())
            .collect::<Vec<_>>();
        let statements = p
            .statements
            .iter()
            .filter(|p| !p.is_native())
            .collect::<Vec<_>>();

        // compile native stmts first
        for stmt in native_stmts {
            if stmt.is_empty() {
                continue;
            }

            // get the stmts inside of Native
            match stmt {
                Statement::NativeStatement(_, stmts) => {
                    // loop through body
                    for stmt in stmts.as_ref() {
                        // add to native_stmts
                        self.native_stmts.push(stmt.clone());

                        // add to context
                        self.add_stmt_to_native_ctx(stmt, false);
                    }
                }
                _ => {
                    unimplemented!("TODO: error for not being a NativeStatement");
                }
            }
        }

        // transpile JS statements..
        for stmt in statements {
            if stmt.is_empty() {
                continue;
            }

            let script = self.transpile_stmt(stmt.to_owned());
            if let Some(script) = script {
                // add to context
                // let _ = interpret_js(&script, &mut self.context);
                self.scripts.push(script);
            }
        }
        self.to_string()
    }

    fn transpile_stmt(&mut self, stmt: ast::Statement) -> Option<String> {
        match stmt {
            ast::Statement::VariableStatement(token, name, ej_type, value, _) => {
                Some(self.transpile_var_stmt(
                    token,
                    name.as_ref().to_owned(),
                    ej_type.as_deref(),
                    value.as_ref().to_owned(),
                ))
            }
            ast::Statement::ReturnStatement(token, expression) => {
                Some(self.transpile_return_stmt(token, expression.as_ref().to_owned()))
            }
            ast::Statement::ImportStatement(_token, file_path) => {
                Some(self.transpile_import_stmt(&file_path))
            }
            ast::Statement::ExpressionStatement(token, expression) => {
                Some(self.transpile_expression_stmt(token, expression.as_ref().to_owned()))
            }
            ast::Statement::BlockStatement(token, stmts) => {
                Some(self.transpile_block_stmt(token, stmts.as_ref().to_owned()))
            }
            ast::Statement::ForStatement(token, condition, body) => Some(self.transpile_for_stmt(
                token,
                condition.as_ref().to_owned(),
                body.as_ref().to_owned(),
            )),
            ast::Statement::JavaScriptStatement(token, js) => {
                Some(self.transpile_javascript_stmt(token, js))
            }
            ast::Statement::StructStatement(
                token,
                name,
                constructor_vars,
                mixins,
                vars,
                methods,
            ) => Some(self.transpile_struct_stmt(
                name.as_ref().to_owned(),
                constructor_vars,
                mixins,
                vars.as_ref().to_owned(),
                methods.as_ref().to_owned(),
            )),
            Statement::ExportStatement(token, stmt) => {
                Some(self.transpile_export_stmt(token, stmt.as_ref().to_owned()))
            }
            Statement::AsyncBlockStatement(tk, block) => {
                Some(self.transpile_async_block_stmt(tk, block.as_ref().to_owned()))
            }
            Statement::MatchStatement(tk, expr, conditions) => Some(self.transpile_match_stmt(
                tk,
                expr.as_ref().to_owned(),
                conditions.as_ref().to_owned(),
            )),
            Statement::EnumStatement(tk, name, options) => {
                Some(self.transpile_enum_stmt(&name, options.as_ref()))
            }
            Statement::BreakStatement(tk) => Some("break".to_string()),
            Statement::ContinueStatement(tk) => Some("continue".to_string()),
            Statement::MacroStatement(_, name, paramaters, body) => {
                let macro_name: String = self.transpile_expression(name.as_ref().to_owned());
                let macro_params = paramaters.as_ref().to_owned();
                let macro_body = body.as_ref().to_owned();

                self.add_macro_function(macro_name, macro_params, macro_body);
                Some(String::from(""))
            }
            _ => None,
        }
    }

    fn transpile_enum_stmt(&mut self, name: &str, options: &Vec<Expression>) -> String {
        let mut result = String::new();

        if options.len() == 0 {
            return "".to_string();
        }

        result.push_str(format!("const {} = ", name).as_str());
        // result.push_str("enum ");
        result.push_str(" Object.freeze({");

        for i in 0..options.len() {
            let option = options.get(i).expect("");
            result.push_str(&self.transpile_expression(option.to_owned()));
            result.push_str(format!(": {}", i).as_str());
            if i < options.len() {
                result.push_str(",");
            }
        }

        result.push_str("});\n");
        result
    }

    fn transpile_import_stmt(&mut self, file_path: &str) -> String {
        // Check if already imported
        if self.imported_modules.contains(&file_path.to_string()) {
            return "".to_string();
        }
        // add to imported modules
        self.imported_modules.push(file_path.to_string());
        let contents = import_file(file_path);
        if contents == "".to_string() {
            return "".to_string();
        }

        let lexer = lex::Lex::new_with_file(contents, file_path.to_owned());
        let mut parser = par::Parser::new(lexer);
        let program = parser.parse_program();

        if parser.errors.len() > 0 {
            for e in parser.errors {
                println!("{}", e);
            }
            return "".to_string();
        }

        self.transpile_module(program)
    }

    fn transpile_native_stmts(&mut self) -> String {
        let mut res = String::new();
        let easy_wasm = compile_native(&self.native_stmts);
        if easy_wasm.is_err() {
            println!("Error: {}", easy_wasm.err().unwrap());
            // TODO: add error
            return res;
        }
        // It is ok now!
        let easy_wasm = easy_wasm.unwrap();
        // save file if in debug mode
        if self.debug_mode {
            save_wasm_bin(&easy_wasm);
        }
        res.push_str("const __easyjs_native_module = new Uint8Array([");
        for byte in easy_wasm {
            res.push_str(&byte.to_string());
            res.push_str(",");
        }
        res.push_str("]);\n");

        res.push_str("
            const __easyjs_native = new WebAssembly.Module(__easyjs_native_module);
            const __easyjs_native_instance = new WebAssembly.Instance(__easyjs_native);
            class __EasyJSNativeInterop {
                /**
                 * Function for converting a string to native.
                 */
                static convert_string_to_native(instance, str) {
                    // get length and bytes
                    const strLen = str.length;
                    const strBytes = new TextEncoder('utf-8').encode(str);

                    // allocate space and get pointer
                    const ptr = instance.exports.__str_alloc(strLen);

                    // store length
                    instance.exports.__str_store_len(ptr, strLen);

                    // Write the string to memory
                    for (let i = 0; i < strBytes.length; i++) {
                        instance.exports.__str_store_byte(ptr, 4 + i, strBytes[i]);
                    }
                    return ptr;
                }

                /**
                 * Function for reading a string from native.
                 */
                static read_string_from_native(instance, ptr) {
                    const length = instance.exports.__str_get_len(ptr);

                    const memoryBuffer = new Uint8Array(instance.exports.memory.buffer, ptr + 4, length);

                    // Decode the string
                    const decodedString = new TextDecoder('utf-8').decode(memoryBuffer);

                    return decodedString;
                }
            }

            function __easyjs_native_call(fnName, paramTypes, returnTypes, ...args) {
                if (!__easyjs_native_instance) {
                    throw new Error('No instance of __easyjs_native loaded');
                }

                if (!__easyjs_native_instance.exports[fnName]) {
                    throw new Error(`Function ${fnName} not found in __easyjs_native`);
                }

                if (paramTypes.length !== args.length) {
                    throw new Error('Number of arguments does not match number of parameters');
                }

                // go through params and make sure args match type
                for (let i = 0; i < args.length; i++) {
                    const arg = args[i];
                    const paramType = paramTypes[i];

                    switch (paramType) {
                        case 'string': {
                        if (typeof arg !== 'string') {
                            throw new Error(`Argument ${i} is not a string`);
                        }

                        // this is a string so we need to convert it to a native pointer.
                        args[i] = __EasyJSNativeInterop.convert_string_to_native(__easyjs_native_instance, args[i])
                        break;
                        }
                        case 'int': {
                        if (typeof arg !== 'number' || !Number.isInteger(arg)) {
                            throw new Error(`Argument ${i} is not an integer`);
                        }
                        break;
                        }
                        case 'float': {
                        if (typeof arg !== 'number' || isNaN(arg)) {
                            throw new Error(`Argument ${i} is not a valid float`);
                        }
                        break;
                        }
                        case 'bool': {
                        // booleans must be true/false or a number
                        if (typeof arg !== 'boolean' && typeof arg !== 'number') {
                            throw new Error(`Argument ${i} is not a valid boolean`);
                        }
                        // if true/false convert it to a int
                        if (typeof arg === 'boolean') {
                            args[i] = arg == true ? 1 : 0;
                        } else {
                            // make sure that the value is 0 or 1
                            args[i] = arg > 0 ? 1 : 0;
                        }
                        break;
                        }
                    }
                }

                let result = __easyjs_native_instance.exports[fnName](...args);

                // match result type
                // TODO: support multiple return types
                switch (returnTypes[0]) {
                    case 'string': {
                        // get length
                        result = __EasyJSNativeInterop.read_string_from_native(__easyjs_native_instance, result);
                        break;
                    }
                    case 'int': {
                        break;
                    }
                    case 'float': {
                        break;
                    }
                    case 'bool': {
                        result = result == 0 ? false : true
                        break;
                    }
                }

                return result;
            }\n\n
        ");

        res
    }

    fn transpile_match_stmt(
        &mut self,
        token: token::Token,
        expr: Expression,
        conditions: Vec<(Expression, Statement)>,
    ) -> String {
        let mut res = String::new();
        res.push_str("switch ");
        // transpile expr
        res.push_str(&format!("({})", self.transpile_expression(expr)));
        res.push_str("{ \n");

        let mut has_default = false;
        for (condition, stmt) in conditions {
            let formatted_condition = self.transpile_expression(condition);
            if formatted_condition == "_" {
                has_default = true;

                res.push_str(
                    format!(
                        "default: \n\t{} \n\t break;\n",
                        self.transpile_stmt(stmt).unwrap()
                    )
                    .as_str(),
                );

                continue;
            }
            res.push_str("case ");
            res.push_str(&format!("{}: ", formatted_condition));
            res.push_str(&self.transpile_stmt(stmt).unwrap());
            res.push_str("\n\t break;\n");
        }

        if !has_default {
            res.push_str(" default: \n\tbreak; \n");
        }
        res.push_str("}\n");

        res
    }

    fn transpile_doc_comment_expr(&mut self, token: token::Token, comments: Vec<String>) -> String {
        let mut res = String::new();
        res.push_str("\n/**\n"); // start doc

        for comment in comments {
            res.push_str(format!(" * {}\n", comment).as_str());
        }

        res.push_str("*/\n"); // end doc
        res
    }

    fn transpile_async_block_stmt(&mut self, token: token::Token, block: ast::Statement) -> String {
        let mut res = String::new();
        res.push_str("(async function() {");
        match block {
            Statement::BlockStatement(tk, stmts) => {
                res.push_str(&self.transpile_block_stmt(tk, stmts.as_ref().to_owned()));
            }
            _ => {
                res.push_str("");
            }
        }
        res.push_str("})();\n");

        res
    }

    fn transpile_export_stmt(&mut self, token: token::Token, stmt: ast::Statement) -> String {
        format!("export {};\n", self.transpile_stmt(stmt).unwrap())
    }

    fn transpile_var_stmt(
        &mut self,
        token: token::Token,
        name: ast::Expression,
        ej_type: Option<&ast::Expression>,
        value: ast::Expression,
    ) -> String {
        let name_string = self.transpile_expression(name.clone());

        // check if this already exists in scope
        let mut found = false;
        for var in self.scopes.iter().rev() {
            for v in var.iter() {
                if v.name == name_string {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }

        if !found {
            // check for type
            let mut easy_type: Option<EasyType> = None;
            if let Some(ej_type) = ej_type {
                easy_type = Some(EasyType {
                    type_name: self.transpile_expression(ej_type.to_owned()),
                });
            }

            self.scopes.last_mut().unwrap().push(Variable {
                name: name_string.clone(),
                is_mutable: true,
                easy_type,
            });
            format!(
                "let {} = {};\n",
                name_string,
                self.transpile_expression(value)
            )
        } else {
            format!("{} = {};\n", name_string, self.transpile_expression(value))
        }
    }

    fn transpile_return_stmt(
        &mut self,
        token: token::Token,
        expression: ast::Expression,
    ) -> String {
        format!("return {};\n", self.transpile_expression(expression))
    }

    /// Transpile the high level macro block stmt
    ///
    /// A macro does not have it's own scope, that is the only difference really.
    fn transpile_macro_block_stmt(&mut self, stmts: Vec<Statement>) -> String {
        let mut response = String::new();
        for stmt in stmts {
            if let Some(stmt) = self.transpile_stmt(stmt) {
                response.push_str(&stmt);
            }
        }
        response
    }

    fn transpile_block_stmt(&mut self, token: token::Token, stmts: Vec<ast::Statement>) -> String {
        self.add_scope();
        let mut response = String::new();
        for stmt in stmts {
            if let Some(stmt) = self.transpile_stmt(stmt) {
                response.push_str(&stmt);
            }
        }
        self.pop_scope();
        response
    }

    // fn transpile_const_var_stmt(
    //     &mut self,
    //     token: token::Token,
    //     name: ast::Expression,
    //     value: ast::Expression,
    // ) -> String {
    //     let left = self.transpile_expression(name.clone());
    //     let value = self.transpile_expression(value);

    //     // search for the variable in scope.
    //     let mut found = false;
    //     for scope in self.scopes.iter() {
    //         for v in scope.iter() {
    //             if v.name == left {
    //                 found = true;
    //                 break;
    //             }
    //         }
    //     }

    //     if found {
    //         format!("{} = {};\n", &left, &value)
    //     } else {
    //         self.scopes.last_mut().unwrap().push(Variable {
    //             name: left.clone(),
    //             is_mutable: false
    //         });
    //         format!("const {} = {};\n", &left, &value)
    //     }
    // }

    fn transpile_javascript_stmt(&mut self, token: token::Token, js: String) -> String {
        format!("\n{}\n", js)
    }

    fn transpile_for_stmt(
        &mut self,
        token: token::Token,
        condition: ast::Expression,
        body: ast::Statement,
    ) -> String {
        let mut res = String::new();
        match condition {
            ast::Expression::Boolean(token, value) => {
                res.push_str(format!("while({})", value).as_str());
            }
            ast::Expression::InfixExpression(token, left, operator, right) => {
                res.push_str(
                    format!(
                        "while({} {} {}) ",
                        self.transpile_expression(left.as_ref().to_owned()),
                        operator,
                        self.transpile_expression(right.as_ref().to_owned())
                    )
                    .as_str(),
                );
            }
            ast::Expression::OfExpression(token, left, right) => {
                res.push_str(
                    format!(
                        "for (let {} of {}) ",
                        self.transpile_expression(left.as_ref().to_owned()),
                        self.transpile_expression(right.as_ref().to_owned())
                    )
                    .as_str(),
                );
            }
            Expression::InExpression(token, left, right) => match right.as_ref().to_owned() {
                Expression::RangeExpression(token, start, end) => {
                    // TODO: error for if end is empty

                    let ident: String = self.transpile_expression(left.as_ref().to_owned());
                    res.push_str("for (let ");
                    res.push_str(&ident);

                    res.push_str(" = ");
                    res.push_str(&self.transpile_expression(start.as_ref().to_owned()));
                    res.push_str(";");
                    res.push_str(&ident);
                    res.push_str(" < ");
                    res.push_str(&self.transpile_expression(end.as_ref().to_owned()));
                    res.push_str(";");
                    res.push_str(&ident);
                    res.push_str("++");
                    res.push_str(") ");
                }
                _ => res.push_str(
                    format!(
                        "for (let {} of {}) ",
                        self.transpile_expression(left.as_ref().to_owned()),
                        self.transpile_expression(right.as_ref().to_owned())
                    )
                    .as_str(),
                ),
            },
            _ => panic!("Condition must be boolean"),
        }

        res.push_str("{\n");

        let stmt = self.transpile_stmt(body);

        if let Some(stmt) = stmt {
            res.push_str(&stmt);
        }

        res.push_str("}\n");

        res
    }

    fn transpile_expression_stmt(
        &mut self,
        token: token::Token,
        expression: ast::Expression,
    ) -> String {
        let has_semicolon = match expression {
            Expression::FunctionLiteral(_, _, _, _, _) => false,
            Expression::DocCommentExpression(_, _) => false,
            _ => true,
        };
        let res = self.transpile_expression(expression);
        let semi = if has_semicolon { ";\n" } else { "" };
        format!("{}{}", res, semi)
    }

    fn transpile_struct_stmt(
        &mut self,
        name: ast::Expression,
        constructor_vars: Option<Box<Vec<Expression>>>,
        mixins: Option<Box<Vec<ast::Expression>>>,
        variables: Vec<ast::Statement>,
        methods: Vec<ast::Expression>,
    ) -> String {
        let mut res = String::new();
        let mut parsed_mixins = vec![];

        if let Some(mixins) = mixins {
            for mixin in mixins.as_ref().to_owned() {
                let mixin_name = self.transpile_expression(mixin.clone());
                parsed_mixins.push(mixin_name);
            }
        }

        res.push_str("function ");
        let struct_name = self.transpile_expression(name);
        self.structs.push(struct_name.clone());
        res.push_str(&struct_name);
        res.push_str("(");

        let mut struct_vars = vec![];
        // add construcot variables
        if let Some(vars) = constructor_vars {
            let vars = vars.as_ref().to_owned();
            for i in 0..vars.len() {
                let var = &vars[i];
                let var_name = self.transpile_expression(var.clone());
                struct_vars.push((var_name.clone(), None));
                res.push_str(&var_name);
                // only if it is not the last variable add a comma
                if i != vars.len() - 1 {
                    res.push_str(", ");
                }
            }
        }

        // close constructor
        res.push_str(") {\n");

        // add variables.
        // static variables are added now while struct variables are added at the end.
        for var in variables {
            match var {
                // ast::Statement::ConstVariableStatement(_, name, _, value, _) => {
                //     let name = self.transpile_expression(name.as_ref().to_owned());
                //     let value = self.transpile_expression(value.as_ref().to_owned());

                //     res.push_str(format!("{}.{} = {};\n", struct_name, name, value).as_str());
                // }
                ast::Statement::VariableStatement(_, name, _, value, _) => {
                    let name = self.transpile_expression(name.as_ref().to_owned());
                    let value = self.transpile_expression(value.as_ref().to_owned());

                    struct_vars.push((name, Some(value)));
                }
                _ => {}
            }
        }

        // add methods.
        // static methods are added now while struct methods are added at the end.
        let mut cleaned_methods = vec![];

        for method in methods {
            let mut result = String::new();
            let cleaned_method_is_static = self.get_struct_method_function_exp(method);
            match cleaned_method_is_static.0.clone() {
                Expression::DocCommentExpression(tk, comments) => {
                    result = self.transpile_doc_comment_expr(tk, comments);
                }
                Expression::FunctionLiteral(_, name, params, _, body) => {
                    result = (self.transpile_struct_method(
                        &struct_name,
                        cleaned_method_is_static.0,
                        false,
                        cleaned_method_is_static.1,
                    ));
                }
                Expression::AsyncExpression(_, function) => {
                    result = (self.transpile_struct_method(
                        &struct_name,
                        function.as_ref().to_owned(),
                        true,
                        cleaned_method_is_static.1,
                    ));
                }
                _ => {}
            }
            if cleaned_method_is_static.1 {
                res.push_str(&result);
            } else {
                cleaned_methods.push(result);
            }
        }

        // begin returning object.
        res.push_str("return ");
        if parsed_mixins.len() > 0 {
            res.push_str(" Object.assign({\n");
        } else {
            res.push_str("{\n");
        }

        // add struct variables second
        for (name, value) in struct_vars {
            res.push_str(&name);
            if let Some(value) = value {
                res.push_str(": ");
                res.push_str(&value);
            }
            res.push_str(", ");
        }
        // add methods last
        for method in cleaned_methods {
            res.push_str(&method);
        }
        // close struct
        res.push_str("}\n");

        if parsed_mixins.len() > 0 {
            res.push_str(", ");
            for mixin in parsed_mixins {
                res.push_str(&mixin);
                res.push_str("(),");
            }
            res.push_str(");\n");
        }

        // close the functoin
        res.push_str("}\n");
        // }\n
        res
    }

    fn transpile_expression(&mut self, expression: ast::Expression) -> String {
        match expression {
            ast::Expression::IntegerLiteral(token, value) => value.to_string(),
            Expression::StringLiteral(token, value) => {
                let quote_type =
                    if (&value.contains("$")).to_owned() || (&value.contains("\n")).to_owned() {
                        "`"
                    } else if (&value.contains("'")).to_owned() {
                        "\""
                    } else {
                        "\'"
                    };

                // supporting string $ interpolation.
                // 1. check if string contains $
                // 2. get the positions of all $
                // 3. if any of the positions is followed by a {, then ignore it because this should be interpreted by itself.
                // 4. for all other positions, get the start and end position of the identifier using lex::ALLOWED_IN_IDENT.contains(char)
                // 5. once we have the start and end position of the identifier, add ${} around the identifier
                let mut str_value = string_interpolation(&value);

                if quote_type == "`" {
                    // extract expressions
                    let rp = RuneParser::new(str_value.clone());
                    let expressions = rp.expressions;

                    for expression in expressions {
                        let mut internal_t = Transpiler::new();
                        internal_t.macros = self.macros.clone();
                        let mut response = internal_t.transpile_from_string(expression.clone());
                        response = response.trim().to_string();
                        if response.ends_with(";") {
                            response = response.strip_suffix(';').unwrap().to_string();
                        }
                        str_value = str_value.replace(
                            format!("${{{}}}", expression).as_str(),
                            format!("${{{}}}", response).as_str(),
                        );
                    }
                }

                format!("{}{}{}", quote_type, str_value, quote_type)
            }
            Expression::PrefixExpression(token, op, value) => {
                format!(
                    // "({}{})",
                    "{}{}",
                    op,
                    self.transpile_expression(value.as_ref().to_owned())
                )
            }
            Expression::InfixExpression(token, left, operator, right) => {
                format!(
                    // "({} {} {})",
                    "{} {} {}",
                    self.transpile_expression(left.as_ref().to_owned()),
                    operator,
                    self.transpile_expression(right.as_ref().to_owned())
                )
            }
            Expression::GroupedExpression(token, expression) => {
                format!(
                    "({})",
                    self.transpile_expression(expression.as_ref().to_owned())
                )
            }
            Expression::IfExpression(token, condition, consequence, elseif, else_) => {
                let mut res = String::new();

                res.push_str("if (");
                res.push_str(&self.transpile_expression(condition.as_ref().to_owned()));
                res.push_str(") {\n");
                res.push_str(
                    self.transpile_stmt(consequence.as_ref().to_owned())
                        .unwrap()
                        .as_str(),
                );
                res.push_str("}");

                // check for elseif and else_
                if !elseif.is_empty() {
                    res.push_str("else ");
                    res.push_str(&self.transpile_expression(elseif.as_ref().to_owned()));
                }

                if !else_.is_empty() {
                    res.push_str("else { \n");
                    let stmt = self.transpile_stmt(else_.as_ref().to_owned());
                    if let Some(stmt) = stmt {
                        res.push_str(&stmt);
                    }
                    res.push_str("}");
                }

                res
            }
            Expression::FunctionLiteral(token, name, paramters, _, body) => {
                let mut res = String::new();

                res.push_str("function ");
                match name.as_ref().to_owned() {
                    Expression::Identifier(token, value) => {
                        self.functions.push(value.clone());
                        res.push_str(&value);
                        res.push_str(" (");
                    }
                    _ => panic!("Function names must be IDENT."),
                }

                let ps = paramters.as_ref().to_owned();
                let joined_params = ps
                    .iter()
                    .map(|p| self.transpile_expression(p.to_owned()))
                    .collect::<Vec<_>>()
                    .join(",");
                res.push_str(&joined_params);
                res.push_str(")");

                res.push_str("{\n");
                let stmt = self.transpile_stmt(body.as_ref().to_owned());
                if let Some(stmt) = stmt {
                    res.push_str(&stmt);
                }
                res.push_str("}\n");

                res
            }
            Expression::CallExpression(token, name, arguments) => {
                let mut res = String::new();

                let name_exp = self.transpile_expression(name.as_ref().to_owned());

                // check if name_exp exists in native_ctx
                let native_fn = self
                    .native_ctx
                    .functions
                    .iter()
                    .find(|f| f.name == name_exp);
                if native_fn.is_some() {
                    let native_fn = native_fn.unwrap();
                    // native call
                    res.push_str("__easyjs_native_call(");
                    // add name
                    res.push_str(format!("'{}',", native_fn.name).as_str());

                    // add param_types
                    res.push_str("[");
                    for param_type in native_fn.params.iter() {
                        res.push_str(format!("'{}'", param_type).as_str());
                        res.push_str(",");
                    }
                    res.push_str("],");

                    // add return types
                    res.push_str("[");
                    for return_type in native_fn.returns.iter() {
                        res.push_str(format!("'{}'", return_type).as_str());
                        res.push_str(",");
                    }
                    res.push_str("]");

                    // check if we need to add a ','
                    if arguments.as_ref().len() > 0 {
                        res.push_str(",");
                    }
                } else {
                    res.push_str(&name_exp);
                    res.push_str("(");
                }

                // parse the args
                let mut parsed_args = vec![];
                // the named args if any
                let mut named_args: Vec<Vec<ast::Expression>> = vec![];

                // keep track if there are named parameters.
                let mut has_named = false;
                // ok, double check the arguments
                for arg in arguments.as_ref().to_owned() {
                    match arg {
                        ast::Expression::AssignExpression(_t, left, right) => {
                            has_named = true;
                            named_args
                                .push(vec![left.as_ref().to_owned(), right.as_ref().to_owned()]);
                        }
                        _ => {
                            if has_named {
                                // this is bad because you can't have non named after named
                                panic!("Non named parameter after named parameter");
                            } else {
                                parsed_args.push(arg);
                            }
                        }
                    }
                }

                let args = arguments.as_ref().to_owned();
                let joined_args = parsed_args
                    .iter()
                    .map(|p| self.transpile_expression(p.to_owned()))
                    .collect::<Vec<_>>()
                    .join(",");
                res.push_str(&joined_args);

                if (has_named) {
                    if parsed_args.len() > 0 {
                        res.push_str(",");
                    }
                    res.push_str("{");
                    for i in 0..named_args.len() {
                        let arg = &named_args[i];
                        res.push_str(&self.transpile_expression(arg.first().unwrap().to_owned()));
                        res.push_str(":");
                        res.push_str(&self.transpile_expression(arg.last().unwrap().to_owned()));

                        if i < named_args.len() - 1 {
                            res.push_str(",");
                        }
                    }
                    res.push_str("}");
                }

                res.push_str(")");

                res
            }
            Expression::Boolean(token, value) => {
                format!("{}", value)
            }
            Expression::Identifier(token, name) => {
                // hash the string if it is a JS keyword
                if is_javascript_keyword(&name) {
                    hash_string(&name)
                } else {
                    name
                }
            }
            Expression::IdentifierWithType(token, name, var_type) => {
                if is_javascript_keyword(&name) {
                    hash_string(&name)
                } else {
                    name
                }
            }
            Expression::DotExpression(token, left, right) => {
                let mut res = String::new();

                res.push_str(&self.transpile_expression(left.as_ref().to_owned()));
                res.push_str(".");
                let mut r = self.transpile_expression(right.as_ref().to_owned());

                if r.starts_with("(") {
                    r = r[1..r.len() - 1].to_string();
                }
                res.push_str(&r);

                res
            }
            Expression::LambdaLiteral(token, paramters, body) => {
                let mut res = String::new();

                res.push_str("(");
                let params = paramters.as_ref().to_owned();
                let joined_params = params
                    .iter()
                    .map(|p| self.transpile_expression(p.to_owned()))
                    .collect::<Vec<_>>()
                    .join(",");
                res.push_str(&joined_params);
                res.push_str(") => {\n");
                let stmt = self.transpile_stmt(body.as_ref().to_owned());
                if let Some(stmt) = stmt {
                    res.push_str(&stmt);
                }
                res.push_str("}");

                res
            }
            Expression::ArrayLiteral(token, elements) => {
                let mut res = String::new();

                res.push_str("[");
                let els = elements.as_ref().to_owned();
                res.push_str(&self.join_expressions(elements.as_ref().to_owned()));
                res.push_str("]");

                res
            }
            Expression::IndexExpression(token, left, index) => {
                let mut res = String::new();

                res.push_str(&self.transpile_expression(left.as_ref().to_owned()));
                match index.as_ref() {
                    Expression::RangeExpression(tk, start, end) => {
                        res.push_str(
                            format!(
                                ".slice({},",
                                self.transpile_expression(start.as_ref().to_owned()),
                                // self.transpile_expression(end.as_ref().to_owned())
                            )
                            .as_str(),
                        );
                        match end.as_ref() {
                            Expression::EmptyExpression => {
                                res.push_str(
                                    format!(
                                        "{}.length)",
                                        self.transpile_expression(left.as_ref().to_owned())
                                    )
                                    .as_str(),
                                );
                            }
                            _ => {
                                res.push_str(&self.transpile_expression(end.as_ref().to_owned()));
                                res.push_str(")");
                            }
                        }
                    }
                    _ => {
                        res.push_str(
                            format!(
                                "[{}]",
                                &self.transpile_expression(index.as_ref().to_owned())
                            )
                            .as_str(),
                        );
                    }
                }

                res
            }
            Expression::ObjectLiteral(token, properties) => {
                let mut res = String::new();

                res.push_str("{");

                for i in 0..properties.len() {
                    let p = properties.get(i).unwrap();
                    let key = p.first().unwrap().as_ref().to_owned();
                    let value = p.last().unwrap().as_ref().to_owned();

                    res.push_str(&self.transpile_expression(key.clone()));
                    if &key.get_token() != &value.get_token() {
                        res.push_str(":");
                        res.push_str(&self.transpile_expression(value));
                    }
                    if i != properties.len() - 1 {
                        res.push_str(",\n");
                    }
                }
                res.push_str("}");

                res
            }
            Expression::AsyncExpression(token, exp) => {
                let mut res = String::new();
                res.push_str("async ");
                res.push_str(&self.transpile_expression(exp.as_ref().to_owned()));
                res
            }
            Expression::AwaitExpression(token, exp) => {
                format!(
                    "await {}",
                    self.transpile_expression(exp.as_ref().to_owned())
                )
            }
            Expression::InExpression(token, left, right) => {
                let mut res = String::new();

                res.push_str(&self.transpile_expression(right.as_ref().to_owned()));
                res.push_str(".includes(");
                res.push_str(&self.transpile_expression(left.as_ref().to_owned()));
                res.push_str(")");

                res
            }
            Expression::OfExpression(token, left, right) => {
                format!(
                    "{} of {}",
                    self.transpile_expression(left.as_ref().to_owned()),
                    self.transpile_expression(right.as_ref().to_owned())
                )
            }
            // Expression::RangeExpression(token, start, end) => {
            //     format!(
            //         "builtins.{}({},{})",
            //         builtins::INT_RANGE,
            //         self.transpile_expression(start.as_ref().to_owned()),
            //         self.transpile_expression(end.as_ref().to_owned())
            //     )
            // }
            Expression::AssignExpression(token, left, right) => {
                let left: String = self.transpile_expression(left.as_ref().to_owned());
                format!(
                    "{} = {}",
                    left,
                    self.transpile_expression(right.as_ref().to_owned())
                )
            }
            Expression::NotExpression(token, exp) => {
                format!("!{}", self.transpile_expression(exp.as_ref().to_owned()))
            }
            Expression::AsExpression(token, left, right) => {
                format!(
                    "{} as {}",
                    self.transpile_expression(left.as_ref().to_owned()),
                    self.transpile_expression(right.as_ref().to_owned())
                )
            }
            Expression::IIFE(_, block) => {
                format!(
                    "(() => {{\n{}\n}})()",
                    self.transpile_stmt(block.as_ref().to_owned()).unwrap()
                )
            }
            Expression::AndExpression(token, left, right) => {
                format!(
                    "{} && {}",
                    self.transpile_expression(left.as_ref().to_owned()),
                    self.transpile_expression(right.as_ref().to_owned())
                )
            }
            Expression::OrExpression(token, left, right) => {
                format!(
                    "{} || {}",
                    self.transpile_expression(left.as_ref().to_owned()),
                    self.transpile_expression(right.as_ref().to_owned())
                )
            }
            Expression::NullExpression(token) => "null".to_string(),
            Expression::DefaultIfNullExpression(token, left, right) => {
                format!(
                    "{} ?? {}",
                    self.transpile_expression(left.as_ref().to_owned()),
                    self.transpile_expression(right.as_ref().to_owned())
                )
            }
            Expression::NewClassExpression(token, exp) => {
                format!("new {}", self.transpile_expression(exp.as_ref().to_owned()))
            }
            Expression::FloatLiteral(token, value) => format!("{}", value),
            Expression::IsExpression(_tk, left, right) => {
                format!(
                    "typeof({}) == {}",
                    self.transpile_expression(left.as_ref().to_owned()),
                    self.transpile_expression(right.as_ref().to_owned())
                )
            }
            Expression::MacroExpression(_, name, arguments) => {
                let macro_name = self.transpile_expression(name.as_ref().to_owned());
                let macro_arguments = arguments.as_ref().to_owned();

                // parse the body first.
                let transpiled_body = if let Some(mac) = self.macros.get(&macro_name) {
                    // parse the macro body
                    match &mac.body {
                        Statement::BlockStatement(tk, stmts) => {
                            let mut body =
                                self.transpile_macro_block_stmt(stmts.as_ref().to_owned());
                            body = body[0..body.len() - 1].to_string();

                            if body.ends_with(';') {
                                body = body.strip_suffix(';').unwrap().to_string();
                            }

                            body
                        }
                        _ => "".to_string(),
                    }
                } else {
                    "".to_string()
                };

                // check if macro args are empty
                if macro_arguments.len() == 0 {
                    if let Some(mac) = self.macros.get(&macro_name) {
                        return mac.compile(vec![], transpiled_body);
                    }
                }
                let macro_arguments: Vec<String> = macro_arguments
                    .iter()
                    .map(|p| self.transpile_expression(p.to_owned()))
                    .collect::<Vec<String>>();

                if let Some(mac) = self.macros.get(&macro_name) {
                    mac.compile(macro_arguments, transpiled_body)
                } else {
                    String::from("")
                }
            }
            Expression::SpreadExpression(tk, expression) => {
                format!(
                    "...{}",
                    self.transpile_expression(expression.as_ref().to_owned())
                )
            }
            Expression::EmptyExpression => String::from(""),
            Expression::DocCommentExpression(token, comments) => {
                self.transpile_doc_comment_expr(token, comments)
            }
            _ => String::from(""),
        }
    }

    fn join_expressions(&mut self, expressions: Vec<Expression>) -> String {
        expressions
            .iter()
            .map(|p| self.transpile_expression(p.to_owned()))
            .collect::<Vec<_>>()
            .join(",")
    }

    /// Add a macro function to later be used when calling.
    fn add_macro_function(&mut self, name: String, params: Vec<Expression>, body: Statement) {
        let pms = self.join_expressions(params.to_owned());
        let mut parsed_args = vec![];

        for a in pms.split(",") {
            parsed_args.push(a.to_string());
        }

        // let body = match body {
        //     Statement::BlockStatement(_, stmts) => {
        //         self.transpile_macro_block_stmt(stmts.as_ref().to_owned())
        //     }
        //     _ => "".to_string()
        // };

        // add the body up to the last ';\n'
        self.macros.insert(
            name.to_owned(),
            Macro::new(name, parsed_args, body), // Macro::new(name, parsed_args, body[0..body.len() - 1].to_string()),
        );
    }

    /// Transpile a struct method
    fn transpile_struct_method(
        &mut self,
        struct_name: &str,
        method: ast::Expression,
        is_async: bool,
        is_static: bool,
    ) -> String {
        let mut res = String::new();
        match method {
            Expression::FunctionLiteral(_, name, params, _, body) => {
                let name = self.transpile_expression(name.as_ref().to_owned());
                let params = {
                    let params = params.as_ref().to_owned();
                    let mut res = String::new();
                    for i in 0..params.len() {
                        let param = &params[i];
                        res.push_str(&self.transpile_expression(param.clone()));
                        // only if it is not the last variable add a comma
                        if i != params.len() - 1 {
                            res.push_str(", ");
                        }
                    }
                    res
                };

                let body = self.transpile_stmt(body.as_ref().to_owned());

                if is_static {
                    res.push_str(format!("{}.{} = ", struct_name, &name).as_str());
                } else {
                    res.push_str(format!("{}: ", &name).as_str());
                }

                if is_async {
                    res.push_str("async ");
                }
                res.push_str(format!("function({})", &params).as_str());
                res.push_str("{");

                if let Some(body) = body {
                    res.push_str(&body);
                }
                res.push_str("}");
                if is_static {
                    res.push_str(";\n");
                } else {
                    res.push_str(",\n");
                }
            }
            _ => {}
        }

        res
    }

    /// This gets the actual data for the struct method expression.
    /// Takes in `method` and returns the inner workings, and whether or not it is static..
    fn get_struct_method_function_exp(
        &mut self,
        method: ast::Expression,
    ) -> (ast::Expression, bool) {
        match method.clone() {
            Expression::DocCommentExpression(_, _) => {
                return (method, false);
            }
            Expression::AsyncExpression(tk, fn_method) => {
                let result = self.get_struct_method_function_exp(fn_method.as_ref().to_owned());
                return (
                    Expression::AsyncExpression(tk, Box::new(result.0)),
                    result.1,
                );
            }
            Expression::FunctionLiteral(fn_token, fn_name, params, var_type, body) => {
                // // check if is a predescribed method like new => constructor
                // if self.transpile_expression(fn_name.as_ref().to_owned()) == "new" {
                //     return (
                //         Expression::FunctionLiteral(
                //             fn_token,
                //             Box::new(Expression::Identifier(
                //                 token::new_token(token::IDENT, "constructor"),
                //                 "constructor".to_string(),
                //             )),
                //             params,
                //             body,
                //         ),
                //         false,
                //     );
                // }

                // check for a self param
                if params.len() == 0 {
                    return (method.to_owned(), true);
                }

                let first_param = params.get(0).unwrap().to_owned();
                if self.transpile_expression(first_param) == "this" {
                    // this is a non static method.
                    return (
                        Expression::FunctionLiteral(
                            fn_token,
                            fn_name,
                            Box::new(params.as_ref().to_owned()[1..].to_vec()),
                            var_type,
                            body,
                        ),
                        false,
                    );
                } else {
                    return (
                        Expression::FunctionLiteral(fn_token, fn_name, params, var_type, body),
                        true,
                    );
                }
            }
            _ => {}
        }
        (method, false)
    }
}

/// Interpolate the string with $$$$$
fn string_interpolation(input: &str) -> String {
    let mut result = String::new();

    let mut listen_for_ending = false;
    let mut found_at = 0;

    let chars: Vec<_> = input.chars().collect(); // Collect chars to access by index

    for i in 0..chars.len() {
        let c = chars[i];
        let next_char = chars.get(i + 1).copied().unwrap_or(' '); // Look ahead safely
        let prev_char = if i > 0 { chars[i - 1] } else { ' ' }; // Look behind safely

        if c == '$' && prev_char != '\\' && next_char != '{' {
            if listen_for_ending {
                result.push('}');
            }
            listen_for_ending = true;
            found_at = i;
            result.push('$');
            result.push('{');
            continue;
        }

        if listen_for_ending && i > found_at {
            if !c.is_alphabetic() && !ALLOWED_IN_IDENT.contains(c) {
                listen_for_ending = false; // Stop listening if a non-alphabetic char is found
                result.push('}');
            }
        }

        result.push(c);

        // Close the interpolation block if at the end of input and still listening
        if listen_for_ending && i == chars.len() - 1 {
            result.push('}');
        }
    }

    result
}
