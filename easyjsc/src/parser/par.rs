use crate::lexer::{lex, token};
use crate::parser::ast::Statement::VariableStatement;
use crate::parser::ast::{self, Expression};

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

/// Prefix Function Type
type PrefixFunction = fn(&mut Parser) -> ast::Expression;
/// Infix Function Type
type InfixFunction = fn(&mut Parser, ast::Expression) -> ast::Expression;

// Constant values
const LOWEST: i64 = 1;

// math
const EQUALS: i64 = 2; // == !=

const LESSGREATER: i64 = 3; // < > >= <=
const SUM: i64 = 4; // + - ::
const PRODUCT: i64 = 5; // * /

const DOT: i64 = 6; // .field or .method or ...spread

const CALL: i64 = 7; // my_function(X)
const BRACKET: i64 = 10; // [
const BRACE: i64 = 11; // {
const DOTDOT: i64 = 12; // ..
const IN: i64 = 13; // in
const AWAIT: i64 = 15; // await

const ASSIGN: i64 = 16;

const AS: i64 = 18;
const MACRO_SYMBOL: i64 = 19;
// const DECORATOR: i64 = 20;
const DOC_COMMENT: i64 = 20;
const AND: i64 = 22;
const OR: i64 = 23;
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
        token::SPREAD => DOT,
        token::LT_OR_EQ => LESSGREATER,
        token::GT_OR_EQ => LESSGREATER,
        token::L_BRACKET => BRACKET,
        token::L_BRACE => BRACE,
        token::DOTDOT => DOTDOT,
        token::IN => IN,
        token::IS => IN,
        token::AWAIT => AWAIT,
        token::ASSIGN => ASSIGN,
        token::AS => AS,
        token::DOC_COMMENT => DOC_COMMENT,
        token::BITWISE_OR => OR,
        token::BITWISE_AND => AND,
        token::AND_SYMBOL => AND,
        token::OR_SYMBOL => OR,
        token::DOUBLE_QUESTION_MARK => DOUBLE_QUESTION_MARK,
        token::MODULUS => PRODUCT,
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
            c_token: token::new_token("", "", "", -1, -1),
            peek_token: token::new_token("", "", "", -1, -1),
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

    /// Returns a prefix function or None
    fn prefix_fns(&mut self, token_type: &str) -> Option<PrefixFunction> {
        match token_type {
            token::IDENT => Some(parse_identifier),
            token::THIS => Some(parse_identifier),
            token::INT => Some(parse_integer_literal),
            token::FLOAT => Some(parse_float_literal),
            token::BANG => Some(parse_prefix_expression),
            token::NOT => Some(parse_not_expression),
            token::MINUS => Some(parse_prefix_expression),
            token::TRUE => Some(parse_boolean),
            token::FALSE => Some(parse_boolean),
            token::NULL => Some(parse_null),
            token::L_PAREN => Some(parse_group_expression),
            token::IF => Some(parse_if_expression),
            token::FUNCTION => Some(parse_function_literal),
            token::STRING => Some(parse_string_literal),
            token::COMMENT => Some(parse_comment),
            token::L_BRACKET => Some(parse_array_literal),
            token::ASYNC => Some(parse_async_expressoin),
            token::AS => Some(parse_as_expression),
            token::AWAIT => Some(parse_await_expression),
            token::SPREAD => Some(parse_spread_expression),
            token::DOC_COMMENT => Some(parse_doc_comment_expression),
            token::CLASS => Some(parse_class_expression),
            token::IMPORT => Some(parse_import_expression),
            _ => None,
        }
    }

    /// This is how we do it, run this function to call a infix method.
    fn infix_fns(&mut self, token_type: &str) -> Option<InfixFunction> {
        match token_type {
            token::PLUS => Some(parse_infix_expression),
            token::MINUS => Some(parse_infix_expression),
            token::SLASH => Some(parse_infix_expression),
            token::ASTERISK => Some(parse_infix_expression),
            token::EQ => Some(parse_infix_expression),
            token::NOT_EQ => Some(parse_infix_expression),
            token::LT => Some(parse_infix_expression),
            token::GT => Some(parse_infix_expression),
            token::LT_OR_EQ => Some(parse_infix_expression),
            token::GT_OR_EQ => Some(parse_infix_expression),
            token::L_PAREN => Some(parse_call_expression),
            token::DOT => Some(parse_dot_expression),
            token::JAVASCRIPT => Some(parse_infix_expression),
            token::L_BRACKET => Some(parse_index_expression),
            token::DOTDOT => Some(parse_range_expression),
            token::IN => Some(parse_in_expression),
            token::ASSIGN => Some(parse_assign_expression),
            token::AND_SYMBOL => Some(parse_and_expression),
            token::BITWISE_AND => Some(parse_infix_expression),
            token::BITWISE_OR => Some(parse_infix_expression),
            token::OR_SYMBOL => Some(parse_or_expression),
            token::DOUBLE_QUESTION_MARK => Some(parse_double_question_mark_expression),
            token::MODULUS => Some(parse_infix_expression),
            token::PLUS_EQUALS => Some(parse_infix_expression),
            token::MINUS_EQUALS => Some(parse_infix_expression),
            token::SLASH_EQUALS => Some(parse_infix_expression),
            token::ASTERISK_EQUALS => Some(parse_infix_expression),
            token::IS => Some(parse_is_expression),
            _ => None,
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

    /// Is (token) or EOF
    fn peek_token_is_or_eof(&self, token_type: &str) -> bool {
        self.peek_token_is(token_type) || self.peek_token_is(token::EOF)
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
        // token::VAR => parse_var_statement(parser),
        token::IDENT => {
            if parser.peek_token_is(token::ASSIGN)
                || parser.peek_token_is(token::COLON)
                || parser.peek_token_is(token::CONSTANT_ASSIGNMENT)
            {
                parse_var_statement(parser)
            } else {
                parse_expression_statement(parser)
            }
        }
        token::RETURN => parse_return_statement(parser),
        // token::USE => parse_use_statement(parser),
        token::JAVASCRIPT => ast::Statement::JavaScriptStatement(
            parser.c_token.to_owned(),
            parser.c_token.to_owned().literal,
        ),
        token::FOR => parse_for_statement(parser),
        token::PUB => parse_export_statement(parser),
        token::ASYNC => parse_async_block_statement(parser),
        token::MATCH => parse_match_statement(parser),
        token::BREAK => parse_break_statement(parser),
        token::CONTINUE => parse_continue_statement(parser),
        token::ANNOTATOR => parse_annotation_statement(parser),
        _ => parse_expression_statement(parser),
    };

    // if next token is a ';'
    if parser.peek_token_is(token::SEMICOLON) {
        parser.next_token();
    }

    stmt
}

fn parse_break_statement(p: &mut Parser) -> ast::Statement {
    p.debug_print("parse_break_statement");
    let token = p.c_token.clone();

    ast::Statement::BreakStatement(token)
}

fn parse_continue_statement(p: &mut Parser) -> ast::Statement {
    p.debug_print("parse_continue_statement");
    ast::Statement::ContinueStatement(p.c_token.clone())
}

fn parse_class_expression(p: &mut Parser) -> ast::Expression {
    p.debug_print("parse_class_expression");
    let token = p.c_token.clone();
    if !p.expect_peek(token::L_BRACE) {
        return ast::empty_expression();
    }
    let mut stmts = vec![];
    while !p.peek_token_is_or_eof(token::R_BRACE) {
        p.next_token();
        let stmt = parse_statement(p);
        stmts.push(stmt);
    }
    if !p.expect_peek(token::R_BRACE) {
        return ast::empty_expression();
    }

    ast::Expression::Class(token, stmts)
}

fn parse_import_expression(p: &mut Parser) -> ast::Expression {
    p.debug_print("parse_import_expression");
    let token = p.c_token.clone();
    if !p.expect_peek(token::L_PAREN) {
        return ast::empty_expression();
    }
    if !p.expect_peek(token::STRING) {
        return ast::empty_expression();
    }
    let path = parse_string_literal(p);
    ast::Expression::Import(token, Box::new(path))
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
        return ast::Statement::MatchStatement(token, Box::new(expr), Box::new(conditions));
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

fn parse_doc_comment_expression(p: &mut Parser) -> ast::Expression {
    p.debug_print("parse_doc_comment_statement");
    let token = p.c_token.to_owned(); // ///

    let mut comments = vec![];

    // add first comment
    comments.push(p.c_token.to_owned().literal);

    while p.peek_token_is(token::DOC_COMMENT) {
        p.next_token();
        comments.push(p.c_token.to_owned().literal);
    }

    ast::Expression::DocCommentExpression(token, comments)
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
    let token = p.c_token.clone(); // identifier

    let name = parse_identifier(p);

    let mut var_type: Box<ast::Expression>;
    // check for type
    if p.peek_token_is(token::COLON) {
        var_type = Box::new(parse_type(p));
    } else {
        var_type = Box::new(none_type(token.clone()));
    }

    // A non initialized variable
    if !p.peek_token_is(token::ASSIGN) && !p.peek_token_is(token::CONSTANT_ASSIGNMENT) {
        return VariableStatement(token, Box::new(name), var_type, Box::new(ast::empty_expression()));
    }

    let is_const = p.peek_token_is(token::CONSTANT_ASSIGNMENT);
    p.next_token();
    p.next_token();

    let value = parse_expression(p, LOWEST);

    if is_const {
        ast::Statement::ConstVariableStatement(token, Box::new(name), var_type, Box::new(value))
    } else {
        ast::Statement::VariableStatement(token, Box::new(name), var_type, Box::new(value))
    }
}

fn parse_return_statement(p: &mut Parser) -> ast::Statement {
    p.debug_print("parse_return_statement");
    let token = p.c_token.clone();

    // Sometimes the return is empty
    if p.peek_token_is(token::R_BRACE) {
        return ast::Statement::ReturnStatement(token, Box::new(empty_expression()));
    }
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

fn parse_annotation_statement(p: &mut Parser) -> ast::Statement {
    p.debug_print("parse_annotation_statement");
    let token = p.c_token.to_owned();

    if !p.expect_peek(token::IDENT) {
        return ast::empty_statement();
    }

    let annotation = token.literal.to_owned();
    // Parse following statement
    p.next_token();
    let stmt = parse_statement(p);

    ast::Statement::AnnotationStatement(token, annotation, Box::new(stmt))
}

fn parse_expression(p: &mut Parser, precedence: i64) -> ast::Expression {
    p.debug_print("parse_expression");
    let token_type = p.c_token.typ.clone();
    let mut left_exp = if let Some(prefix_fn) = p.prefix_fns(&token_type) {
        prefix_fn(p)
    } else {
        return ast::Expression::EmptyExpression;
    };

    while !(p.peek_token_is_eos() || p.peek_token_is(token::EOF))
        && precedence < p.peek_precedence()
    {
        let peek_type = p.peek_token.typ.clone();
        if let Some(infix_fn) = p.infix_fns(&token_type) {
            p.next_token();
            left_exp = infix_fn(p, left_exp);
        } else {
            return left_exp;
        }
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
fn parse_identifier(parser: &mut Parser) -> ast::Expression {
    parser.debug_print("parse_identifier");
    let token = parser.c_token.clone();

    let mut lit = token.literal.to_owned();

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
        let mut is_elif = false;
        p.next_token(); // consume else
        if p.peek_token_is(token::IF) {
            is_elif = true;
            p.next_token();
            elseif = parse_if_expression(p);
        }
        if !is_elif {
            if !p.expect_peek(token::L_BRACE) {
                return ast::Expression::EmptyExpression;
            }

            // we got em
            else_ = parse_block_statement(p);
        }
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

    // if p.peek_token_is(token::L_PAREN) {
        // this is a lambda
        // return parse_lambda_literal(p);
    // }

    if p.peek_token_is(token::L_BRACE) {
        // this is a IIFE.
        return parse_iife_literal(p);
    }

    if !p.expect_peek(token::L_PAREN) {
        return ast::empty_expression();
    }

    // params
    let parameters = parse_function_paramaters(p);

    let var_type = if p.peek_token_is(token::RETURN_TYPE) {
        parse_type(p)
    } else {
        dyn_type(p.c_token.clone())
    };

    if !p.expect_peek(token::L_BRACE) {
        return ast::Expression::EmptyExpression;
    }

    let body = parse_block_statement(p);
    if body.is_empty() {
        return ast::Expression::EmptyExpression;
    }

    ast::Expression::Function(token.to_owned(), Box::new(parameters), Box::new(var_type), body)
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
    loop {
        idents.push(parse_expression(p, LOWEST));
        // Leave the loop dog!
        if !p.peek_token_is(token::COMMA) {
            break;
        } else {
            p.next_token(); // Comma
            p.next_token(); // get to expression
        }
    }

    if !p.expect_peek(token::R_PAREN) {
        return vec![ast::Expression::EmptyExpression];
    }

    idents
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

// fn parse_object_literal(p: &mut Parser) -> ast::Expression {
//     p.debug_print("parse_object_literal");
//     let token = p.c_token.to_owned();
//     let mut elements = vec![];

//     if p.peek_token_is(token::R_BRACE) {
//         // consume it
//         p.next_token();

//         return ast::Expression::ObjectLiteral(token.to_owned(), elements);
//     }

//     let mut brace_count = 1;
//     while !p.peek_token_is(token::EOF) {
//         p.next_token();

//         if p.cur_token_is(token::L_BRACE) {
//             brace_count += 1;
//         } else if p.cur_token_is(token::R_BRACE) {
//             brace_count -= 1;
//             if brace_count == 0 {
//                 break;
//             }
//         }

//         // Key has to be either string or identifier
//         let key = parse_key_expression(p);
//         // let key = parse_expression(p, LOWEST);
//         // check if key : value
//         if p.peek_token_is(token::COLON) {
//             p.next_token(); // move out of key
//             p.next_token(); // move out of : and into value
//             let value = parse_expression(p, LOWEST);

//             // check emtpy
//             if key.is_empty() || value.is_empty() {
//                 p.add_error("Empty key or value in object literal".to_string().as_str());
//             }

//             elements.push(vec![Box::new(key), Box::new(value)]);
//         } else {
//             // this is not a key : value pair, probably just a KEY
//             // but check the key type, it must be a identifier
//             match key.clone() {
//                 ast::Expression::Identifier(_, name) => {
//                     elements.push(vec![Box::new(key.clone()), Box::new(key)]);
//                 }
//                 _ => {
//                     p.add_error("Expected a key in object literal".to_string().as_str());
//                     return ast::Expression::EmptyExpression;
//                 }
//             }
//         }

//         // check for comma.
//         if p.peek_token_is(token::COMMA) {
//             p.next_token();
//         }
//     }

//     if !p.cur_token_is(token::R_BRACE) {
//         // what what what??
//         return ast::Expression::EmptyExpression;
//     }

//     ast::Expression::ObjectLiteral(token, elements)
// }

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

    if !p.expect_peek(token::IDENT) {
        return ast::empty_expression();
    }

    let right = parse_expression(p, LOWEST);
    if right.is_empty() {
        return ast::Expression::EmptyExpression;
    }

    // If right side is a in expression we have to grab it correctly
    match right {
        Expression::InExpression(tk, in_left, in_right) => {
            return Expression::InExpression(
                tk,
                Box::new(Expression::DotExpression(token, Box::new(left), in_left)),
                in_right,
            );
        }
        _ => {}
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
    let mut right: Expression;

    if p.peek_token_is(token::R_BRACKET) {
        right = Expression::EmptyExpression;
    } else {
        p.next_token();
        right = parse_expression(p, LOWEST);
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

fn parse_as_expression(p: &mut Parser) -> ast::Expression {
    p.debug_print("parse_as_expression");
    let token = p.c_token.to_owned(); // as

    if !p.expect_peek(token::IDENT) {
        return ast::empty_expression();
    }

    let right = parse_expression(p, LOWEST);

    ast::Expression::AsExpression(token, Box::new(right))
}

fn parse_is_expression(p: &mut Parser, left: ast::Expression) -> ast::Expression {
    p.debug_print("parse_is_expression");
    let token = p.c_token.to_owned(); // is

    p.next_token();

    let right = parse_expression(p, LOWEST);

    ast::Expression::IsExpression(token, Box::new(left), Box::new(right))
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

fn parse_double_question_mark_expression(p: &mut Parser, left: ast::Expression) -> ast::Expression {
    p.debug_print("parse_double_question_mark_expression");
    let token = p.c_token.to_owned(); // ??
    p.next_token();
    let right = parse_expression(p, LOWEST);

    ast::Expression::DefaultIfNullExpression(token, Box::new(left), Box::new(right))
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

fn parse_spread_expression(p: &mut Parser) -> ast::Expression {
    p.debug_print("parse_spread_expression");
    let token = p.c_token.to_owned(); // ...

    if !p.expect_peek(token::IDENT) {
        p.add_error(
            format!(
                "Expected a identifer after the ... got {} instead",
                p.peek_token.literal
            )
            .as_str(),
        );
        return ast::empty_expression();
    }
    let ident = parse_expression(p, LOWEST);

    ast::Expression::SpreadExpression(token, Box::new(ident))
}

fn parse_null(p: &mut Parser) -> ast::Expression {
    p.debug_print("parse_null");
    let token = p.c_token.to_owned();

    ast::Expression::NullExpression(token)
}

/// Helper for a none type
fn none_type(tk: token::Token) -> ast::Expression {
    ast::Expression::Type(tk, String::from("none"))
}

/// Helper for a dyn type
fn dyn_type(tk: token::Token) -> ast::Expression {
    ast::Expression::Type(tk, String::from("dyn"))
}