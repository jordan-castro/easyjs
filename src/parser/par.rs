use crate::lexer::{lex, token};
use crate::parser::ast;

use super::ast::empty_expression;

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

    /// is debug mode
    is_debug_mode: bool,
}

// Constant values
const LOWEST: i64 = 1;

// math
const EQUALS: i64 = 2; // == !=

const LESSGREATER: i64 = 3; // < > >= <=
const SUM: i64 = 4; // + -
const PRODUCT: i64 = 5; // * /

const DOT: i64 = 6; // .field or .method

const CALL: i64 = 7; // my_function(X)
const BRACKET: i64 = 10; // [
const BRACE: i64 = 11; // {
const DOTDOT: i64 = 12; // ..
const IN: i64 = 13; // in
const OF: i64 = 14; // of
const AWAIT: i64 = 15; // await

const ASSIGN: i64 = 16;

const AS: i64 = 18;
// const MACRO_SYMBOL: i64 = 19;
// const DECORATOR: i64 = 20;
// const MACRO: i64 = 21;
const AND: i64 = 22;
const OR: i64 = 23;
const QUESTION_MARK: i64 = 24;
const DOUBLE_QUESTION_MARK: i64 = 25;
const NEW: i64 = 27;

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
        token::LT_OR_EQ => LESSGREATER,
        token::GT_OR_EQ => LESSGREATER,
        token::L_BRACKET => BRACKET,
        token::L_BRACE => BRACE,
        token::DOTDOT => DOTDOT,
        token::IN => IN,
        token::OF => OF,
        token::IS => IN,
        token::AWAIT => AWAIT,
        token::ASSIGN => ASSIGN,
        token::AS => AS,
        // token::MACRO_SYMBOL => MACRO_SYMBOL,
        // token::DECORATOR => DECORATOR,
        // token::MACRO => MACRO,
        token::AND_SYMBOL => AND,
        token::OR_SYMBOL => OR,
        token::QUESTION_MARK => QUESTION_MARK,
        token::DOUBLE_QUESTION_MARK => DOUBLE_QUESTION_MARK,
        token::MODULUS => PRODUCT,
        token::NEW => NEW,
        token::PLUS_EQUALS => ASSIGN,
        token::MINUS_EQUALS => ASSIGN,
        token::SLASH_EQUALS => ASSIGN,
        token::ASTERISK_EQUALS => ASSIGN,
        _ => LOWEST,
    }
}

impl Parser {
    pub fn new(l: lex::Lex) -> Self {
        let is_debug_mode_var = std::env::var("EASYJS_DEBUG");
        let is_debug_mode = if is_debug_mode_var.is_err() {
            false
        } else {
            is_debug_mode_var.unwrap() == "1"
        };

        let mut p = Parser {
            l,
            c_token: token::new_token("", ""),
            peek_token: token::new_token("", ""),
            errors: vec![],
            is_debug_mode,
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
            token::IDENT => parse_identifier(self, false),
            token::SELF => parse_identifier(self, false),
            token::INT => parse_integer_literal(self),
            token::FLOAT => parse_float_literal(self),
            token::BANG => parse_prefix_expression(self),
            token::NOT => parse_not_expression(self),
            token::MINUS => parse_prefix_expression(self),
            token::TRUE => parse_boolean(self),
            token::FALSE => parse_boolean(self),
            token::L_PAREN => parse_group_expression(self),
            token::IF => parse_if_expression(self),
            token::FUNCTION => parse_function_literal(self),
            token::STRING => parse_string_literal(self),
            token::COMMENT => parse_comment(self),
            token::L_BRACKET => parse_array_literal(self),
            token::L_BRACE => parse_object_literal(self),
            token::ASYNC => parse_async_expressoin(self),
            token::AWAIT => parse_await_expression(self),
            // token::MACRO_SYMBOL => parse_macro_expression(self),
            // token::DECORATOR => parse_macro_expression(self),
            // token::MACRO => parse_macro_decleration(self),
            token::NEW => parse_new_expression(self),
            token::BUILTIN => parse_builtin_expression(self),
            _ => ast::Expression::EmptyExpression,
        }
    }

