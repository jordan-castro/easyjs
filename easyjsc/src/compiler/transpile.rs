use std::collections::HashMap;
use std::iter::Map;

use easyjs_utils::utils::sanatize;
use regex::Regex;

use super::macros::Macro;
use super::native::compile_native;
use crate::builtins;
use crate::compiler::namespaces::{Function, Namespace, Struct, Variable};
use crate::compiler::runes::RuneParser;
// use crate::interpreter::{interpret_js, is_javascript_var_defined};
use crate::lexer::lex::{self, ALLOWED_IN_IDENT};
use crate::lexer::token;
use crate::parser::ast::{Expression, Statement};
use crate::parser::{ast, par};
use crate::typechecker::{
    StrongValType, get_param_type_by_string, get_param_type_by_string_ej, get_string_rep_of_type,
};
use easyjs_utils::utils::{h::hash_string, js_helpers::is_javascript_keyword};

use super::import::import_file;

pub struct Transpiler {
    /// Stmt by Stmt
    scripts: Vec<String>,
    /// The namespace of this transpiler.
    pub namespace: Namespace,

    /// All internal modules/namespaces.
    pub modules: Vec<Namespace>,

    /// Keep a list of all scopes the EasyJS code has.
    scopes: Vec<Vec<Variable>>,

    /// Track all native statements
    native_stmts: Vec<Statement>,

    /// Are we in debug mode?
    pub debug_mode: bool,

    /// Is this being compiled as a module
    is_module: bool,

    /// Custom libraries
    custom_libs: HashMap<String, String>,
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
            namespace: Namespace::new("".to_string(), "_".to_string()),
            modules: vec![],
            scopes: vec![],
            native_stmts: vec![],
            debug_mode: false,
            is_module: false,
            custom_libs: HashMap::new(),
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

    /// Create a transpiler with custom libs
    pub fn with_custom_libs(custom_libs: HashMap<String, String>) -> Self {
        let mut t = Transpiler::new();
        t.custom_libs = custom_libs;

        t
    }

    /// Apply Namespace mangling to native statement.
    ///
    /// Call this to get the correct NativeStatement.
    fn apply_namespace_mangling_to_native(&self, stmt: &Statement) -> Statement {
        match stmt {
            Statement::ExportStatement(tk, stmt) => Statement::ExportStatement(
                tk.to_owned(),
                Box::new(self.apply_namespace_mangling_to_native(stmt)),
            ),
            Statement::ExpressionStatement(tk, expr) => Statement::ExpressionStatement(
                tk.to_owned(),
                Box::new(self.apply_namespace_mangling_to_native_expr(&expr)),
            ),
            Statement::VariableStatement(tk, name, val_type, value, infer_type) => {
                let name_identifier = match name.as_ref() {
                    Expression::Identifier(tk, name) => (tk, name),
                    _ => {
                        unreachable!("It is not possible to reach this in a varaible statement.")
                    }
                };
                Statement::VariableStatement(
                    tk.to_owned(),
                    Box::new(Expression::Identifier(
                        name_identifier.0.to_owned(),
                        self.namespace.get_obj_name(name_identifier.1),
                    )),
                    val_type.to_owned(),
                    value.to_owned(),
                    *infer_type,
                )
            }
            _ => stmt.to_owned(),
        }
    }

