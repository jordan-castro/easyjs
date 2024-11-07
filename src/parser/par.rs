use crate::lexer::{lex, token};
use crate::parser::ast;

/// Our AST parser.
pub struct Parser {
    /// Access to the lexer.
    l: lex::Lex, // <-- Lex
    /// The current token we are parsing
    c_token: token::Token,
    /// The next token we are parsing
    peek_token: token::Token,
    /// parsing errors
    pub errors: Vec<String>,
    // prefix_fns: HashMap<String, PrefixParseFn>,
    // infix_fns: HashMap<String, InfixParseFn>,
}

// Constant values
const LOWEST: i64 = 1;
const EQUALS: i64 = 2; // ==
const LESSGREATER: i64 = 3; // < or >
const SUM: i64 = 4; // +
const PRODUCT: i64 = 5; // *
// const PREFIX: i64 = 6; // -X or !X
const CALL: i64 = 7; // my_function(X)
const DOT: i64 = 8; // .field or .method
const JAVASCRIPT: i64 = 9; // javascript code
const LESSGREATER_OR_EQUALS: i64 = 10; // <= or >=
const BRACKET: i64 = 11; // [
const BRACE: i64 = 12; // {
const DOTDOT: i64 = 13; // ..
const IN: i64 = 14; // in
const OF: i64 = 15; // of
const AWAIT: i64 = 16; // await
const ASSIGN: i64 = 17;

/// Find the precedence of a token.
fn precedences(tk: &str) -> i64 {
    match tk {
        token::EQ => EQUALS,
        token::NOT_EQ => EQUALS,
        token::LT => LESSGREATER,
        token::GT => LESSGREATER,
        token::PLUS => SUM,
        token::MINUS => SUM,
        token::SLASH => PRODUCT,
        token::ASTERISK => PRODUCT,
        token::L_PAREN => CALL,
        token::DOT => DOT,
        token::JAVASCRIPT => JAVASCRIPT,
        token::LT_OR_EQ => LESSGREATER_OR_EQUALS,
        token::GT_OR_EQ => LESSGREATER_OR_EQUALS,
        token::L_BRACKET => BRACKET,
        token::L_BRACE => BRACE,
        token::DOTDOT => DOTDOT,
        token::IN => IN,
        token::OF => OF,
        token::AWAIT => AWAIT,
        token::ASSIGN => ASSIGN,
        _ => LOWEST,
    }
}

impl Parser {
    pub fn new(l: lex::Lex) -> Self {
        let mut p = Parser {
            l,
            c_token: token::new_token("", ""),
            peek_token: token::new_token("", ""),
            errors: vec![],
        };
        p.next_token();
        p.next_token();

        p
    }

    /// peek the precedence
    fn peek_precedence(&self) -> i64 {
        precedences(&self.peek_token.typ)
    }

    /// the current precedence
    fn cur_precedence(&self) -> i64 {
        precedences(&self.c_token.typ)
    }

    /// This is how we do it, run this function to call a prefix method.
    fn prefix_fns(&mut self, token_type: &str) -> ast::Expression {
        match token_type {
            token::IDENT => parse_identifier(self),
            token::INT => parse_integer_literal(self),
            token::BANG => parse_prefix_expression(self),
            token::MINUS => parse_prefix_expression(self),
            token::TRUE => parse_boolean(self),
            token::FALSE => parse_boolean(self),
            token::L_PAREN => parse_group_expression(self),
            token::IF => parse_if_expression(self),
            token::FUNCTION => parse_function_literal(self),
            token::STRING => parse_string_literal(self),
            token::COMMENT => parse_comment(self),
            token::JAVASCRIPT => parse_javascript_expression(self),
            token::L_BRACKET => parse_array_literal(self),
            token::L_BRACE => parse_object_literal(self),
            token::ASYNC => parse_async_expressoin(self),
            token::AWAIT => parse_await_expression(self),

            _ => ast::Expression::EmptyExpression,
        }
    }

    /// check if has prefix
    fn has_prefix(&self, token_type: &str) -> bool {
        match token_type {
            token::IDENT => true,
            token::INT => true,
            token::BANG => true,
            token::MINUS => true,
            token::TRUE => true,
            token::FALSE => true,
            token::L_PAREN => true,
            token::IF => true,
            token::FUNCTION => true,
            token::STRING => true,
            token::COMMENT => true,
            token::JAVASCRIPT => true,
            token::L_BRACKET => true,
            token::L_BRACE => true,
            token::ASYNC => true,
            token::AWAIT => true,
            _ => false,
        }
    }