    /// check if has prefix
    fn has_prefix(&self, token_type: &str) -> bool {
        match token_type {
            token::IDENT => true,
            token::SELF => true,
            token::INT => true,
            token::BANG => true,
            token::FLOAT => true,
            token::NOT => true,
            token::MINUS => true,
            token::TRUE => true,
            token::FALSE => true,
            token::L_PAREN => true,
            token::IF => true,
            token::FUNCTION => true,
            token::STRING => true,
            token::COMMENT => true,
            token::L_BRACKET => true,
            token::L_BRACE => true,
            token::ASYNC => true,
            token::AWAIT => true,
            // token::MACRO_SYMBOL => true,
            // token::DECORATOR => true,
            // token::MACRO => true,
            token::NEW => true,
            token::BUILTIN => true,
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
            token::AS => true,
            token::AND_SYMBOL => true,
            token::OR_SYMBOL => true,
            token::QUESTION_MARK => true,
            token::DOUBLE_QUESTION_MARK => true,
            token::MODULUS => true,
            token::PLUS_EQUALS => true,
            token::MINUS_EQUALS => true,
            token::SLASH_EQUALS => true,
            token::ASTERISK_EQUALS => true,
            token::IS => true,
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
            token::AND_SYMBOL => parse_and_expression(self, left),
            token::OR_SYMBOL => parse_or_expression(self, left),
            token::QUESTION_MARK => parse_question_mark_expression(self, left),
            token::DOUBLE_QUESTION_MARK => parse_double_question_mark_expression(self, left),
            token::MODULUS => parse_infix_expression(self, left),
            token::PLUS_EQUALS => parse_infix_expression(self, left),
            token::MINUS_EQUALS => parse_infix_expression(self, left),
            token::SLASH_EQUALS => parse_infix_expression(self, left),
            token::ASTERISK_EQUALS => parse_infix_expression(self, left),
            token::AS => parse_as_expression(self, left),
            token::IS => parse_is_expression(self, left),
            _ => ast::Expression::EmptyExpression,
        }
    }

    /// Print if is debug mode
    fn debug_print(&self, msg: &str) {
        if self.is_debug_mode {
            println!("{}", msg);
        }
    }

    /// Add an error
    fn add_error(&mut self, error: &str) {
        self.errors.push(format!(
            "File: {} Line {}.{}: {}",
            self.l.current_file, self.l.current_line, self.l.current_col, error
        ))
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

        self.add_error(
            format!(
                "Expected next token to be {} but got {} instead.",
                token_type, self.peek_token.typ
            )
            .as_str(),
        );

        false
    }

    /// Move forward in the token hierachy
    fn next_token(&mut self) {
        self.c_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();

        if self.is_debug_mode {
            print!("C Token: ");
            self.c_token.pretty_print();
        }
    }

    /// Expect peek token to be eos
    fn expect_peek_eos(&mut self) -> bool {
        if !self.peek_token_is_eos() {
            self.add_error(
                format!("Expected EOS but got: {} instead", self.peek_token.typ).as_str(),
            );

            return false;
        }

        self.next_token();
        true
    }

