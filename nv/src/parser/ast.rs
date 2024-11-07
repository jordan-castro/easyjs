use crate::lexer::token as tk;

pub enum NodeType {
    Statement,
    Expression,
}

#[derive(Clone, Debug)]
pub enum Statement {
    EmptyStatement,                                     // there was an issue
    VariableStatement(tk::Token, Box<Expression>, Box<Expression>),      // variable = expression
    ReturnStatement(tk::Token, Box<Expression>),        // return expression
    ImportStatement(tk::Token, String, Option<String>), // import path as something_else
    ExpressionStatement(tk::Token, Box<Expression>),    // token expression
    BlockStatement(tk::Token, Box<Vec<Statement>>),     // { statements }
    // token identifier = value
    ConstVariableStatement(tk::Token, Box<Expression>, Box<Expression>),
    // for condition { body }
    ForStatement(tk::Token, Box<Expression>, Box<Statement>),
    // javascript{}
    JavaScriptStatement(tk::Token, String),
}

impl Statement {
    pub fn variant_type(&self) -> &'static str {
        match self {
            Statement::EmptyStatement => "EmptyStatement",
            Statement::VariableStatement(_,_, _) => "VariableStatement",
            Statement::ReturnStatement(_, _) => "ReturnStatement",
            Statement::ImportStatement(_, _, _) => "ImportStatement",
            Statement::ExpressionStatement(_, _) => "ExpressionStatement",
            Statement::BlockStatement(_, _) => "BlockStatement",
            Statement::ConstVariableStatement(_, _, _) => "ConstVarStatement",
            Statement::ForStatement(_, _, _) => "ForStatement",
            Statement::JavaScriptStatement(_, _) => "JavaScriptStatement"
        }
    }

    pub fn eq(&self, other:Statement) -> bool {
        self.variant_type() == other.variant_type()
    }

    pub fn is_empty(&self) -> bool {
        self.eq(Statement::EmptyStatement)
    }
}

#[derive(Clone, Debug)]
pub enum Expression {
    EmptyExpression,                                      // there was an issue
    Identifier(tk::Token, String),                        // token value
    PrefixExpression(tk::Token, String, Box<Expression>), // token operator expression
    IntegerLiteral(tk::Token, i64),                       // token value(i64)
    StringLiteral(tk::Token, String),                     // token value(String)
    CommentExpression(tk::Token, String),                 // token value(String)
    InfixExpression(tk::Token, Box<Expression>, String, Box<Expression>), // token left operator right
    Boolean(tk::Token, bool),                                             // token <- boolean
    // if (condition) { block statement } |elseif (condition) {  }|else {}|
    IfExpression(
        tk::Token,
        Box<Expression>,
        Box<Statement>,
        Box<Expression>,
        Box<Statement>,
    ),
    // async expression
    AsyncExpression(tk::Token, Box<Expression>),
    // await expression
    AwaitExpression(tk::Token, Box<Expression>),
    // fn function_name paramaters {}
    FunctionLiteral(
        tk::Token,
        Box<Expression>,
        Box<Vec<Expression>>,
        Box<Statement>,
    ),
    // fn(params) {statement}
    LambdaLiteral(tk::Token, Box<Vec<Expression>>, Box<Statement>),
    // ( caller params
    CallExpression(tk::Token, Box<Expression>, Box<Vec<Expression>>),
    // left in right
    InExpression(tk::Token, Box<Expression>, Box<Expression>),
    // left of right
    OfExpression(tk::Token, Box<Expression>, Box<Expression>),
    // i64(left)..i64(right)
    RangeExpression(tk::Token, Box<Expression>, Box<Expression>),
    // left.right
    DotExpression(tk::Token, Box<Expression>, Box<Expression>),
    // left.if {}
    DotIfExpression(tk::Token, Box<Expression>, Box<Statement>),
    // javascript{}
    JavaScriptExpression(tk::Token, String),
    // []
    ArrayLiteral(tk::Token, Box<Vec<Expression>>),
    // [i]
    IndexExpression(tk::Token, Box<Expression>, Box<Expression>, Box<Expression>),
    // {}
    ObjectLiteral(tk::Token, Vec<Vec<Box<Expression>>>),
}

impl Expression {
    // Returns a unique identifier for each variant
    pub fn variant_type(&self) -> &'static str {
        match self {
            Expression::EmptyExpression => "EmptyExpression",
            Expression::Identifier(_, _) => "Identifier",
            Expression::PrefixExpression(_, _, _) => "PrefixExpression",
            Expression::IntegerLiteral(_, _) => "IntegerLiteral",
            Expression::StringLiteral(_, _) => "StringLiteral",
            Expression::CommentExpression(_, _) => "CommentExpression",
            Expression::InfixExpression(_, _, _, _) => "InfixExpression",
            Expression::Boolean(_, _) => "Boolean",
            Expression::IfExpression(_, _, _, _, _) => "IfExpression",
            Expression::AsyncExpression(_, _) => "AsyncExpression",
            Expression::AwaitExpression(_, _) => "AwaitExpression",
            Expression::FunctionLiteral(_, _, _, _) => "FunctionLiteral",
            Expression::LambdaLiteral(_, _, _) => "LambdaLiteral",
            Expression::CallExpression(_, _, _) => "CallExpression",
            Expression::InExpression(_, _, _) => "InExpression",
            Expression::OfExpression(_, _, _) => "OfExpression",
            Expression::RangeExpression(_, _, _) => "RangeExpression",
            Expression::DotExpression(_, _, _) => "DotExpression",
            Expression::DotIfExpression(_, _, _) => "DotIfExpression",
            Expression::JavaScriptExpression(_, _) => "JavaScriptExpression",
            Expression::ArrayLiteral(_, _) => "ArrayLiteral",
            Expression::IndexExpression(_, _, _, _) => "IndexExpression",
            Expression::ObjectLiteral(_, _) => "ObjectLiteral",
        }
    }

    pub fn eq(&self, other:Expression) -> bool {
        self.variant_type() == other.variant_type()
    }

    pub fn is_empty(&self) -> bool {
        self.eq(Expression::EmptyExpression)
    }
}

pub struct Program {
    pub statements: Vec<Statement>,
}

pub fn empty_expression() -> Expression {
    Expression::EmptyExpression
}

pub fn empty_statement() -> Statement {
    Statement::EmptyStatement
}