    /// Check if this token has an infix
    fn has_infix(&self, token_type: &str) -> bool {
        match token_type {
            token::PLUS => true,
            token::MINUS => true,
            token::SLASH => true,
            token::ASTERISK => true,
            token::EQ => true,
            token::NOT_EQ => true,
            token::LT => true,
            token::GT => true,
            token::LT_OR_EQ => true,
            token::GT_OR_EQ => true,
            token::L_PAREN => true,
            token::DOT => true,
            token::JAVASCRIPT => true,
            token::L_BRACKET => true,
            token::DOTDOT => true,
            token::IN => true,
            token::OF => true,
            token::ASSIGN => true,
            _ => false,
        }
    }

    /// This is how we do it, run this function to call a infix method.
    fn infix_fns(&mut self, token_type: &str, left: ast::Expression) -> ast::Expression {
        match token_type {
            token::PLUS => parse_infix_expression(self, left),
            token::MINUS => parse_infix_expression(self, left),
            token::SLASH => parse_infix_expression(self, left),
            token::ASTERISK => parse_infix_expression(self, left),
            token::EQ => parse_infix_expression(self, left),
            token::NOT_EQ => parse_infix_expression(self, left),
            token::LT => parse_infix_expression(self, left),
            token::GT => parse_infix_expression(self, left),
            token::LT_OR_EQ => parse_infix_expression(self, left),
            token::GT_OR_EQ => parse_infix_expression(self, left),
            token::L_PAREN => parse_call_expression(self, left),
            token::DOT => parse_dot_expression(self, left),
            token::JAVASCRIPT => parse_infix_expression(self, left),
            token::L_BRACKET => parse_index_expression(self, left),
            token::DOTDOT => parse_range_expression(self, left),
            token::IN => parse_in_expression(self, left),
            token::OF => parse_of_expression(self, left),
            token::ASSIGN => parse_assign_expression(self, left),
            _ => ast::Expression::EmptyExpression,
        }
    }

    /// Is the current token this type?
    fn cur_token_is(&self, token_type: &str) -> bool {
        &self.c_token.typ == token_type
    }

    /// is the peek token this type?
    fn peek_token_is(&self, token_type: &str) -> bool {
        &self.peek_token.typ == token_type
    }

    /// Expect the peek token to be of type, writes error if failed.
    fn expect_peek(&mut self, token_type: &str) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            return true;
        }

        self.errors.push(format!(
            "Expected next token to be {} but got {} instead.",
            token_type, self.peek_token.typ
        ));

        false
    }

    /// Move forward in the token hierachy
    fn next_token(&mut self) {
        self.c_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    /// is our current token and eos.
    fn _cur_token_is_eos(&self) -> bool {
        self.cur_token_is(token::SEMICOLON) || self.cur_token_is(token::EOL)
    }

    /// is the peek token an eos.
    fn peek_token_is_eos(&self) -> bool {
        self.peek_token_is(token::SEMICOLON) || self.peek_token_is(token::EOL)
    }

    /// Parse a program
    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program { statements: vec![] };

        // parse until EOF token
        while !self.cur_token_is(token::EOF) {
            let stmt = parse_statement(self);
            if !stmt.is_empty() {
                // we got one!
                program.statements.push(stmt);
            }

            // go to the next token
            self.next_token();
        }

        program
    }
}

/// Parse a statement, returns EmptyStatement on error.
fn parse_statement(parser: &mut Parser) -> ast::Statement {
    match parser.c_token.typ.as_str() {
        token::IDENT => {
            if parser.peek_token_is(token::ASSIGN) {
                parse_var_statement(parser)
            } else if parser.peek_token_is(token::CONST_ASSIGNMENT) {
                parse_const_var_statement(parser)
            } else {
                parse_expression_statement(parser)
            }
        }
        token::RETURN => parse_return_statement(parser),
        token::IMPORT => parse_import_statement(parser),
        token::JAVASCRIPT => ast::Statement::JavaScriptStatement(
            parser.c_token.to_owned(),
            parser.c_token.to_owned().literal,
        ),
        token::FOR => parse_for_statement(parser),
        _ => parse_expression_statement(parser),
    }
}