    /// Apply Namespace mangling to native expressions.
    ///
    /// Call this to get the correct Expression.
    fn apply_namespace_mangling_to_native_expr(&self, expression: &Expression) -> Expression {
        match expression {
            Expression::FunctionLiteral(tk, name, params, return_type, body) => {
                let name_transpiled = match name.as_ref() {
                    Expression::Identifier(tk, identifier) => (tk, identifier),
                    _ => {
                        panic!("Not possible that function name is not a IDENTIFIER")
                    }
                };
                let name_expression = Expression::Identifier(
                    name_transpiled.0.to_owned(),
                    self.namespace.get_obj_name(name_transpiled.1),
                );
                Expression::FunctionLiteral(
                    tk.to_owned(),
                    Box::new(name_expression),
                    params.to_owned(),
                    return_type.to_owned(),
                    body.to_owned(),
                )
            }
            _ => expression.to_owned(),
        }
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

                // add to native ctx
                self.namespace.native_ctx.functions.push(Function {
                    name: fn_name,
                    params: param_types
                        .iter()
                        .map(|v| Variable {
                            name: String::from(""),
                            is_mut: true,
                            val_type: get_param_type_by_string(v),
                        })
                        .collect::<Vec<Variable>>(),
                    return_type: get_param_type_by_string(&return_types),
                })
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
    pub fn transpile_module(&mut self, file_name: &str, alias: &str, p: ast::Program) -> String {
        let mut t = Transpiler::new();
        t.is_module = true;
        // Clean the filename
        let cleaned_file_name = sanatize::get_filename_without_extension(file_name);
        t.namespace.id = cleaned_file_name;
        t.namespace.alias = alias.to_string();

        // Transpile the code now.
        let js = t.transpile(p);

        // Add the namespace to our modules
        self.modules.push(t.namespace.clone());

        // Check if this namespace goes into global scope
        if alias == "_" {
            self.namespace.variables.extend(t.namespace.variables);
            // also extend variable scope
            if let (Some(self_inner), Some(t_inner)) = (self.scopes.get_mut(0), t.scopes.get(0)) {
                self_inner.extend(t_inner.iter().cloned());
            }

            self.namespace.functions.extend(t.namespace.functions);
            self.namespace.structs.extend(t.namespace.structs);
            self.namespace.macros.extend(t.namespace.macros);
        }

        if t.native_stmts.len() > 0 {
            // extend native_stmts
            let mut new_native_stmts = t.native_stmts.clone();
            new_native_stmts.extend(self.native_stmts.clone());
            self.native_stmts = new_native_stmts;

            // also extend native_ctx
            // This will always go to the global scope ma G.
            self.namespace
                .native_ctx
                .functions
                .extend(t.namespace.native_ctx.functions);
            self.namespace
                .native_ctx
                .variables
                .extend(t.namespace.native_ctx.variables);
        }

        // return JS code
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
                        let mangled_stmt = self.apply_namespace_mangling_to_native(stmt);
                        // add to native_stmts
                        self.native_stmts.push(mangled_stmt.clone());

                        // add to context
                        self.add_stmt_to_native_ctx(&mangled_stmt.clone(), false);
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
            ast::Statement::ImportStatement(_token, file_path, alias) => {
                Some(self.transpile_import_stmt(&file_path, alias))
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
            Statement::ClassStatement(tk, name, extends, stmts) => {
                Some(self.transpile_class_stmt(&tk, name.as_ref(), extends.as_ref(), &stmts))
            }
            _ => None,
        }
    }

    fn transpile_internal_class_stmt(
        &mut self,
        class_name: &String,
        stmt: &Statement,
        is_pub: bool,
    ) -> String {
        match stmt {
            Statement::ExportStatement(_, stmt) => {
                self.transpile_internal_class_stmt(class_name, stmt, true)
            }
            Statement::VariableStatement(tk, ident, type_, value, should_infer) => {
                let tag = { if is_pub { "" } else { "#" } };

                let var_name = self.transpile_expression(ident.as_ref().to_owned());
                let var_result: String = self.transpile_expression(value.as_ref().to_owned());

                format!("{tag}{var_name}={var_result}\n\n")
            }
            Statement::ExpressionStatement(tk, expr) => {
                let mut result = String::new();
                match expr.as_ref() {
                    Expression::FunctionLiteral(tk, fn_name, params, type_, block) => {
                        // Check name of function
                        let mut fn_name_parsed =
                            self.transpile_expression(fn_name.as_ref().to_owned());
                        let mut fn_is_pub = is_pub;
                        match fn_name_parsed.as_str() {
                            "__new__" => {
                                fn_name_parsed = "constructor".to_string();
                                fn_is_pub = true;
                            }
                            _ => {}
                        }
                        // Check if static or non static
                        let mut is_static = true;
                        let mut cleaned_params = vec![];
                        for param in params.as_ref() {
                            match param {
                                Expression::Identifier(_, ident) => {
                                    if ident == "self" {
                                        is_static = false;
                                    }
                                }
                                Expression::IdentifierWithType(_, ident, _) => {
                                    if ident == "self" {
                                        is_static = false;
                                    }
                                }
                                _ => {
                                    cleaned_params.push(param.to_owned());
                                }
                            }
                        }
                        let tag = { if fn_is_pub { "" } else { "#" } };

                        let function = Expression::FunctionLiteral(
                            tk.to_owned(),
                            Box::new(Expression::Identifier(
                                fn_name.get_token().to_owned(),
                                format!("{tag}{fn_name_parsed}"),
                            )),
                            Box::new(cleaned_params),
                            type_.to_owned(),
                            block.to_owned(),
                        );
                        // Transpile the function but just removed the `function` keyword from the beginning
                        let tf = self.transpile_expression(function);
                        // Remove 'function'
                        // FUNCTION = 8 len
                        result.push_str(&tf.trim()[8..]);
                        result.push('\n');
                        result
                    }
                    _ => {
                        return String::from("");
                    }
                }
            }
            // Statement::MacroStatement(_, _, _, _) => {
            // }
            _ => {
                // Could not parse
                // TODO: Error of some sort
                return String::from("");
            }
        }
    }

    fn transpile_class_stmt(
        &mut self,
        tk: &token::Token,
        name: &Expression,
        extends: &Vec<Expression>,
        stmts: &Vec<Statement>,
    ) -> String {
        let mut result = String::new();

        let mut class_name: String;
        let mut base_name: String;
        match name {
            Expression::Identifier(_, ident) => {
                base_name = self.namespace.get_obj_name(ident);
                class_name = format!("__EASYJS_{}_INTERNAL", base_name);
                result
                    .push_str(format!("const {class_name} = Base => class extends Base ").as_str());
            }
            _ => {
                return String::from("");
            }
        };
        result.push('{');

        // Variables, Expressions...
        for stmt in stmts {
            let sss = self.transpile_internal_class_stmt(&class_name, stmt, false);
            result.push_str(sss.as_str());
        }

        result.push('}');

        // Ok now let's actually create our class.
        result.push_str(format!("\nclass {base_name} extends ").as_str());

        // Extensions
        let mut times_extended = 0;
        if extends.len() > 0 {
            for expr in extends {
                times_extended += 1;
                let mut real_class_name: String;
                match expr {
                    Expression::Identifier(_, ident) => {
                        real_class_name = format!("__EASYJS_{ident}_INTERNAL");
                    }
                    Expression::DotExpression(_, _, _) => {
                        real_class_name = format!(
                            "__EASYJS_{}_INTERNAL",
                            self.transpile_expression(expr.to_owned())
                        );
                    }
                    _ => {
                        return String::from("");
                    }
                }

                // Add extension
                result.push_str(format!("{real_class_name}(").as_str());
            }
        }
        result.push_str(class_name.as_str());
        result.push_str("(class{})");
        for i in 0..times_extended {
            result.push(')');
        }
        result.push_str("{}");
        result
    }

    fn transpile_enum_stmt(&mut self, name: &str, options: &Vec<Expression>) -> String {
        let mut result = String::new();

        if options.len() == 0 {
            return "".to_string();
        }

        result.push_str(
            format!(
                "const {} = ",
                self.namespace.get_obj_name(&name.to_string())
            )
            .as_str(),
        );
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

    fn transpile_import_stmt(&mut self, file_path: &str, alias: Option<Box<Expression>>) -> String {
        // Check if already imported
        if self.modules.iter().any(|v| v.id == file_path) {
            return "".to_string();
        }
        // Load contents
        let contents = import_file(file_path, &self.custom_libs);
        if contents == "".to_string() {
            return "".to_string();
        }

        // Parse the code.
        let lexer = lex::Lex::new_with_file(contents, file_path.to_owned());
        let mut parser = par::Parser::new(lexer);
        let program = parser.parse_program();

        if parser.errors.len() > 0 {
            for e in parser.errors {
                println!("{}", e);
            }
            return "".to_string();
        }

        // Grab the alias, if any.
        let alias_string: String;
        if let Some(alias) = alias {
            alias_string = self.transpile_expression(alias.as_ref().to_owned());
        } else {
            alias_string = String::from("");
        }

        self.transpile_module(file_path, &alias_string, program)
    }

    fn transpile_native_stmts(&mut self) -> String {
        let mut res = String::new();
        let easy_wasm = compile_native(&self.native_stmts, &self.namespace, &self.modules);
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
        res.push_str(include_str!("../native_runner.js"));

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
        let transpiled_name = self.transpile_expression(name.clone());
        let name_string = self.namespace.get_obj_name(&transpiled_name);

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
            let mut val_type: StrongValType = StrongValType::None;
            if let Some(ej_type) = ej_type {
                // transpile
                let type_name = self.transpile_expression(ej_type.to_owned());
                val_type = get_param_type_by_string_ej(&type_name);
            }

            // Add to scope
            self.scopes.last_mut().unwrap().push(Variable {
                name: name_string.clone(),
                is_mut: true,
                val_type: val_type.clone(),
            });
            // Add to namespace if there is only one scope i.e. global scope.
            if self.scopes.len() == 1 {
                // Add to namespace
                self.namespace.variables.push(Variable {
                    name: name_string.clone(),
                    is_mut: true,
                    val_type: val_type,
                });
            }
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
            Expression::MacroExpression(_, _, _) => false,
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
        // Struct namespace holders.
        let mut struct_params: Vec<Variable> = vec![];
        let mut struct_variables: Vec<Variable> = vec![];
        let mut struct_methods: Vec<Function> = vec![];
        let mut struct_static_methods: Vec<Function> = vec![];

        if let Some(mixins) = mixins {
            for mixin in mixins.as_ref().to_owned() {
                let mixin_name = self.transpile_expression(mixin.clone());
                parsed_mixins.push(mixin_name);
            }
        }

        res.push_str("function ");
        let name_transpiled = self.transpile_expression(name);
        let struct_name = self.namespace.get_obj_name(&name_transpiled);

        // Compile the actual namespaced version.
        res.push_str(&struct_name);
        // res.push_str(&struct_name);
        res.push_str("(");

        let mut struct_vars = vec![];
        // add construcot variables
        if let Some(vars) = constructor_vars {
            let vars = vars.as_ref().to_owned();
            for i in 0..vars.len() {
                let var = &vars[i];
                let mut var_name: String;
                let mut val_type: StrongValType = StrongValType::None;
                match var {
                    Expression::Identifier(_, name) => {
                        var_name = name.to_owned();
                    }
                    Expression::IdentifierWithType(_, name, var_type) => {
                        var_name = name.to_owned();
                        val_type = get_param_type_by_string_ej(
                            &self.transpile_expression(var_type.as_ref().to_owned()),
                        );
                    }
                    _ => {
                        panic!("TODO: some transpiler error for struct variables")
                    }
                }
                // Add to transpilation process. name: value
                struct_vars.push((var_name.clone(), None));
                res.push_str(&var_name);
                // only if it is not the last variable add a comma
                if i != vars.len() - 1 {
                    res.push_str(", ");
                }

                // Add to namespace struct params
                struct_params.push(Variable {
                    name: var_name,
                    is_mut: true,
                    val_type,
                });
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
                    let mut val_type = StrongValType::None;

                    match name.as_ref() {
                        Expression::IdentifierWithType(_, _, var_type) => {
                            val_type = get_param_type_by_string_ej(
                                &self.transpile_expression(var_type.as_ref().to_owned()),
                            );
                        }
                        _ => {}
                    }

                    // TODO: infer type automatically

                    let name = self.transpile_expression(name.as_ref().to_owned());
                    let value = self.transpile_expression(value.as_ref().to_owned());

                    struct_vars.push((name.clone(), Some(value)));

                    // Also add to struct_variable
                    struct_variables.push(Variable {
                        name,
                        is_mut: false,
                        val_type,
                    });
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
                Expression::FunctionLiteral(_, name, params, return_val_type, body) => {
                    result = (self.transpile_struct_method(
                        &struct_name,
                        cleaned_method_is_static.0,
                        false,
                        cleaned_method_is_static.1,
                    ));

                    let fn_name = &self.transpile_expression(name.as_ref().to_owned());
                    // Add to struct methods
                    let function = self.create_namespace_function(fn_name, params, return_val_type);

                    if cleaned_method_is_static.1 {
                        struct_static_methods.push(function);
                    } else {
                        struct_methods.push(function);
                    }
                }
                Expression::AsyncExpression(_, function) => {
                    result = (self.transpile_struct_method(
                        &struct_name,
                        function.as_ref().to_owned(),
                        true,
                        cleaned_method_is_static.1,
                    ));

                    // Add to struct methods
                    match function.as_ref() {
                        Expression::FunctionLiteral(_, name, params, return_type, body) => {
                            let fn_name = &self.transpile_expression(name.as_ref().to_owned());
                            let namespace_function = self.create_namespace_function(
                                fn_name,
                                params.to_owned(),
                                return_type.to_owned(),
                            );
                            if cleaned_method_is_static.1 {
                                struct_static_methods.push(namespace_function);
                            } else {
                                struct_methods.push(namespace_function);
                            }
                        }
                        _ => {
                            panic!("Transpiler error: This has to be a function.")
                        }
                    }
                    // let function = self.create_namespace_function(name, params, return_type)
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

        // add struct to namespace
        self.namespace.structs.push(Struct {
            name: struct_name,
            params: struct_params,
            variables: struct_variables,
            methods: struct_methods,
            static_methods: struct_static_methods,
        });

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
                        internal_t.namespace = self.namespace.clone();
                        internal_t.modules = self.modules.clone();

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
            Expression::FunctionLiteral(token, name, paramters, return_type, body) => {
                let mut res = String::new();

                // add to namespace
                let fn_name = self.transpile_expression(name.as_ref().to_owned());
                {
                    let namespace_function =
                        self.create_namespace_function(&fn_name, paramters.clone(), return_type);
                    self.namespace.functions.push(namespace_function);
                }

                res.push_str(
                    format!("function {}(", self.namespace.get_obj_name(&fn_name)).as_str(),
                );

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
                    .namespace
                    .native_ctx
                    .functions
                    .iter()
                    .find(|f| f.name == self.namespace.get_obj_name(&name_exp));
                if native_fn.is_some() {
                    res.push_str(&self.transpile_native_function_with_args(
                        &self.namespace,
                        &self.namespace.get_obj_name(&name_exp),
                        arguments.as_ref().to_owned(),
                    ));
                } else {
                    res.push_str(&name_exp);
                    res.push_str("(");
                }

                // parse the args
                let parsed_args = self
                    .transpile_call_arguments(arguments.as_ref().to_owned())
                    .join(",");
                res.push_str(&parsed_args);
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

                // Check if the left side transpiled is actually a namespace.
                let left_side = self.transpile_expression(left.as_ref().to_owned());

                let cloned_modules = self.modules.clone();
                for namespace in cloned_modules.iter() {
                    if namespace.has_name(&left_side) {
                        // It is a namespace!
                        // There are only 3 possibilities for the right side:
                        // - A call expression
                        // - A identifier
                        // - A dot expression
                        let new_right = self.convert_namespaced_dot_expression(namespace, &right);
                        res.push_str(&self.transpile_expression(new_right));
                    }
                }

                // Otherwise it is a regular dot expression.
                if res.len() == 0 {
                    let right_side = self.transpile_expression(right.as_ref().to_owned());
                    res.push_str(&left_side);
                    res.push_str(".");
                    let mut r = right_side.clone();

                    if r.starts_with("(") {
                        r = r[1..r.len() - 1].to_string();
                    }
                    res.push_str(&r);
                }
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
                res.push_str(
                    match body.as_ref() {
                        Statement::EmptyStatement => String::from(" return undefined; "),
                        Statement::ExpressionStatement(token, expression) => {
                            let mut finish = String::from("return ");
                            let compiled =
                                self.transpile_expression(expression.as_ref().to_owned());
                            finish.push_str(&compiled);
                            finish
                        }
                        Statement::BlockStatement(token, statements) => {
                            let stmt = self.transpile_stmt(body.as_ref().to_owned()).unwrap();
                            stmt
                        }
                        _ => unimplemented!(),
                    }
                    .as_str(),
                );

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
                // Check if this has a namespace
                let mut macro_object: Option<Macro> = None;

                // Check macro namespace is correct, or udapet it
                let full_macro_name = self.transpile_expression(name.as_ref().to_owned());

                if full_macro_name.contains('.') {
                    let parts: Vec<&str> = full_macro_name.split('.').collect();

                    if let (Some(ns), Some(name_part)) = (parts.first(), parts.get(1)) {
                        if let Some(found_namespace) = self
                            .modules
                            .iter()
                            .find(|namespace| namespace.has_name(&ns.to_string()))
                        {
                            // Set the macro object.
                            if let Some(found_macro) = found_namespace
                                .macros
                                .get(&found_namespace.get_obj_name(&name_part.to_string()))
                            {
                                macro_object = Some(found_macro.clone());
                            }
                        }
                    } else {
                        unreachable!("How can this be reached in Macros?")
                        // Fallback if split failed
                        // if let Some(found_macro) = self.namespace.macros.get
                    }
                } else {
                    if let Some(found_macro) = self.namespace.macros.get(&full_macro_name) {
                        macro_object = Some(found_macro.clone());
                    }
                }
                if macro_object.is_none() {
                    return "".to_string();
                }
                let macro_object = macro_object.unwrap();
                let macro_arguments = arguments.as_ref().to_owned();

                // parse the body first.
                let transpiled_body = match &macro_object.body {
                    Statement::BlockStatement(tk, stmts) => {
                        let mut body = self.transpile_macro_block_stmt(stmts.as_ref().to_owned());
                        // body = body[0..body.len() - 1].to_string();

                        if body.ends_with(';') {
                            body = body.strip_suffix(';').unwrap().to_string();
                        }

                        body
                    }
                    Statement::ExpressionStatement(tk, macro_expro) => {
                        self.transpile_expression(macro_expro.as_ref().to_owned())
                    }
                    _ => "".to_string(),
                };

                // Check for named macro arguments
                let macro_arguments = self.transpile_call_arguments(macro_arguments);

                let macro_arguments =
                    self.lineup_macro_args(macro_arguments, macro_object.paramaters.clone());
                macro_object.compile(macro_arguments, transpiled_body)
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

        let macro_name = self.namespace.get_obj_name(&name);
        for a in pms.split(",") {
            parsed_args.push(a.to_string());
        }

        let ej_macro = Macro::new(macro_name.clone(), parsed_args, body);
        // add to namespace
        self.namespace.macros.insert(macro_name, ej_macro);
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

    /// Transpile the arguments in a call.
    ///
    /// Works for CallExpression and MacroExpression
    fn transpile_call_arguments(&mut self, arguments: Vec<Expression>) -> Vec<String> {
        let mut result = vec![];
        let mut has_named = false;
        let mut named_params = String::new();

        for i in 0..arguments.len() {
            let argument = arguments[i].clone();
            match argument {
                // Expression::Identifier(tk, name) => {
                //     if has_named {
                //         // TODO: better errors
                //         panic!("Can not have unnmaed after named");
                //     }
                //     result.push_str(&name);
                //     if i < arguments.len() - 1 {
                //         result.push_str(",");
                //     }
                // }
                Expression::AssignExpression(tk, ident, value) => {
                    if !has_named {
                        has_named = true;
                        named_params.push_str("{");
                    }
                    let ident_parsed = self.transpile_expression(ident.as_ref().to_owned());
                    let value_parsed = self.transpile_expression(value.as_ref().to_owned());

                    named_params.push_str(format!("'{ident_parsed}': {value_parsed},").as_str());
                }
                _ => {
                    if has_named {
                        // TODO: better errors
                        panic!("Can not have unnamed after named");
                    }

                    result.push(self.transpile_expression(argument));
                }
            }
        }

        if has_named {
            named_params.push_str("}");
            result.push(named_params);
        }

        result
    }

    /// Lineup macro arguments to correctly pass in:
    ///
    /// regular arguments
    /// default paramaters
    /// n number arguments.
    fn lineup_macro_args(
        &self,
        macro_arguments: Vec<String>,
        macro_params: Vec<String>,
    ) -> Vec<String> {
        let mut result = vec![];

        for (i, param) in macro_params.iter().enumerate() {
            if param.contains("=") {
                // This is a default paramater
                let value = param
                    .split("=")
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap()
                    .trim()
                    .to_string();

                if let Some(macro_param) = macro_arguments.get(i) {
                    result.push(macro_param.to_owned());
                } else {
                    result.push(value);
                }
            } else if param.contains("...") {
                // This is a n number of params
                let mut n_args = String::new();
                for j in i..macro_arguments.len() {
                    n_args.push_str(&macro_arguments.get(j).unwrap().to_string());
                    if j < macro_arguments.len() - 1 {
                        n_args.push_str(",");
                    }
                }
                result.push(n_args);
            } else {
                if macro_arguments.len() <= i {
                    result.push(String::from("undefined"));
                } else {
                    // Regular
                    result.push(macro_arguments.get(i).unwrap().to_string());
                }
            }
        }
        result
    }

    /// Create a Namespace function from name, params, and return_type
    fn create_namespace_function(
        &mut self,
        name: &String,
        params: Box<Vec<Expression>>,
        return_type: Option<Box<Expression>>,
    ) -> Function {
        let function_type = {
            if let Some(return_type) = return_type {
                get_param_type_by_string_ej(
                    &self.transpile_expression(return_type.as_ref().to_owned()),
                )
            } else {
                StrongValType::None
            }
        };

        let fn_name: String;
        if !name.contains('.') {
            fn_name = self.namespace.get_obj_name(name);
        } else {
            fn_name = name.to_owned();
        }

        Function {
            name: fn_name,
            params: params
                .as_ref()
                .to_owned()
                .iter()
                .map(|v| match v {
                    Expression::Identifier(_, name) => Variable {
                        name: name.to_owned(),
                        is_mut: true,
                        val_type: StrongValType::None,
                    },
                    Expression::IdentifierWithType(_, name, type_name) => Variable {
                        name: name.to_owned(),
                        is_mut: true,
                        val_type: get_param_type_by_string_ej(
                            &self.transpile_expression(type_name.as_ref().to_owned()),
                        ),
                    },
                    _ => {
                        panic!("Some transpiler error for params in structs.")
                    }
                })
                .collect(),
            return_type: function_type,
        }
    }

    /// Transpile a native function call with arguments.
    ///
    /// Arguments can be empty.
    ///
    /// Function name MUST ALREADY BE created!!!
    fn transpile_native_function_with_args(
        &self,
        namespace: &Namespace,
        fn_name: &str,
        args: Vec<Expression>,
    ) -> String {
        let mut res = String::new();
        // Get native function
        let native_fn = namespace
            .native_ctx
            .functions
            .iter()
            .find(|v| v.name == fn_name);
        if native_fn.is_some() {
            let native_fn = native_fn.unwrap();
            res.push_str("__easyjs_native_call(");
            // add name
            res.push_str(format!("'{}',", native_fn.name).as_str());

            // add param types
            res.push_str("[");
            for param_type in native_fn.params.iter() {
                res.push_str(&format!(
                    "'{}'",
                    get_string_rep_of_type(&param_type.val_type)
                ));
                res.push_str(",");
            }
            res.push_str("], ");

            // add return types
            res.push_str("[");
            res.push_str(format!("'{}'", get_string_rep_of_type(&native_fn.return_type)).as_str());
            res.push_str("]");

            // Not sure I need this?
            if args.len() > 0 {
                res.push_str(",");
            }
        }

        res
    }

    /// Convert a namespaced DotExpression.
    ///
    /// This is for when a namespaced dot expressions right side is another dot expression.
    ///
    /// We could have:
    /// import 'c.ej'
    /// import 'std'
    ///
    /// native {
    ///    fn test() {
    ///       @std.print(c.variable)
    ///       // or
    ///       @std.print(c.method().x)
    ///    }
    /// }
    fn convert_namespaced_dot_expression(
        &mut self,
        namespace: &Namespace,
        expression: &Expression,
    ) -> Expression {
        match expression {
            Expression::Identifier(token, name) => {
                Expression::Identifier(token.to_owned(), namespace.get_obj_name(name))
            }
            Expression::CallExpression(token, name, args) => {
                let name_as_string = self.transpile_expression(name.as_ref().to_owned());
                Expression::CallExpression(
                    token.to_owned(),
                    Box::new(Expression::Identifier(
                        name.get_token().to_owned(),
                        namespace.get_obj_name(&name_as_string),
                    )),
                    args.to_owned(),
                )
            }
            Expression::DotExpression(token, left, right) => {
                let new_left = self.convert_namespaced_dot_expression(namespace, left);
                Expression::DotExpression(token.to_owned(), Box::new(new_left), right.to_owned())
            }
            Expression::AssignExpression(tk, left, right) => {
                let new_left = self.convert_namespaced_dot_expression(namespace, &left);
                Expression::AssignExpression(tk.to_owned(), Box::new(new_left), right.to_owned())
            }
            _ => expression.to_owned(),
        }
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