    /// is our current token and eos.
    fn cur_token_is_eos(&self) -> bool {
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
    let stmt = match parser.c_token.typ.as_str() {
        token::VAR => parse_var_statement(parser),
        token::IDENT => {
            if parser.peek_token_is(token::ASSIGN) || parser.peek_token_is(token::COLON) || parser.peek_token_is(token::TYPE_ASSIGNMENT) {
                parse_const_var_statement(parser)
            } else {
                parse_expression_statement(parser)
            }
        }
        token::RETURN => parse_return_statement(parser),
        token::USE => parse_use_statement(parser),
        token::JAVASCRIPT => ast::Statement::JavaScriptStatement(
            parser.c_token.to_owned(),
            parser.c_token.to_owned().literal,
        ),
        token::FOR => parse_for_statement(parser),
        token::STRUCT => parse_struct_statement(parser),
        token::PUB => parse_export_statement(parser),
        token::ASYNC => parse_async_block_statement(parser),
        token::DOC_COMMENT => parse_doc_comment_statement(parser),
        token::MATCH => parse_match_statement(parser),
        token::NATIVE => parse_native_statement(parser),
        _ => parse_expression_statement(parser),
    };

    // if next token is a ';'
    if parser.peek_token_is(token::SEMICOLON) {
        parser.next_token();
    }

    stmt
}

fn parse_native_statement(p: &mut Parser) -> ast::Statement {
    p.debug_print("parse_native_statement");
    let token = p.c_token.clone(); // native

    if !p.expect_peek(token::L_BRACE) {
        return ast::empty_statement();
    }

    let mut stmts = vec![];

    while !p.peek_token_is(token::R_BRACE) {
        p.next_token();
        let stmt = parse_statement(p);
        if !stmt.is_empty() {
            stmts.push(stmt);
        }
    }
    p.next_token(); // consume the }

    ast::Statement::NativeStatement(token, Box::new(stmts))
}

fn parse_match_statement(p: &mut Parser) -> ast::Statement {
    p.debug_print("parse_match_statement");
    let token = p.c_token.to_owned(); // match

    if !p.expect_peek(token::IDENT) {
        return ast::empty_statement();
    }

    let expr = parse_expression(p, LOWEST);

    if !p.expect_peek(token::L_BRACE) {
        return ast::empty_statement();
    }

    // a default block will be provided in the case that one is not set
    let mut conditions = vec![];

    if p.peek_token_is(token::R_BRACE) {
        p.next_token(); // consume it and continue
        return ast::Statement::MatchStatement(token, Box::new(expr), Box::new(conditions))
    }

    while !p.peek_token_is(token::R_BRACE) {
        p.next_token(); // go to the condition.
        let left_condition = parse_expression(p, LOWEST);
        if !p.expect_peek(token::COLON) {
            return ast::empty_statement();
        }
        p.next_token(); 
        let right_block = parse_block_statement(p);
        // p.next_token(); // consume the brace...
        conditions.push((left_condition, right_block));
    }

    // expect a ending brace
    if !p.expect_peek(token::R_BRACE) {
        return ast::empty_statement();
    }

    ast::Statement::MatchStatement(token, Box::new(expr), Box::new(conditions))
}

fn parse_doc_comment_statement(p: &mut Parser) -> ast::Statement {
    p.debug_print("parse_doc_comment_statement");
    let token = p.c_token.to_owned(); // ///

    let mut comments = vec![];

    // add first comment
    comments.push(p.c_token.to_owned().literal);

    while p.peek_token_is(token::DOC_COMMENT) {
        p.next_token();
        comments.push(p.c_token.to_owned().literal);
    }

    ast::Statement::DocCommentStatement(token, comments)
}

fn parse_export_statement(p: &mut Parser) -> ast::Statement {
    p.debug_print("parse_export_statement");
    let token = p.c_token.to_owned(); // pub
    p.next_token(); // get stmt

    ast::Statement::ExportStatement(token, Box::new(parse_statement(p)))
}

fn parse_async_block_statement(p: &mut Parser) -> ast::Statement {
    p.debug_print("parse_async_block_statement");
    let token = p.c_token.to_owned();

    if !p.peek_token_is(token::L_BRACE) {
        return parse_expression_statement(p);
    }

    p.next_token(); // {
    let block = parse_block_statement(p);

    ast::Statement::AsyncBlockStatement(token, Box::new(block))
}

fn parse_var_statement(p: &mut Parser) -> ast::Statement {
    p.debug_print("parse_var_statement");
    let token = p.c_token.clone(); // var

    if !p.expect_peek(token::IDENT) {
        return ast::empty_statement();
    }
    let name = parse_identifier(p, false);

    let mut var_type: Option<Box<ast::Expression>> = None;
    // check for type
    if p.peek_token_is(token::COLON) {
        var_type = Some(Box::new(parse_type(p)));
    }

    if !p.peek_token_is(token::ASSIGN) && !p.peek_token_is(token::TYPE_ASSIGNMENT) {
        p.add_error(format!("Expected {} or {} but got {} instead.", token::ASSIGN, token::TYPE_ASSIGNMENT, p.peek_token.literal).as_str());
        return ast::Statement::EmptyStatement;
    } 
    let infer_type = p.peek_token_is(token::TYPE_ASSIGNMENT);
    p.next_token();
    p.next_token();

    let value = parse_expression(p, LOWEST);

    ast::Statement::VariableStatement(token, Box::new(name), var_type, Box::new(value), infer_type)
}

fn parse_const_var_statement(p: &mut Parser) -> ast::Statement {
    p.debug_print("parse_const_var_statement");
    let token = p.c_token.clone();

    let mut var_type: Option<Box<ast::Expression>> = None;
    // check for type
    if p.peek_token_is(token::COLON) {
        var_type = Some(Box::new(parse_type(p)));
    }

    if !p.peek_token_is(token::ASSIGN) && !p.peek_token_is(token::TYPE_ASSIGNMENT) {
        p.add_error(format!("Expected {} or {} but got {} instead.", token::ASSIGN, token::TYPE_ASSIGNMENT, p.peek_token.literal).as_str());
        return ast::Statement::EmptyStatement;
    } 
    let infer_type = p.peek_token_is(token::TYPE_ASSIGNMENT);
    p.next_token();

    let name = ast::Expression::Identifier(token.to_owned(), token.to_owned().literal);

    p.next_token();

    let value = parse_expression(p, LOWEST);

    if value.is_empty() {
        return ast::Statement::EmptyStatement;
    }

    ast::Statement::ConstVariableStatement(token, Box::new(name), var_type, Box::new(value), infer_type)
}

fn parse_return_statement(p: &mut Parser) -> ast::Statement {
    p.debug_print("parse_return_statement");
    let token = p.c_token.clone();

    p.next_token();

    let value = parse_expression(p, LOWEST);

    ast::Statement::ReturnStatement(token, Box::new(value))
}

fn parse_expression_statement(p: &mut Parser) -> ast::Statement {
    p.debug_print("parse_expression_statement");
    let token = p.c_token.clone();
    let expression = parse_expression(p, LOWEST);

    if expression.is_empty() {
        return ast::Statement::EmptyStatement;
    }

    ast::Statement::ExpressionStatement(token, Box::new(expression))
}

fn parse_use_statement(p: &mut Parser) -> ast::Statement {
    p.debug_print("parse_use_statement");
    let token = p.c_token.clone(); // use

    let mut is_use_from = false;
    let mut import_args = vec![];

    // check if this is a use_from.
    if p.peek_token_is(token::L_BRACE) {
        p.next_token(); // consume {
        p.next_token(); // get to the args...
                        // get the expressions here...
        is_use_from = true;
        import_args.push(parse_expression(p, LOWEST));
        while p.peek_token_is(token::COMMA) {
            p.next_token(); // consume the ,
            p.next_token(); // expression
            import_args.push(parse_expression(p, LOWEST));
        }
        if !p.expect_peek(token::R_BRACE) {
            return ast::empty_statement();
        }

        // expect the from token
        if !p.expect_peek(token::FROM) {
            return ast::empty_statement();
        }
    }
    p.next_token();

    // get the prefix:path
    let prefix = parse_identifier(p, false);

    if !p.expect_peek(token::COLON) {
        return ast::empty_statement();
    }
    p.next_token();

    let path = parse_expression(p, LOWEST);

    if is_use_from {
        return ast::Statement::UseFromStatement(
            token,
            Box::new(import_args),
            Box::new(prefix),
            Box::new(path),
        );
    }

    ast::Statement::UseStatement(token, Box::new(prefix), Box::new(path))
}

fn parse_block_statement(p: &mut Parser) -> ast::Statement {
    p.debug_print("parse_block_statement");
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
    p.debug_print("parse_for_statement");
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
    p.debug_print("parse_expression");
    let token_type = p.c_token.typ.clone();
    let prefix = p.has_prefix(&token_type);
    if !prefix {
        p.add_error(format!("No prefix function for {} found.", token_type).as_str());
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
    p.debug_print("parse_prefix_expression");
    let token = p.c_token.clone();
    let operator = p.c_token.literal.to_owned();

    p.next_token();

    let right = parse_expression(p, LOWEST);

    ast::Expression::PrefixExpression(token, operator, Box::new(right))
}

/// Parse an identifier
fn parse_identifier(parser: &mut Parser, try_parse_type: bool) -> ast::Expression {
    parser.debug_print("parse_identifier");
    let token = parser.c_token.clone();
    let mut lit = token.literal.to_owned();
    // check if we are parsing a self
    if token.typ == token::SELF {
        lit = "this".to_owned();
    } 

    // should we try to parse a type?
    if try_parse_type {
        if parser.peek_token_is(token::COLON) {
            return ast::Expression::IdentifierWithType(token, lit, Box::new(parse_type(parser)));
        }
    }
    ast::Expression::Identifier(token, lit)
}

/// Parse a type
fn parse_type(p: &mut Parser) -> ast::Expression {
    p.debug_print("parse_type");
    let token = p.c_token.clone();
    
    // consume the : 
    p.next_token();
    // and go to the identifer
    if !p.expect_peek(token::IDENT) {
        return ast::Expression::EmptyExpression;
    }

    ast::Expression::Type(token, p.c_token.literal.clone())
}

/// parse an integer literal, returns EmptyExpression if not valid.
fn parse_integer_literal(parser: &mut Parser) -> ast::Expression {
    parser.debug_print("parse_integer_literal");
    let tk = parser.c_token.clone();
    // check is number
    let is_number = parser.c_token.literal.parse::<i64>().is_ok();
    if !is_number {
        parser.add_error(format!("Epected type INT got {} instead", tk.literal).as_str());
        return ast::Expression::EmptyExpression;
    }
    let integer = parser.c_token.literal.parse::<i64>().unwrap();

    ast::Expression::IntegerLiteral(tk, integer)
}

/// parse a boolean
fn parse_boolean(p: &mut Parser) -> ast::Expression {
    p.debug_print("parse_boolean");
    ast::Expression::Boolean(p.c_token.clone(), p.cur_token_is(token::TRUE))
}

fn parse_group_expression(p: &mut Parser) -> ast::Expression {
    p.debug_print("parse_group_expression");
    let token = p.c_token.clone();
    p.next_token();
    let exp = parse_expression(p, LOWEST);
    if !p.expect_peek(token::R_PAREN) {
        return ast::Expression::EmptyExpression;
    }

    ast::Expression::GroupedExpression(token, Box::new(exp))
}

fn parse_if_expression(p: &mut Parser) -> ast::Expression {
    p.debug_print("parse_if_expression");
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
    p.debug_print("parse_function_literal");
    let token = p.c_token.clone();

    if p.peek_token_is(token::L_PAREN) {
        // this is a lambda
        return parse_lambda_literal(p);
    }

    if p.peek_token_is(token::L_BRACE) {
        // this is a IIFE.
        return parse_iife_literal(p);
    }

    // ok lets make sure this is a function
    if !(p.peek_token_is(token::IDENT) || p.peek_token_is(token::NEW)) {
        p.add_error(format!("Expected a IDENT or NEW, got {} instead", p.peek_token.typ).as_str());
        // not a function
        return ast::Expression::EmptyExpression;
    }
    p.next_token();
    let name = parse_identifier(p, false);

    if !p.expect_peek(token::L_PAREN) {
        return ast::Expression::EmptyExpression;
    }

    // params
    let parameters = parse_function_paramaters(p);

    let mut var_type: Option<Box<ast::Expression>> = None;
    // check for type
    if p.peek_token_is(token::COLON) {
        var_type = Some(Box::new(parse_type(p)));
    }

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
        var_type,
        Box::new(body),
    )
}

fn parse_function_paramaters(p: &mut Parser) -> Vec<ast::Expression> {
    p.debug_print("parse_function_paramaters");
    // starts at (
    let mut idents = vec![];

    // if we got no params
    if p.peek_token_is(token::R_PAREN) {
        p.next_token(); // consume the )
        return idents;
    }

    // go to first identifier
    p.next_token();
    idents.push(parse_identifier(p, true));

    while p.peek_token_is(token::COMMA) {
        p.next_token();
        p.next_token();
        idents.push(parse_identifier(p, true));
    }

    if !p.expect_peek(token::R_PAREN) {
        return vec![ast::Expression::EmptyExpression];
    }

    idents
}

fn parse_lambda_literal(p: &mut Parser) -> ast::Expression {
    p.debug_print("parse_lambda_literal");
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

fn parse_iife_literal(p: &mut Parser) -> ast::Expression {
    p.debug_print("parse_iife_literal");
    let token = p.c_token.clone();

    if !p.expect_peek(token::L_BRACE) {
        return ast::Expression::EmptyExpression;
    }

    // parse block
    let block = parse_block_statement(p);

    if block.is_empty() {
        return ast::Expression::EmptyExpression;
    }

    ast::Expression::IIFE(token.to_owned(), Box::new(block))
}

fn parse_string_literal(p: &mut Parser) -> ast::Expression {
    p.debug_print("parse_string_literal");
    ast::Expression::StringLiteral(p.c_token.clone().to_owned(), p.c_token.to_owned().literal)
}

fn parse_comment(p: &mut Parser) -> ast::Expression {
    p.debug_print("parse_comment");
    ast::Expression::CommentExpression(p.c_token.to_owned(), p.c_token.to_owned().literal)
}

fn parse_array_literal(p: &mut Parser) -> ast::Expression {
    p.debug_print("parse_array_literal");
    let token = p.c_token.to_owned();
    let elements = parse_array_arguments(p);
    ast::Expression::ArrayLiteral(token, Box::new(elements))
}

fn parse_array_arguments(p: &mut Parser) -> Vec<ast::Expression> {
    p.debug_print("parse_array_arguments");
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
    p.debug_print("parse_object_literal");
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
        // check if key : value
        if p.peek_token_is(token::COLON) {
            p.next_token(); // move out of key
            p.next_token(); // move out of : and into value
            let value = parse_expression(p, LOWEST);

            // check emtpy
            if key.is_empty() || value.is_empty() {
                p.add_error("Empty key or value in object literal".to_string().as_str());
            }

            elements.push(vec![Box::new(key), Box::new(value)]);
        } else {
            // this is not a key : value pair, probably just a KEY
            // but check the key type, it must be a identifier
            match key.clone() {
                ast::Expression::Identifier(_, name) => {
                    elements.push(vec![Box::new(key.clone()), Box::new(key)]);
                }
                _ => {
                    p.add_error("Expected a key in object literal".to_string().as_str());
                    return ast::Expression::EmptyExpression;
                }
            }
        }

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
    p.debug_print("parse_async_expressoin");
    let token = p.c_token.to_owned();
    if !p.expect_peek(token::FUNCTION) {
        return ast::Expression::EmptyExpression;
    }

    ast::Expression::AsyncExpression(token.to_owned(), Box::new(parse_function_literal(p)))
}

fn parse_await_expression(p: &mut Parser) -> ast::Expression {
    p.debug_print("parse_await_expression");
    let token = p.c_token.to_owned();
    p.next_token(); // consome keyword
    let value = parse_expression(p, LOWEST);

    if value.is_empty() {
        return value.to_owned();
    }

    ast::Expression::AwaitExpression(token, Box::new(value))
}

fn parse_infix_expression(p: &mut Parser, left: ast::Expression) -> ast::Expression {
    p.debug_print("parse_infix_expression");
    let token = p.c_token.to_owned();
    let operator = p.c_token.to_owned().literal;

    let precedence = p.cur_precedence();
    p.next_token();
    let right = parse_expression(p, precedence);

    ast::Expression::InfixExpression(token, Box::new(left), operator, Box::new(right))
}

fn parse_call_expression(p: &mut Parser, left: ast::Expression) -> ast::Expression {
    p.debug_print("parse_call_expression");
    let token = p.c_token.to_owned();
    let arguments = parse_call_arguments(p);

    ast::Expression::CallExpression(token, Box::new(left), Box::new(arguments))
}

fn parse_call_arguments(p: &mut Parser) -> Vec<ast::Expression> {
    p.debug_print("parse_call_arguments");
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
    p.debug_print("parse_dot_expression");
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
    p.debug_print("parse_index_expression");
    let token = p.c_token.to_owned();

    if p.peek_token_is(token::R_BRACKET) {
        return ast::Expression::EmptyExpression;
    }
    p.next_token();

    let index = parse_expression(p, LOWEST);

    if index.is_empty() {
        return ast::Expression::EmptyExpression;
    }

    if !p.expect_peek(token::R_BRACKET) {
        return ast::Expression::EmptyExpression;
    }

    ast::Expression::IndexExpression(token, Box::new(left), Box::new(index))
}

fn parse_range_expression(p: &mut Parser, left: ast::Expression) -> ast::Expression {
    p.debug_print("parse_range_expression");
    let token = p.c_token.to_owned();
    p.next_token();

    let right = parse_expression(p, LOWEST);
    if right.is_empty() {
        return ast::Expression::EmptyExpression;
    }

    ast::Expression::RangeExpression(token, Box::new(left), Box::new(right))
}

fn parse_in_expression(p: &mut Parser, left: ast::Expression) -> ast::Expression {
    p.debug_print("parse_in_expression");
    let token = p.c_token.to_owned();
    p.next_token();
    let right = parse_expression(p, LOWEST);
    if right.is_empty() {
        return ast::Expression::EmptyExpression;
    }

    ast::Expression::InExpression(token, Box::new(left), Box::new(right))
}

fn parse_of_expression(p: &mut Parser, left: ast::Expression) -> ast::Expression {
    p.debug_print("parse_of_expression");
    let token = p.c_token.to_owned();
    p.next_token();
    let right = parse_expression(p, LOWEST);
    if right.is_empty() {
        return ast::Expression::EmptyExpression;
    }

    ast::Expression::OfExpression(token, Box::new(left), Box::new(right))
}

fn parse_assign_expression(p: &mut Parser, left: ast::Expression) -> ast::Expression {
    p.debug_print("parse_assign_expression");
    let token = p.c_token.clone(); // =
    p.next_token();
    let right = parse_expression(p, LOWEST);

    ast::Expression::AssignExpression(token, Box::new(left), Box::new(right))
}

fn parse_not_expression(p: &mut Parser) -> ast::Expression {
    p.debug_print("parse_not_expression");
    let token = p.c_token.to_owned(); // not
    p.next_token();

    let expression = parse_expression(p, LOWEST);

    if expression.is_empty() {
        return ast::empty_expression();
    }
    ast::Expression::NotExpression(token, Box::new(expression))
}

fn parse_as_expression(p: &mut Parser, left: ast::Expression) -> ast::Expression {
    p.debug_print("parse_as_expression");
    let token = p.c_token.to_owned(); // as

    if !p.expect_peek(token::IDENT) {
        return ast::empty_expression();
    }

    let right = parse_expression(p, LOWEST);

    ast::Expression::AsExpression(token, Box::new(left), Box::new(right))
}

fn parse_is_expression(p: &mut Parser, left: ast::Expression) -> ast::Expression {
    p.debug_print("parse_is_expression");
    let token = p.c_token.to_owned(); // is

    p.next_token();

    let right = parse_expression(p, LOWEST);

    ast::Expression::IsExpression(token, Box::new(left), Box::new(right))
}

// fn parse_macro_expression(p: &mut Parser) -> ast::Expression {
//     p.debug_print("parse_macro_expression");
//     let token = p.c_token.to_owned(); // $ || @

//     if !p.expect_peek(token::IDENT) {
//         return ast::empty_expression();
//     }
//     let ident = parse_identifier(p);

//     if !p.expect_peek(token::L_PAREN) {
//         return ast::empty_expression();
//     }

//     let args = {
//         let mut args = Vec::new();
//         if p.peek_token_is(token::R_PAREN) {
//             args
//         } else {
//             p.next_token();
//             args.push(parse_expression(p, LOWEST));
//             while p.peek_token_is(token::COMMA) {
//                 p.next_token(); // ,
//                 p.next_token(); // expr
//                 args.push(parse_expression(p, LOWEST));
//             }
//             args
//         }
//     };

//     if !p.expect_peek(token::R_PAREN) {
//         return ast::empty_expression();
//     }

//     // let args = parse_args
//     ast::Expression::MacroExpression(token, Box::new(ident), Box::new(args))
// }

// fn parse_macro_decleration(p: &mut Parser) -> ast::Expression {
//     p.debug_print("parse_macro_decleration");
//     let token = p.c_token.to_owned(); // macro

//     if !p.expect_peek(token::IDENT) {
//         return ast::empty_expression();
//     }

//     let name = parse_identifier(p);

//     if !p.expect_peek(token::L_PAREN) {
//         return ast::empty_expression();
//     }

//     let args = {
//         let mut args = Vec::new();
//         if p.peek_token_is(token::R_PAREN) {
//             args
//         } else {
//             p.next_token();
//             args.push(parse_expression(p, LOWEST));
//             while p.peek_token_is(token::COMMA) {
//                 p.next_token(); // ,
//                 p.next_token(); // expr
//                 args.push(parse_expression(p, LOWEST));
//             }
//             args
//         }
//     };

//     if !p.expect_peek(token::R_PAREN) {
//         return ast::empty_expression();
//     }

//     if !p.expect_peek(token::L_BRACE) {
//         return ast::empty_expression();
//     }

//     let body = parse_block_statement(p);

//     ast::Expression::MacroDecleration(token, Box::new(name), Box::new(args), Box::new(body))
//     // ast::Expression::MacroLiteral(token)
// }

fn parse_struct_statement(p: &mut Parser) -> ast::Statement {
    p.debug_print("parse_struct_statement");
    let token = p.c_token.to_owned(); // struct
    if !p.expect_peek(token::IDENT) {
        return ast::empty_statement();
    }
    let ident = parse_identifier(p, false);

    let mut constructor_vars : Option<Box<Vec<ast::Expression>>> = None;

    // We have constructor variables
    if p.peek_token_is(token::L_BRACKET) {
        let mut constructor_vars_vector = vec![];
        p.next_token(); // consume the [
        p.next_token(); // be on the indentifier

        constructor_vars_vector.push(parse_identifier(p, true));

        while p.peek_token_is(token::COMMA) {
            p.next_token(); // consume the ,
            p.next_token(); // be on the indentifier
            constructor_vars_vector.push(parse_identifier(p, true));
        }

        constructor_vars = Some(Box::new(constructor_vars_vector));
        p.next_token(); // consume the ]
    }

    let mut mixins: Option<Box<Vec<ast::Expression>>> = None;
   if p.peek_token_is(token::WITH) {
        let mut mixin_names = vec![];
        p.next_token(); // consume the WITH
        p.next_token(); // be on the mixin
        mixin_names.push(parse_identifier(p, false));

        while p.peek_token_is(token::COMMA) {
            p.next_token(); // consume the ,
            p.next_token(); // be on the mixin
            mixin_names.push(parse_identifier(p, false));
        }

        mixins = Some(Box::new(mixin_names));
    }

    if !p.expect_peek(token::L_BRACE) {
        return ast::empty_statement();
    }

    let mut methods = vec![];
    let mut variables = vec![];

    if p.peek_token_is(token::R_BRACE) {
        p.next_token(); // consume the }
        return ast::Statement::StructStatement(
            token,
            Box::new(ident),
            constructor_vars,
            mixins,
            Box::new(variables),
            Box::new(methods),
        );
    }

    // Check if we have a list of variables
    if p.peek_token_is(token::IDENT) || p.peek_token_is(token::VAR) {
        p.next_token();
        loop {
            let stmt = parse_statement(p);
            // only allow variable stmts for the moment.
            if !stmt.eq(ast::Statement::VariableStatement(
                token::EMPTY_TOKEN,
                ast::empty_box_exp(),
                None,
                ast::empty_box_exp(),
                false
            )) && !stmt.eq(ast::Statement::ConstVariableStatement(
                token::EMPTY_TOKEN,
                ast::empty_box_exp(),
                None,
                ast::empty_box_exp(),
                false
            )) {
                return ast::empty_statement();
            }
            variables.push(stmt);
            if !p.peek_token_is(token::IDENT) {
                break;
            }
            p.next_token();
        }
        // the vars is closed...
        if p.peek_token_is(token::R_BRACE) {
            p.next_token();
            return ast::Statement::StructStatement(
                token,
                Box::new(ident),
                constructor_vars,
                mixins,
                Box::new(variables),
                Box::new(methods),
            );
        }
    }

    // what else could this be???
    if !p.expect_peek(token::FUNCTION) {
        return ast::empty_statement();
    }

    // start parsing the functions
    loop {
        let func = parse_expression(p, LOWEST);
        if !func.is_empty() {
            methods.push(func);
        }

        if p.cur_token_is(token::R_BRACE) && p.peek_token_is(token::R_BRACE) {
            break;
        }

        p.next_token();
    }
    p.next_token();

    ast::Statement::StructStatement(
        token,
        Box::new(ident),
        constructor_vars,
        mixins,
        Box::new(variables),
        Box::new(methods),
    )
}

fn parse_and_expression(p: &mut Parser, left: ast::Expression) -> ast::Expression {
    p.debug_print("parse_and_expression");
    let token = p.c_token.to_owned(); // &&
    p.next_token();
    let right = parse_expression(p, LOWEST);

    ast::Expression::AndExpression(token, Box::new(left), Box::new(right))
}

fn parse_or_expression(p: &mut Parser, left: ast::Expression) -> ast::Expression {
    p.debug_print("parse_or_expression");
    let token = p.c_token.to_owned(); // ||
    p.next_token();
    let right = parse_expression(p, LOWEST);

    ast::Expression::OrExpression(token, Box::new(left), Box::new(right))
}

fn parse_question_mark_expression(p: &mut Parser, left: ast::Expression) -> ast::Expression {
    p.debug_print("parse_question_mark_expression");
    let token = p.c_token.to_owned(); // ?
    p.next_token();
    let right = parse_expression(p, LOWEST);

    ast::Expression::NullExpression(token, Box::new(left), Box::new(right))
}

fn parse_double_question_mark_expression(p: &mut Parser, left: ast::Expression) -> ast::Expression {
    p.debug_print("parse_double_question_mark_expression");
    let token = p.c_token.to_owned(); // ??
    p.next_token();
    let right = parse_expression(p, LOWEST);

    ast::Expression::DefaultIfNullExpression(token, Box::new(left), Box::new(right))
}

fn parse_new_expression(p: &mut Parser) -> ast::Expression {
    p.debug_print("parse_new_expression");
    let token = p.c_token.to_owned(); // new

    // expect a identifier
    if !p.expect_peek(token::IDENT) {
        p.next_token();
        return ast::empty_expression();
    }

    let ident = parse_expression(p, LOWEST);

    ast::Expression::NewClassExpression(token, Box::new(ident))
}

fn parse_float_literal(p: &mut Parser) -> ast::Expression {
    p.debug_print("parse_float_literal");
    let token = p.c_token.to_owned(); // 1.0
                                      // check is float
    let is_float = p.c_token.literal.parse::<f64>().is_ok();
    if !is_float {
        p.add_error(format!("Epected type FLOAT got {} instead", token.literal).as_str());
        return ast::empty_expression();
    }

    let float = p.c_token.literal.parse::<f64>().unwrap();

    ast::Expression::FloatLiteral(token, float)
}

fn parse_builtin_expression(p: &mut Parser) -> ast::Expression {
    p.debug_print("parse_builtin_expression");
    let token = p.c_token.to_owned(); // builtin

    if !p.expect_peek(token::L_PAREN) {
        // (
        return ast::empty_expression();
    }

    let mut args = vec![];

    if p.peek_token_is(token::R_PAREN) {
        // )
        p.next_token();
        return ast::Expression::BuiltinCall(token, Box::new(args));
    }
    p.next_token(); // consume the (

    while !p.cur_token_is(token::R_PAREN) {
        // !)
        args.push(parse_expression(p, LOWEST));
        p.next_token();

        // check if , is current
        if p.cur_token_is(token::COMMA) {
            p.next_token();
        }
    }

    ast::Expression::BuiltinCall(token, Box::new(args))
}