fn parse_var_statement(p: &mut Parser) -> ast::Statement {
    let token = p.c_token.clone();

    if !p.expect_peek(token::ASSIGN) {
        return ast::Statement::EmptyStatement;
    }
    let name = ast::Expression::Identifier(token.to_owned(), token.to_owned().literal);
    p.next_token();

    let value = parse_expression(p, LOWEST);

    if p.peek_token_is_eos() {
        p.next_token();
    }

    ast::Statement::VariableStatement(token, Box::new(name), Box::new(value))
}

fn parse_const_var_statement(p: &mut Parser) -> ast::Statement {
    let token = p.c_token.clone();

    if !p.expect_peek(token::CONST_ASSIGNMENT) {
        return ast::Statement::EmptyStatement;
    }

    let name = ast::Expression::Identifier(token.to_owned(), token.to_owned().literal);
    
    p.next_token();

    let value = parse_expression(p, LOWEST);

    if value.is_empty() {
        return ast::Statement::EmptyStatement;
    }

    if p.peek_token_is_eos() {
        p.next_token();
    }

    ast::Statement::ConstVariableStatement(token, Box::new(name), Box::new(value))
}

fn parse_return_statement(p: &mut Parser) -> ast::Statement {
    let token = p.c_token.clone();

    p.next_token();

    let value = parse_expression(p, LOWEST);

    // consume the ;
    if p.peek_token_is_eos() {
        p.next_token();
    }

    ast::Statement::ReturnStatement(token, Box::new(value))
}

fn parse_expression_statement(p: &mut Parser) -> ast::Statement {
    let token = p.c_token.clone();
    let expression = parse_expression(p, LOWEST);

    if expression.is_empty() {
        return ast::Statement::EmptyStatement;
    }

    // we hit the end of the line
    if p.peek_token_is_eos() {
        p.next_token();
    }

    ast::Statement::ExpressionStatement(token, Box::new(expression))
}

fn parse_import_statement(p: &mut Parser) -> ast::Statement {
    let token = p.c_token.clone();

    if !p.expect_peek(token::STRING) {
        return ast::Statement::EmptyStatement;
    }

    let path = p.c_token.literal.to_owned();
    let mut as_ = String::new();

    // check for an as
    if p.peek_token_is(token::AS) {
        p.next_token();

        if !p.expect_peek(token::IDENT) {
            return ast::Statement::EmptyStatement;
        }
        as_ = p.c_token.literal.to_owned();
    }

    let as_option = if as_.len() > 0 { Some(as_) } else { None };

    ast::Statement::ImportStatement(token, path, as_option)
}

fn parse_block_statement(p: &mut Parser) -> ast::Statement {
    let token = p.c_token.clone();
    let mut staments: Vec<ast::Statement> = vec![];

    p.next_token(); // consume the {

    while !p.cur_token_is(token::R_BRACE) && !p.cur_token_is(token::EOF) {
        let stmt = parse_statement(p);
        if !stmt.is_empty() {
            staments.push(stmt);
        }
        p.next_token();
    }

    ast::Statement::BlockStatement(token.to_owned(), Box::new(staments))
}

fn parse_for_statement(p: &mut Parser) -> ast::Statement {
    let token = p.c_token.to_owned(); // for
    let mut has_paren: bool = false;

    if p.peek_token_is(token::L_PAREN) {
        // we got paran
        // consume it
        p.next_token(); // (
        has_paren = true;
    }

    // go to expression
    p.next_token();
    let condition = parse_expression(p, LOWEST);
    if condition.is_empty() {
        return ast::empty_statement();
    }

    if has_paren && !p.expect_peek(token::R_PAREN) {
        return ast::empty_statement();
    }

    if !p.expect_peek(token::L_BRACE) {
        return ast::empty_statement();
    }

    let body = parse_block_statement(p);

    ast::Statement::ForStatement(token.to_owned(), Box::new(condition), Box::new(body))
}

fn parse_expression(p: &mut Parser, precedence: i64) -> ast::Expression {
    let token_type = p.c_token.typ.clone();
    let prefix = p.has_prefix(&token_type);
    if !prefix {
        p.errors
            .push(format!("No prefix function for {} found.", token_type));
        return ast::Expression::EmptyExpression;
    }
    let mut left_exp = p.prefix_fns(&token_type);

    while !(p.peek_token_is_eos() || p.peek_token_is(token::EOF))
        && precedence < p.peek_precedence()
    {
        let peek_type = p.peek_token.typ.clone();
        let infix = p.has_infix(&peek_type);
        if !infix {
            return left_exp;
        }
        p.next_token();
        left_exp = p.infix_fns(&peek_type, left_exp)
    }

    left_exp
}

