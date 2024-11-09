use std::fmt::format;

use crate::lexer::token;
use crate::parser::ast;
use crate::parser::ast::Expression;

pub struct Transpiler {
    scripts: Vec<String>,
    variables: Vec<String>,
    functions: Vec<String>,
    imports: Vec<String>,
}

impl Transpiler {
    pub fn new() -> Self {
        Transpiler {
            scripts: vec![],
            variables: vec![],
            functions: vec![],
            imports: vec![],
        }
    }

    pub fn reset(&mut self) {
        self.scripts = vec![];
    }

    fn to_string(&self, pretty: bool) -> String {
        let mut res = String::new();

        for script in self.scripts.iter() {
            if pretty {
                let script = script.replace(";", ";\n");
                res.push_str(&format!("{}\n", script));
            } else {
                res.push_str(&script);
            }
        }

        res
    }
    pub fn transpile(&mut self, p: ast::Program, pretty: bool) -> String {
        let code = self.transpile_from(p, pretty);
        code
    }

    fn transpile_from(&mut self, p: ast::Program, pretty: bool) -> String {
        for stmt in p.statements {
            if stmt.is_empty() {
                continue;
            }

            let script = self.transpile_stmt(stmt);
            if let Some(script) = script {
                self.scripts.push(script);
            }
        }
        self.to_string(pretty)
    }

