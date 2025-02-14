// use boa_engine::{value, Context};
use std::collections::HashMap;
use std::fmt::format;
use std::io::Write;
use std::path;

use super::macros::Macro;
use crate::emitter::wasm_emitter::emit_wasm;
use crate::{builtins, emitter};
// use crate::interpreter::{interpret_js, is_javascript_var_defined};
use crate::lexer::lex::{self, ALLOWED_IN_IDENT};
use crate::parser::ast::{Expression, Statement};
use crate::parser::{ast, par};
use crate::utils::js_helpers::is_javascript_keyword;
use crate::{lexer::token, utils::h::hash_string};

use super::import::{get_js_module_name, import, ImportType};

/// Variable data. (used mostly for scoping)
struct Variable {
    name: String, 
    /// Is mutable
    is_mutable: bool
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
    native_stmts: Vec<Statement>
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
            native_stmts: vec![]
        };

        // add the first scope. This scope will never be popped.
        t.add_scope();
        t
    }

    pub fn reset(&mut self) {
        self.scripts = vec![];
    }

    /// Add a module to the current scope.
    ///
    /// `module: Transpiler` the modules transpiler.
    #[deprecated(since = "0.2.1", note = "use `transpile_module` instead")]
    pub fn add_module(&mut self, module: &mut Transpiler) {
        // self.functions.append(&mut module.functions);
        // self.variables.append(&mut module.variables);
        self.structs.append(&mut module.structs);
    }

    /// Transpile a module.
    #[deprecated(since = "0.4.0", note = "easyjs supports ES6 modules now, this is no longer needed.")]
    pub fn transpile_module(p : ast::Program) -> String {
        let mut t = Transpiler::new();

        let mut res = String::new();

        res.push_str("(function() {\n");

        // exported identifiers
        let mut exported_names = vec![];

        for stmt in p.statements {
            let mut export_name = String::new();
            if stmt.is_empty() {
                continue;
            }

            // check if the statement is an export
            match stmt.clone() {
                // add it to a list of exported identifiers
                Statement::ExportStatement(tk, stmt) => {
                    match stmt.as_ref().to_owned() {
                        Statement::VariableStatement(_, name, _, _, _) => {
                            export_name = t.transpile_expression(name.as_ref().to_owned());
                        }
                        Statement::ConstVariableStatement(_, name, _, _, _) => {
                            export_name = t.transpile_expression(name.as_ref().to_owned());
                        }
                        Statement::StructStatement(_, name, _, _, _, _) => {
                            export_name = t.transpile_expression(name.as_ref().to_owned());
                        }
                        Statement::ExpressionStatement(_, expression) => {
                            match expression.as_ref().to_owned() {
                                Expression::Identifier(_, name) => {
                                    exported_names.push(name);
                                }
                                // TODO: continue module work
                                // Expression::
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                    // let name = t.transpile_stmt(stmt.as_ref().to_owned());

                    // if let Some(name) = name {
                    //     // name is between the decleration and the identifier
                    //     let name = name.split(" ").collect::<Vec<_>>()[1].to_string();
                    //     exported_names.push(name);
                    // }
                },
                _ => {}
            }
            
            if !export_name.is_empty() {
                exported_names.push(export_name);
            }

            // transpile the statement
            let script = t.transpile_stmt(stmt);
            if let Some(script) = script {
                // check if script starts with "export"
                if script.starts_with("export") {
                    // remove the beginning of the script
                    let script = script.split(" ").collect::<Vec<_>>()[1..].join(" ");
                    res.push_str(&script);
                } else {
                    res.push_str(&script);
                }
                res.push_str("\n");
            }
        }

        // return all exported values...
        res.push_str("return {");
        for name in exported_names {
            res.push_str(format!("{},", name).as_str());
        }
        res.push_str("};\n");

        // close the module
        res.push_str("})();\n");

        res
    }

    /// Add a new scope
    fn add_scope(&mut self) {
        self.scopes.push(vec![]);
    }

    /// Remove last scope
    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn to_string(&self) -> String {
        let mut res = String::new();

        // TODO: compile native...
        // let compiled_native = emitter::wasm_emitter::emit_wasm(self.native_stmts.clone());
        // res.push_str("const n_a_t_i_v_e = new WebAssembly.Instance(new WebAssembly.Module(Uint8Array.from([");
        // for byte in compiled_native {
        //     res.push_str(&format!("{},", byte));
        // }
        // res.push_str("])));\n");

        for script in self.scripts.iter() {
            res.push_str(&script);
        }

        res
    }

    pub fn transpile_from_string(&mut self, p: String) -> String {
        let l = lex::Lex::new(p);
        let mut p = par::Parser::new(l);
        let program = p.parse_program();

        self.transpile_from(program)
    }

    pub fn transpile(&mut self, p: ast::Program) -> String {
        let code = self.transpile_from(p);
        code
    }

    fn transpile_from(&mut self, p: ast::Program) -> String {
        for stmt in p.statements {
            if stmt.is_empty() {
                continue;
            }

            let script = self.transpile_stmt(stmt);
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
            ast::Statement::VariableStatement(token, name, _, value, _) => Some(self.transpile_var_stmt(
                token,
                name.as_ref().to_owned(),
                value.as_ref().to_owned(),
            )),
            ast::Statement::ReturnStatement(token, expression) => {
                Some(self.transpile_return_stmt(token, expression.as_ref().to_owned()))
            }
            ast::Statement::UseStatement(token, prefix, path) => Some(self.transpile_use_stmt(
                token,
                prefix.as_ref().to_owned(),
                path.as_ref().to_owned(),
            )),
            ast::Statement::UseFromStatement(token, specs, prefix, path) => {
                Some(self.transpile_use_from_stmt(
                    token,
                    specs.as_ref().to_owned(),
                    prefix.as_ref().to_owned(),
                    path.as_ref().to_owned(),
                ))
            }
            ast::Statement::ExpressionStatement(token, expression) => {
                Some(self.transpile_expression_stmt(token, expression.as_ref().to_owned()))
            }
            ast::Statement::BlockStatement(token, stmts) => {
                Some(self.transpile_block_stmt(token, stmts.as_ref().to_owned()))
            }
            ast::Statement::ConstVariableStatement(token, name, _, value, _) => {
                Some(self.transpile_const_var_stmt(
                    token,
                    name.as_ref().to_owned(),
                    value.as_ref().to_owned(),
                ))
            }
            ast::Statement::ForStatement(token, condition, body) => Some(self.transpile_for_stmt(
                token,
                condition.as_ref().to_owned(),
                body.as_ref().to_owned(),
            )),
            ast::Statement::JavaScriptStatement(token, js) => {
                Some(self.transpile_javascript_stmt(token, js))
            }
            ast::Statement::StructStatement(token, name, constructor_vars, mixins, vars, methods) => {
                Some(self.transpile_struct_stmt(
                    name.as_ref().to_owned(),
                    constructor_vars,
                    mixins,
                    vars.as_ref().to_owned(),
                    methods.as_ref().to_owned(),
                ))
            }
            Statement::ExportStatement(token, stmt) => {
                Some(self.transpile_export_stmt(token, stmt.as_ref().to_owned()))
            }
            Statement::AsyncBlockStatement(tk, block) => {
                Some(self.transpile_async_block_stmt(tk, block.as_ref().to_owned()))
            }
            Statement::DocCommentStatement(tk, comments) => {
                Some(self.transpile_doc_comment_stmt(tk, comments))
            }
            Statement::MatchStatement(tk, expr, conditions) => {
                Some(self.transpile_match_stmt(tk, expr.as_ref().to_owned(), conditions.as_ref().to_owned()))
            }
            Statement::NativeStatement(tk, stmts) => {
                Some(self.transpile_native_stmt(stmts.as_ref().to_owned()))
            }
            // Statement::NativeStatement(tk, stmts) => {
            //     // add statments
            //     for stmt in stmts.as_ref().to_owned() {
            //         self.native_stmts.push(stmt);
            //     }
            //     None
            // }
            _ => None,
        }
    }

    fn transpile_native_stmt(&mut self, stmts: Vec<Statement>) -> String {
        let mut res = String::new();
        let easy_wasm = emit_wasm(stmts);
        // save the wasm to a file
        let mut file = std::fs::File::create("easyjs.wasm").unwrap();
        file.write(&easy_wasm).unwrap();

        res
    }

    fn transpile_match_stmt(&mut self, token: token::Token, expr: Expression, conditions: Vec<(Expression, Statement)>) -> String {
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
                
                res.push_str(format!("default: \n\t{} \n\t break;\n", self.transpile_stmt(stmt).unwrap()).as_str());

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

    fn transpile_doc_comment_stmt(&mut self, token: token::Token, comments: Vec<String>) -> String {
        let mut res = String::new();
        res.push_str("/**\n"); // start doc

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
        value: ast::Expression,
    ) -> String {
        let name_string = self.transpile_expression(name.clone());

        self.scopes.last_mut().unwrap().push(Variable {
            name: name_string.clone(),
            is_mutable: true
        });
        format!(
            "let {} = {};\n",
            name_string,
            self.transpile_expression(value)
        )
    }

    fn transpile_return_stmt(
        &mut self,
        token: token::Token,
        expression: ast::Expression,
    ) -> String {
        format!("return {};\n", self.transpile_expression(expression))
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

    fn transpile_const_var_stmt(
        &mut self,
        token: token::Token,
        name: ast::Expression,
        value: ast::Expression,
    ) -> String {
        let left = self.transpile_expression(name.clone());
        let value = self.transpile_expression(value);

        // search for the variable in scope.
        let mut found = false;
        for scope in self.scopes.iter() {
            for v in scope.iter() {
                if v.name == left {
                    found = true;
                    break;
                }
            }
        }

        if found {
            format!("{} = {};\n", &left, &value)
        } else {
            self.scopes.last_mut().unwrap().push(Variable {
                name: left.clone(),
                is_mutable: false
            });
            format!("const {} = {};\n", &left, &value)
        }
    }

    fn get_module_n_path(&mut self, exp: Expression) -> (String, String) {
        match exp.to_owned() {
            ast::Expression::Identifier(_token, path) => {
                let module_name = get_js_module_name(path.split(".").last().unwrap());
                let path_to_use = path;
                (module_name, path_to_use)
            }
            ast::Expression::AsExpression(_tk, left, right) => {
                let module_name = self.transpile_expression(right.as_ref().to_owned());
                let path_to_use = self.transpile_expression(left.as_ref().to_owned());
                (module_name, path_to_use)
            }
            ast::Expression::StringLiteral(_tk, lit) => {
                let module_name = get_js_module_name(&lit);
                let path_to_use = lit;
                (module_name, path_to_use)
            }
            ast::Expression::DotExpression(_tk, left, right) => {
                let full_path = self.transpile_expression(exp.to_owned());
                let module_name = get_js_module_name(&full_path);
                let path_to_use = full_path;
                (module_name, path_to_use)
            }
            _ => {
                panic!(
                    "Path must be of type (Identifier, AsExpression, StringLiteral, DotExpression)"
                );
            }
        }
    }

    fn transpile_use_stmt(
        &mut self,
        token: token::Token,
        prefix: Expression,
        path: Expression,
    ) -> String {
        let mut res = String::new();
        let mut import_type = ImportType::Base;

        // check prefix value.
        match prefix {
            ast::Expression::Identifier(_token, prefix) => {
                if prefix == "core" {
                    import_type = ImportType::Core;
                } else if prefix == "base" {
                    import_type = ImportType::Base;
                } else if prefix == "js" {
                    import_type = ImportType::JS;
                } else if prefix == "string" {
                    import_type = ImportType::String;
                }
            }
            _ => {
                panic!("Prefix must be of type Identifier");
            }
        }

        res.push_str("import ");
        let (module_name, path_to_use) = self.get_module_n_path(path);

        // match import type
        let path_to_use = import(path_to_use.as_str(), import_type, self);

        res.push_str(" * as ");
        res.push_str(&module_name);
        res.push_str(" from '");
        res.push_str(&path_to_use);
        res.push_str("';\n");

        res
    }

    fn transpile_use_from_stmt(
        &mut self,
        token: token::Token,
        specs: Vec<Expression>,
        prefix: Expression,
        path: Expression,
    ) -> String {
        let mut res = String::new();
        let mut import_type = ImportType::Base;

        match prefix {
            ast::Expression::Identifier(_token, prefix) => {
                if prefix == "core" {
                    import_type = ImportType::Core;
                } else if prefix == "base" {
                    import_type = ImportType::Base;
                // } else if prefix == "js" {
                //     import_type = ImportType::JS;
                } else if prefix == "string" {
                    import_type = ImportType::String;
                }
            }
            _ => {
                panic!("Prefix must be of type Identifier");
            }
        }

        res.push_str("import {");
        for spec in specs {
            match spec {
                ast::Expression::Identifier(_token, name) => {
                    res.push_str(&name);
                    res.push_str(", ");
                }
                _ => {
                    panic!("Spec must be of type Identifier");
                }
            }
        }
        res.push_str("}");

        let (_, path_to_use) = self.get_module_n_path(path);

        let path_to_use = import(path_to_use.as_str(), import_type, self);

        res.push_str(" from '");
        res.push_str(&path_to_use);
        res.push_str("';\n");

        res
    }

    fn transpile_javascript_stmt(&mut self, token: token::Token, js: String) -> String {
        format!("{}", js)
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
        let res = self.transpile_expression(expression);
        let semi = if res.trim().len() > 0 { ";\n" } else { "" };
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
                ast::Statement::ConstVariableStatement(_, name, _, value, _) => {
                    let name = self.transpile_expression(name.as_ref().to_owned());
                    let value = self.transpile_expression(value.as_ref().to_owned());

                    res.push_str(format!("{}.{} = {};\n", struct_name, name, value).as_str());
                }
                ast::Statement::VariableStatement(_, name, _,value, _) => {
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
                    Expression::FunctionLiteral(_, name, params,_, body) => {
                      result = (self.transpile_struct_method(&struct_name, cleaned_method_is_static.0, false, cleaned_method_is_static.1));
                    }
                    Expression::AsyncExpression(_, function) => {
                        result = (self.transpile_struct_method(&struct_name, cleaned_method_is_static.0, true, cleaned_method_is_static.1));
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

                let str_value = string_interpolation(&value);
                // supporting string $ interpolation.
                // 1. check if string contains $
                // 2. get the positions of all $
                // 3. if any of the positions is followed by a {, then ignore it because this should be interpreted by itself.
                // 4. for all other positions, get the start and end position of the identifier using lex::ALLOWED_IN_IDENT.contains(char)
                // 5. once we have the start and end position of the identifier, add ${} around the identifier

                // let quote_type = token.literal.chars().nth(0).unwrap();
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
            Expression::FunctionLiteral(token, name, paramters,_, body) => {
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
                if name_exp.chars().nth(0).unwrap().is_ascii_uppercase() {
                    res.push_str(&name_exp);
                } else {
                    res.push_str(name_exp.as_str());
                }

                res.push_str("(");

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
                // let joined_els = els
                //     .iter()
                //     .map(|p| self.transpile_expression(p.to_owned()))
                //     .collect::<Vec<_>>()
                //     .join(",");
                res.push_str(&self.join_expressions(elements.as_ref().to_owned()));
                res.push_str("]");

                res
            }
            Expression::IndexExpression(token, left, index) => {
                let mut res = String::new();

                res.push_str(&self.transpile_expression(left.as_ref().to_owned()));
                res.push_str("[");
                res.push_str(&self.transpile_expression(index.as_ref().to_owned()));
                res.push_str("]");

                res
            }
            Expression::ObjectLiteral(token, properties) => {
                let mut res = String::new();

                res.push_str("{");

                for i in 0..properties.len() {
                    let p = properties.get(i).unwrap();
                    let key = p.first().unwrap().as_ref().to_owned();
                    let value = p.last().unwrap().as_ref().to_owned();
                    res.push_str(&self.transpile_expression(key));
                    res.push_str(":");
                    res.push_str(&self.transpile_expression(value));
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
            Expression::RangeExpression(token, start, end) => {
                format!(
                    "builtins.{}({},{})",
                    builtins::INT_RANGE,
                    self.transpile_expression(start.as_ref().to_owned()),
                    self.transpile_expression(end.as_ref().to_owned())
                )
            }
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
            // Expression::MacroExpression(token, name, arguments) => {
            //     let name = self.transpile_expression(name.as_ref().to_owned());
            //     let mut parsed_args = vec![];

            //     for a in arguments.as_ref().to_owned() {
            //         parsed_args.push(self.transpile_expression(a));
            //     }

            //     let m = self.macros.get(&name);
            //     if m.is_some() {
            //         let m: &Macro = m.unwrap();
            //         let code = m.compile(parsed_args);
            //         if token.typ == token::MACRO_SYMBOL {
            //             // let result = interpret_js(&code, &mut self.context);
            //             if result.is_err() {
            //                 println!("Error: {}", result.err().unwrap());
            //                 println!("Macro in question: \n{}", code);
            //                 "".to_string()
            //             } else {
            //                 result.unwrap().display().to_string()
            //             }
            //         } else {
            //             code
            //         }
            //     } else {
            //         "".to_string()
            //     }
            // }
            // Expression::MacroDecleration(token, name, parameters, body) => {
            //     let name_as_string = self.transpile_expression(name.as_ref().to_owned());
            //     self.add_macro_function(
            //         name_as_string,
            //         parameters.as_ref().to_owned(),
            //         body.as_ref().to_owned(),
            //     );
            //     "".to_string()
            // }
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
            Expression::NullExpression(token, left, right) => {
                format!(
                    "{}?{}",
                    self.transpile_expression(left.as_ref().to_owned()),
                    self.transpile_expression(right.as_ref().to_owned())
                )
            }
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
            Expression::BuiltinCall(tk, params) => {
                // check builtin method name
                let method_name = tk.literal.clone();
                match method_name.as_str() {
                    "use_mod" => {
                        // get the first param
                        let param = &params[0];
                        let result = builtins::include(&self.transpile_expression(param.to_owned()));
                        
                        // let structs = &module_t.structs;
                        // update our transpilers structs context only.
                        // self.structs_in_modules.append(&mut structs);
                        result
                    }
                    _ => {
                        "".to_string()
                    }
                }
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

    fn add_macro_function(&mut self, name: String, params: Vec<Expression>, body: Statement) {
        let pms = self.join_expressions(params.to_owned());
        let mut parsed_args = vec![];

        for a in pms.split(",") {
            parsed_args.push(a.to_string());
        }

        let body = self.transpile_stmt(body).expect("No body error?");

        // add the body up to the last ';\n'
        self.macros
            .insert(name.to_owned(), Macro::new(name, parsed_args, body[0..body.len() - 2].to_string()));
    }

    /// Transpile a struct method
    fn transpile_struct_method(&mut self, struct_name: &str, method: ast::Expression, is_async: bool, is_static : bool) -> String {
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
                    res.push_str(" async ");
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