fn parse_prefix_expression(p: &mut Parser) -> ast::Expression {
    let token = p.c_token.clone();
    let operator = p.c_token.literal.to_owned();

    p.next_token();

    let right = parse_expression(p, LOWEST);

    ast::Expression::PrefixExpression(token, operator, Box::new(right))
}

/// Parse an identifier
fn parse_identifier(parser: &mut Parser) -> ast::Expression {
    ast::Expression::Identifier(parser.c_token.clone(), parser.c_token.literal.clone())
}

/// parse an integer literal, returns EmptyExpression if not valid.
fn parse_integer_literal(parser: &mut Parser) -> ast::Expression {
    let tk = parser.c_token.clone();
    // check is number
    let is_number = parser.c_token.literal.parse::<i64>().is_ok();
    if !is_number {
        parser
            .errors
            .push(format!("Epected type INT got {} instead", tk.literal));
        return ast::Expression::EmptyExpression;
    }
    let integer = parser.c_token.literal.parse::<i64>().unwrap();

    ast::Expression::IntegerLiteral(tk, integer)
}

/// parse a boolean
fn parse_boolean(p: &mut Parser) -> ast::Expression {
    ast::Expression::Boolean(p.c_token.clone(), p.cur_token_is(token::TRUE))
}

fn parse_group_expression(p: &mut Parser) -> ast::Expression {
    p.next_token();
    let exp = parse_expression(p, LOWEST);
    if !p.expect_peek(token::R_PAREN) {
        return ast::Expression::EmptyExpression;
    }

    exp
}

fn parse_if_expression(p: &mut Parser) -> ast::Expression {
    let token = p.c_token.clone();
    let mut elseif = ast::Expression::EmptyExpression;
    let mut else_ = ast::Statement::EmptyStatement;

    let mut has_par = false;

    // check for a (
    if p.peek_token_is(token::L_PAREN) {
        // consume it
        has_par = true;
        p.next_token(); // (
    }

    // update tokens
    p.next_token(); // get to first param

    let condition = parse_expression(p, LOWEST);
    if condition.is_empty() {
        return ast::Expression::EmptyExpression;
    }

    // consume ) if any
    if has_par && !p.expect_peek(token::R_PAREN) {
        return ast::Expression::EmptyExpression;
    }

    if !p.expect_peek(token::L_BRACE) {
        return ast::Expression::EmptyExpression;
    }

    // parse block
    let consequence = parse_block_statement(p);

    // check for elseif or else
    if p.peek_token_is(token::ELSE) {
        p.next_token(); // consume else
        if !p.expect_peek(token::L_BRACE) {
            return ast::Expression::EmptyExpression;
        }

        // we got em
        else_ = parse_block_statement(p);
    } else if p.peek_token_is(token::ELIF) {
        p.next_token(); // consume it
        elseif = parse_if_expression(p);
    }

    ast::Expression::IfExpression(
        token,
        Box::new(condition),
        Box::new(consequence),
        Box::new(elseif),
        Box::new(else_),
    )
}

fn parse_function_literal(p: &mut Parser) -> ast::Expression {
    let token = p.c_token.clone();

    if p.peek_token_is(token::L_PAREN) {
        // this is a lambda
        return parse_lambda_literal(p);
    }

    // ok lets make sure this is a function
    if !p.expect_peek(token::IDENT) {
        // not a function
        return ast::Expression::EmptyExpression;
    }
    let name = parse_identifier(p);

    if !p.expect_peek(token::L_PAREN) {
        return ast::Expression::EmptyExpression;
    }

    // params
    let parameters = parse_function_paramaters(p);

    if !p.expect_peek(token::L_BRACE) {
        return ast::Expression::EmptyExpression;
    }

    let body = parse_block_statement(p);
    if body.is_empty() {
        return ast::Expression::EmptyExpression;
    }

    ast::Expression::FunctionLiteral(
        token.to_owned(),
        Box::new(name),
        Box::new(parameters),
        Box::new(body),
    )
}