    fn transpile_stmt(&mut self, stmt: ast::Statement) -> Option<String> {
        match stmt {
            ast::Statement::VariableStatement(token, name, value) => Some(self.transpile_var_stmt(
                token,
                name.as_ref().to_owned(),
                value.as_ref().to_owned(),
            )),
            ast::Statement::ReturnStatement(token, expression) => {
                Some(self.transpile_return_stmt(token, expression.as_ref().to_owned()))
            }
            ast::Statement::ImportStatement(token, path, optinal_as) => {
                Some(self.transpile_import_stmt(token, path, optinal_as.as_ref().to_owned()))
            }
            ast::Statement::FromImportStatement(token, path, imports) => {
                Some(self.transpile_from_import_stmt(token, path, imports.as_ref().to_owned()))
            }
            ast::Statement::ExpressionStatement(token, expression) => {
                Some(self.transpile_expression_stmt(token, expression.as_ref().to_owned()))
            }
            ast::Statement::BlockStatement(token, stmts) => {
                Some(self.transpile_block_stmt(token, stmts.as_ref().to_owned()))
            }
            ast::Statement::ConstVariableStatement(token, name, value) => {
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
            ast::Statement::JavaScriptStatement(token, js) => None,
            _ => None,
        }
    }

    fn transpile_var_stmt(
        &mut self,
        token: token::Token,
        name: ast::Expression,
        value: ast::Expression,
    ) -> String {
        let mut response = String::new();
        match name {
            ast::Expression::Identifier(token, name) => {
                if !self.variables.contains(&name) {
                    self.variables.push(name.clone());
                    response.push_str("let ");
                }
                response.push_str(name.as_str());
                response.push_str("=");
                response.push_str(&self.transpile_expression(value));
                response.push_str(";");
            }
            _ => {
                panic!("Name must be of type Identifier");
            }
        }
        response
    }

    fn transpile_return_stmt(
        &mut self,
        token: token::Token,
        expression: ast::Expression,
    ) -> String {
        format!("return {};", self.transpile_expression(expression))
    }

    fn transpile_block_stmt(&mut self, token: token::Token, stmts: Vec<ast::Statement>) -> String {
        let mut response = String::new();
        for stmt in stmts {
            if let Some(stmt) = self.transpile_stmt(stmt) {
                response.push_str(&stmt);
            }
        }
        response
    }

    fn transpile_const_var_stmt(
        &mut self,
        token: token::Token,
        name: ast::Expression,
        value: ast::Expression,
    ) -> String {
        match name.clone() {
            ast::Expression::Identifier(_token, name) => {
                if !self.variables.contains(&name) {
                    self.variables.push(name);
                }
            }
            _ => {
                panic!("Name must be of type Identifier");
            }
        }
        format!(
            "const {} = {};",
            self.transpile_expression(name),
            self.transpile_expression(value)
        )
    }

    fn transpile_import_stmt(
        &mut self,
        token: token::Token,
        path: String,
        optional_as: Expression,
    ) -> String {
        let mut res = String::new();

        // TODO: check import file path type,
        // supported in EasyJS is ".ej", ".js", ".json", ".wasm"
        // no ".ts" <-- they're the competition

        match optional_as {
            Expression::AsExpression(token, exp) => {
                res.push_str("import ");
                res.push_str("{");
                res.push_str("default as ");
                res.push_str(&self.transpile_expression(exp.as_ref().to_owned()));
                res.push_str("} ");
                res.push_str("from \"");
                res.push_str(&path);
                res.push_str("\"");
            }
            _ => {
                res.push_str(format!("import \"{}\"", path).as_str());
            }
        }

        res.push_str(";");
        res
    }

    fn transpile_from_import_stmt(
        &mut self,
        token: token::Token,
        path: String,
        imports: Vec<Expression>,
    ) -> String {
        let mut res = String::new();
        res.push_str("import ");

        // TODO: check import file path type,
        // supported in EasyJS is ".ej", ".js", ".json", ".wasm"
        // no ".ts" <-- they're the competition

        let mut has_brace = false;
        for i in 0..imports.len() {
            let imp = &imports[i];
            match imp {
                Expression::DefExpression(token, exp) => {
                    if has_brace {
                        // can not have a brace here...
                        return "".to_string();
                    }
                    res.push_str(&self.transpile_expression(exp.as_ref().to_owned()));
                }
                _ => {
                    if !has_brace {
                        res.push_str("{");
                        has_brace = true;
                    }
                    res.push_str(&self.transpile_expression(imp.to_owned()));
                }
            }

            if i < imports.len() - 1 {
                res.push_str(", ");
            }
        }
        if has_brace {
            res.push_str("}");
        }

        res.push_str("from ");
        res.push_str(&format!("\"{}\"", path).as_str());
        res.push_str(";");

        res
        // "".to_string()
    }

    fn transpile_javascript_stmt(&mut self, token: token::Token, js: ast::Expression) -> String {
        format!("{};", self.transpile_expression(js))
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
                        "while({} {} {})",
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
                        "for (let {} of {})",
                        self.transpile_expression(left.as_ref().to_owned()),
                        self.transpile_expression(right.as_ref().to_owned())
                    )
                    .as_str(),
                );
            }
            Expression::InExpression(token, left, right) => {
                match right.as_ref().to_owned() {
                    Expression::RangeExpression(token, start, end) => {
                        let ident = self.transpile_expression(left.as_ref().to_owned());
                        res.push_str("for (let ");
                        res.push_str(&ident);
                        // get sn and en
                        let mut sn = 0;
                        let mut en = 0;

                        match start.as_ref().to_owned() {
                            Expression::IntegerLiteral(token, value) => sn = value,
                            _ => panic!("start must be of type number"),
                        }

                        match end.as_ref().to_owned() {
                            Expression::IntegerLiteral(token, value) => en = value,
                            _ => panic!("end must be of type number"),
                        }

                        res.push_str(" = ");
                        res.push_str(&sn.to_string());
                        res.push_str(";");
                        res.push_str(&ident);
                        res.push_str(" < ");
                        res.push_str(&en.to_string());
                        res.push_str(";");
                        res.push_str(&ident);
                        res.push_str("++");
                        res.push_str(")");
                    }
                    _ => res.push_str(
                        format!(
                            "for (let {} of {})",
                            self.transpile_expression(left.as_ref().to_owned()),
                            self.transpile_expression(right.as_ref().to_owned())
                        )
                        .as_str(),
                    ),
                }
            }
            _ => panic!("Condition must be boolean"),
        }

        res.push_str("{");

        let stmt = self.transpile_stmt(body);

        if let Some(stmt) = stmt {
            res.push_str(&stmt);
        }

        res.push_str("}");

        res
    }

    fn transpile_expression_stmt(
        &mut self,
        token: token::Token,
        expression: ast::Expression,
    ) -> String {
        format!("{};", self.transpile_expression(expression))
    }

    fn transpile_expression(&mut self, expression: ast::Expression) -> String {
        match expression {
            ast::Expression::IntegerLiteral(token, value) => value.to_string(),
            Expression::StringLiteral(token, value) => {
                let quote_type = if (&value.contains("'")).to_owned() {
                    "\""
                } else {
                    "\'"
                };
                // let quote_type = token.literal.chars().nth(0).unwrap();
                format!("{}{}{}", quote_type, value, quote_type)
            }
            Expression::PrefixExpression(token, op, value) => {
                format!(
                    "({}{})",
                    op,
                    self.transpile_expression(value.as_ref().to_owned())
                )
            }
            Expression::InfixExpression(token, left, operator, right) => {
                format!(
                    "({} {} {})",
                    self.transpile_expression(left.as_ref().to_owned()),
                    operator,
                    self.transpile_expression(right.as_ref().to_owned())
                )
            }
            Expression::IfExpression(token, condition, consequence, elseif, else_) => {
                let mut res = String::new();

                res.push_str("if (");
                res.push_str(&self.transpile_expression(condition.as_ref().to_owned()));
                res.push_str(") {");
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
                    res.push_str("else { ");
                    let stmt = self.transpile_stmt(else_.as_ref().to_owned());
                    if let Some(stmt) = stmt {
                        res.push_str(&stmt);
                    }
                    res.push_str("}");
                }

                res
            }
            Expression::FunctionLiteral(token, name, paramters, body) => {
                let mut res = String::new();

                res.push_str("function ");
                match name.as_ref().to_owned() {
                    Expression::Identifier(token, value) => {
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

                res.push_str("{");
                let stmt = self.transpile_stmt(body.as_ref().to_owned());
                if let Some(stmt) = stmt {
                    res.push_str(&stmt);
                }
                res.push_str("}");

                res
            }
            Expression::CallExpression(token, name, arguments) => {
                let mut res = String::new();

                res.push_str(&self.transpile_expression(name.as_ref().to_owned()));
                res.push_str("(");

                let args = arguments.as_ref().to_owned();
                let joined_args = args
                    .iter()
                    .map(|p| self.transpile_expression(p.to_owned()))
                    .collect::<Vec<_>>()
                    .join(",");
                res.push_str(&joined_args);
                res.push_str(")");

                res
            }
            Expression::Boolean(token, value) => {
                format!("{}", value)
            }
            Expression::Identifier(token, name) => name,
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
            Expression::JavaScriptExpression(token, js) => js
                .strip_prefix("{")
                .unwrap()
                .strip_suffix("}")
                .unwrap()
                .to_string(),
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
                res.push_str(") => {");
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
                let joined_els = els
                    .iter()
                    .map(|p| self.transpile_expression(p.to_owned()))
                    .collect::<Vec<_>>()
                    .join(",");
                res.push_str(&joined_els);
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
                        res.push_str(",");
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
                let mut res = String::new();

                res.push_str("[");
                let mut sn = 0;
                let mut en = 0;
                match start.as_ref().to_owned() {
                    Expression::IntegerLiteral(token, value) => sn = value,
                    _ => panic!("start must be of type number"),
                }

                match end.as_ref().to_owned() {
                    Expression::IntegerLiteral(token, value) => en = value,
                    _ => panic!("end must be of type number"),
                }

                let mut numbers = vec![];
                for i in sn..en {
                    numbers.push(i.to_string());
                }

                let joined = numbers.join(",");

                res.push_str(&joined);
                res.push_str("]");
                res
            }
            Expression::AwaitExpression(token, exp) => {
                format!(
                    "await {}",
                    self.transpile_expression(exp.as_ref().to_owned())
                )
            }
            Expression::AssignExpression(token, left, right) => {
                format!(
                    "{} = {}",
                    self.transpile_expression(left.as_ref().to_owned()),
                    self.transpile_expression(right.as_ref().to_owned())
                )
            }
            Expression::NotExpression(token, exp) => {
                format!("!{}", self.transpile_expression(exp.as_ref().to_owned()))
            }
            Expression::AsExpression(token, exp) => {
                format!(" as {}", self.transpile_expression(exp.as_ref().to_owned()))
            }
            _ => String::from(""),
        }
    }
}