fn parse_function_paramaters(p: &mut Parser) -> Vec<ast::Expression> {
    // starts at (
    let mut idents = vec![];

    // if we got no params
    if p.peek_token_is(token::R_PAREN) {
        p.next_token(); // consume the )
        return idents;
    }

    // go to first identifier
    p.next_token();
    idents.push(parse_identifier(p));

    while p.peek_token_is(token::COMMA) {
        p.next_token();
        p.next_token();
        idents.push(parse_identifier(p));
    }

    if !p.expect_peek(token::R_PAREN) {
        return vec![ast::Expression::EmptyExpression];
    }

    idents
}

fn parse_lambda_literal(p: &mut Parser) -> ast::Expression {
    let token = p.c_token.clone();

    if !p.expect_peek(token::L_PAREN) {
        // not a lambda
        return ast::Expression::EmptyExpression;
    }

    // params
    let paramaters = parse_function_paramaters(p);

    if !p.expect_peek(token::L_BRACE) {
        return ast::Expression::EmptyExpression;
    }

    let body = parse_block_statement(p);

    if body.is_empty() {
        return ast::Expression::EmptyExpression;
    }

    ast::Expression::LambdaLiteral(token.to_owned(), Box::new(paramaters), Box::new(body))
}

fn parse_string_literal(p: &mut Parser) -> ast::Expression {
    ast::Expression::StringLiteral(p.c_token.clone().to_owned(), p.c_token.to_owned().literal)
}

fn parse_comment(p: &mut Parser) -> ast::Expression {
    ast::Expression::CommentExpression(p.c_token.to_owned(), p.c_token.to_owned().literal)
}

fn parse_javascript_expression(p: &mut Parser) -> ast::Expression {
    ast::Expression::JavaScriptExpression(p.c_token.to_owned(), p.c_token.to_owned().literal)
}

fn parse_array_literal(p: &mut Parser) -> ast::Expression {
    let token = p.c_token.to_owned();
    let elements = parse_array_arguments(p);
    ast::Expression::ArrayLiteral(token, Box::new(elements))
}

fn parse_array_arguments(p: &mut Parser) -> Vec<ast::Expression> {
    let mut args = vec![];

    if p.peek_token_is(token::R_BRACKET) {
        // consume
        p.next_token();
        return args;
    }

    p.next_token();
    args.push(parse_expression(p, LOWEST));

    while p.peek_token_is(token::COMMA) {
        p.next_token();
        p.next_token();

        let el = parse_expression(p, LOWEST);
        if el.is_empty() {
            continue;
        }

        args.push(el);
    }

    if !p.expect_peek(token::R_BRACKET) {
        return vec![];
    }

    args
}

fn parse_object_literal(p: &mut Parser) -> ast::Expression {
    let token = p.c_token.to_owned();
    let mut elements = vec![];

    if p.peek_token_is(token::R_BRACE) {
        // consume it
        p.next_token();

        return ast::Expression::ObjectLiteral(token.to_owned(), elements);
    }

    let mut brace_count = 1;
    while !p.peek_token_is(token::EOF) {
        p.next_token();

        if p.cur_token_is(token::L_BRACE) {
            brace_count += 1;
        } else if p.cur_token_is(token::R_BRACE) {
            brace_count -= 1;
            if brace_count == 0 {
                break;
            }
        }

        let key = parse_expression(p, LOWEST);
        if !p.expect_peek(token::COLON) {
            return ast::Expression::EmptyExpression;
        }
        p.next_token();
        let value = parse_expression(p, LOWEST);

        if key.is_empty() || value.is_empty() {
            return ast::Expression::EmptyExpression;
        }
        elements.push(vec![Box::new(key), Box::new(value)]);

        // check for comma.
        if p.peek_token_is(token::COMMA) {
            p.next_token();
        }
    }

    if !p.cur_token_is(token::R_BRACE) {
        // what what what??
        return ast::Expression::EmptyExpression;
    }

    ast::Expression::ObjectLiteral(token, elements)
}

fn parse_async_expressoin(p: &mut Parser) -> ast::Expression {
    let token = p.c_token.to_owned();
    if !p.expect_peek(token::FUNCTION) {
        return ast::Expression::EmptyExpression;
    }

    ast::Expression::AsyncExpression(token.to_owned(), Box::new(parse_function_literal(p)))
}

fn parse_await_expression(p: &mut Parser) -> ast::Expression {
    let token = p.c_token.to_owned();
    p.next_token(); // consome keyword
    let value = parse_expression(p, LOWEST);

    if value.is_empty() {
        return value.to_owned();
    }

    ast::Expression::AwaitExpression(token, Box::new(value))
}

fn parse_infix_expression(p: &mut Parser, left: ast::Expression) -> ast::Expression {
    let token = p.c_token.to_owned();
    let operator = p.c_token.to_owned().literal;

    let precedence = p.cur_precedence();
    p.next_token();
    let right = parse_expression(p, precedence);

    ast::Expression::InfixExpression(token, Box::new(left), operator, Box::new(right))
}

fn parse_call_expression(p: &mut Parser, left: ast::Expression) -> ast::Expression {
    let token = p.c_token.to_owned();
    let arguments = parse_call_arguments(p);

    ast::Expression::CallExpression(token, Box::new(left), Box::new(arguments))
}

fn parse_call_arguments(p: &mut Parser) -> Vec<ast::Expression> {
    let mut args = vec![];

    if p.peek_token_is(token::R_PAREN) {
        p.next_token();
        return args;
    }

    p.next_token();
    args.push(parse_expression(p, LOWEST));

    while p.peek_token_is(token::COMMA) {
        p.next_token();
        p.next_token();

        args.push(parse_expression(p, LOWEST));
    }

    if !p.expect_peek(token::R_PAREN) {
        return vec![];
    }

    args
}

fn parse_dot_expression(p: &mut Parser, left: ast::Expression) -> ast::Expression {
    let token = p.c_token.to_owned();

    if p.peek_token_is(token::IF) {
        p.next_token();
        return parse_dot_if_expression(p, left);
    }

    if !p.expect_peek(token::IDENT) {
        return ast::Expression::EmptyExpression;
    }

    let right = parse_expression(p, LOWEST);
    if right.is_empty() {
        return ast::Expression::EmptyExpression;
    }

    ast::Expression::DotExpression(token, Box::new(left), Box::new(right))
}

fn parse_dot_if_expression(_p: &mut Parser, _left: ast::Expression) -> ast::Expression {
    ast::Expression::EmptyExpression
}

fn parse_index_expression(p: &mut Parser, left: ast::Expression) -> ast::Expression {
    let token = p.c_token.to_owned();

    if p.peek_token_is(token::R_BRACKET) {
        return ast::Expression::EmptyExpression;
    }

    let index = parse_expression(p, LOWEST);

    if index.is_empty() {
        return ast::Expression::EmptyExpression;
    }

    if p.peek_token_is(token::ASSIGN) {
        p.next_token();
        p.next_token();

        let right = parse_expression(p, LOWEST);

        if !right.eq(ast::Expression::EmptyExpression) {
            return ast::Expression::IndexExpression(
                token.to_owned(),
                Box::new(left),
                Box::new(index)
            );
        }
    }

    ast::Expression::IndexExpression(
        token,
        Box::new(left),
        Box::new(index)
    )
}

fn parse_range_expression(p: &mut Parser, left: ast::Expression) -> ast::Expression {
    let token = p.c_token.to_owned();

    if !left.eq(ast::Expression::IntegerLiteral(token::EMPTY_TOKEN, 0)) {
        p.errors
            .push("Range must start with an integer".to_string());
        return ast::Expression::EmptyExpression;
    }

    if !p.expect_peek(token::INT) {
        p.errors.push("Range must end with a integer".to_string());
        return ast::Expression::EmptyExpression;
    }

    let right = parse_expression(p, LOWEST);
    if right.is_empty() {
        return ast::Expression::EmptyExpression;
    }

    ast::Expression::RangeExpression(token, Box::new(left), Box::new(right))
}

fn parse_in_expression(p: &mut Parser, left: ast::Expression) -> ast::Expression {
    let token = p.c_token.to_owned();
    p.next_token();
    let right = parse_expression(p, LOWEST);
    if right.is_empty() {
        return ast::Expression::EmptyExpression;
    }

    ast::Expression::InExpression(token, Box::new(left), Box::new(right))
}

fn parse_of_expression(p: &mut Parser, left: ast::Expression) -> ast::Expression {
    let token = p.c_token.to_owned();
    p.next_token();
    let right = parse_expression(p, LOWEST);
    if right.is_empty() {
        return ast::Expression::EmptyExpression;
    }

    ast::Expression::OfExpression(token, Box::new(left), Box::new(right))
}

fn parse_assign_expression(p: &mut Parser, left: ast::Expression) -> ast::Expression {
    let token = p.c_token.clone(); // =
    p.next_token();
    let right = parse_expression(p, LOWEST);

    ast::Expression::AssignExpression(token, Box::new(left), Box::new(right))
}