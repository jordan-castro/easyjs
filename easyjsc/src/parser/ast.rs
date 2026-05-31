use crate::lexer::token as tk;

pub enum NodeType {
    Statement,
    Expression,
}

#[derive(Clone, Debug)]
pub enum Statement {
    EmptyStatement, // there was an issue
    /// name : string = "Jordan"
    VariableStatement(
        tk::Token,
        Box<Expression>, // ident
        Box<Expression>, // type
        Box<Expression> // value
    ),
    /// name := "Jordan"
    ConstVariableStatement(
        tk::Token,
        Box<Expression>, // ident
        Box<Expression>, // expected type
        Box<Expression>  // value
    ),
    ReturnStatement(tk::Token, Box<Expression>), // return expression
    // ImportStatement(tk::Token, String, Option<Expression>), // import 'path.ej' as alias (or) import 'path' as alias
    ExpressionStatement(tk::Token, Box<Expression>), // token expression
    BlockStatement(tk::Token, Box<Vec<Statement>>), // { statements }

    /// for condition { body }
    ForStatement(tk::Token, Box<Expression>, Box<Statement>),
    /// javascript{}
    JavaScriptStatement(tk::Token, String),

    /// pub fn
    /// pub struct
    /// pub var
    /// pub const
    ExportStatement(tk::Token, Box<Statement>),

    /// Async block statement
    ///
    /// async {
    ///   await this()
    ///   await that()
    ///   await thisotherthing()
    /// }
    AsyncBlockStatement(tk::Token, Box<Statement>),

    /// Match Statement
    MatchStatement(
        tk::Token,
        Box<Expression>,
        Box<Vec<(Expression, Statement)>>,
    ),
    /// A break statement
    BreakStatement(tk::Token),

    /// A continue statement
    ContinueStatement(tk::Token),

    /// @annoatation
    AnnotationStatement(tk::Token, String, Box<Statement>)
}

impl Statement {
    /// Get the token of the `Statement`.
    pub fn get_token(&self) -> &tk::Token {
        match self {
            Statement::EmptyStatement => {
                // No token stored here, so panic or handle as needed
                panic!("EmptyStatement has no token")
            }
            Statement::VariableStatement(token, _, _, _) => token,
            Statement::ReturnStatement(token, _) => token,
            Statement::ExpressionStatement(token, _) => token,
            Statement::BlockStatement(token, _) => token,
            Statement::ForStatement(token, _, _) => token,
            Statement::JavaScriptStatement(token, _) => token,
            Statement::ExportStatement(token, _) => token,
            Statement::AsyncBlockStatement(token, _) => token,
            Statement::MatchStatement(token, _, _) => token,
            Statement::BreakStatement(token) => token,
            Statement::ContinueStatement(token) => token,
            Statement::AnnotationStatement(token, _, _) => token,
            Statement::ConstVariableStatement(token, _, _, _) => token
        }
    }

    pub fn variant_type(&self) -> String {
        match self {
            Statement::EmptyStatement => "EmptyStatement",
            Statement::VariableStatement(_, _, _, _) => "VariableStatement",
            Statement::ReturnStatement(_, _) => "ReturnStatement",
            Statement::ExpressionStatement(_, _) => "ExpressionStatement",
            Statement::BlockStatement(_, _) => "BlockStatement",
            Statement::ForStatement(_, _, _) => "ForStatement",
            Statement::JavaScriptStatement(_, _) => "JavaScriptStatement",
            Statement::ExportStatement(_, _) => "ExportStatement",
            Statement::AsyncBlockStatement(_, _) => "AsyncBlockStatement",
            Statement::MatchStatement(_, _, _) => "MatchStatement",
            Statement::BreakStatement(_) => "BreakStatement",
            Statement::ContinueStatement(_) => "ContinueStatement",
            Statement::ConstVariableStatement(_, _, _, _) => "ConstVariableStatement",
            Statement::AnnotationStatement(_, _, _) => "AnnotationStatement"
        }
        .to_string()
    }

    pub fn eq(&self, other: Statement) -> bool {
        self.variant_type() == other.variant_type()
    }

    pub fn is_empty(&self) -> bool {
        self.eq(Statement::EmptyStatement)
    }

    // pub fn is_native(&self) -> bool {
        // self.variant_type() == "NativeStatement"
    // }

    /// Get the final stmt of a Block.
    ///
    /// If not being called on a block, it will return the current stmt.
    pub fn get_final_stmt(&self) -> &Statement {
        match self {
            Statement::BlockStatement(token, statements) => {
                statements.last().unwrap().get_final_stmt()
            }
            _ => self,
        }
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
    /// Function expression (toke)
    Function(
        /// Main token
        tk::Token,
        /// The paramaters
        Box<Vec<Expression>>,
        /// The return type.
        Box<Expression>,
        /// The body
        Statement
    ),
    // name()params
    CallExpression(tk::Token, Box<Expression>, Box<Vec<Expression>>),
    // left in right
    InExpression(tk::Token, Box<Expression>, Box<Expression>),
    // i64(left)..i64(right)
    RangeExpression(tk::Token, Box<Expression>, Box<Expression>),
    // left.right
    DotExpression(tk::Token, Box<Expression>, Box<Expression>),
    // // left.if {}
    // DotIfExpression(tk::Token, Box<Expression>, Box<Statement>),
    // []
    ArrayLiteral(tk::Token, Box<Vec<Expression>>),
    // [i]
    IndexExpression(tk::Token, Box<Expression>, Box<Expression>),
    // // {}
    // ObjectLiteral(tk::Token, Vec<Vec<Box<Expression>>>),
    /// = something else
    AssignExpression(tk::Token, Box<Expression>, Box<Expression>),
    /// not expression
    NotExpression(tk::Token, Box<Expression>),
    /// left as right
    AsExpression(tk::Token, Box<Expression>),
    /// And expression
    AndExpression(tk::Token, Box<Expression>, Box<Expression>),
    /// Or expression
    OrExpression(tk::Token, Box<Expression>, Box<Expression>),
    /// Null Expression ?
    NullExpression(tk::Token),
    /// Default if null exp ??
    DefaultIfNullExpression(tk::Token, Box<Expression>, Box<Expression>),
    // /// new Class
    // NewClassExpression(tk::Token, Box<Expression>),
    /// Float literal 0.0
    FloatLiteral(tk::Token, f64),
    /// Grouped Expression ()
    GroupedExpression(tk::Token, Box<Expression>),
    /// left is right (typeof(left) == right)
    IsExpression(tk::Token, Box<Expression>, Box<Expression>),
    // /// Builtin function call
    // BuiltinCall(tk::Token, Box<Vec<Expression>>),
    /// Identifier with type
    IdentifierWithType(tk::Token, String, Box<Expression>),
    /// Type expression
    Type(tk::Token, String),
    /// IIFE
    ///
    /// a = fn { return 1 } // a = 1
    IIFE(tk::Token, Box<Statement>),
    /// ...variable
    SpreadExpression(tk::Token, Box<Expression>),
    /// Doc comment '///'
    DocCommentExpression(tk::Token, Vec<String>),
    /// a = class { stmts go here }
    Class(tk::Token, Vec<Statement>),
    /// a = import('module/path')
    Import(tk::Token, Box<Expression>)
}

impl Expression {
    /// Get the token of the `Expression`
    pub fn get_token(&self) -> &tk::Token {
        match self {
            Expression::EmptyExpression => {
                // No token stored? Return a reference to a dummy or panic
                // But since all variants have token except EmptyExpression, you could panic
                panic!("EmptyExpression has no token")
            }
            Expression::Identifier(token, _) => token,
            Expression::PrefixExpression(token, _, _) => token,
            Expression::IntegerLiteral(token, _) => token,
            Expression::StringLiteral(token, _) => token,
            Expression::CommentExpression(token, _) => token,
            Expression::InfixExpression(token, _, _, _) => token,
            Expression::Boolean(token, _) => token,
            Expression::IfExpression(token, _, _, _, _) => token,
            Expression::AsyncExpression(token, _) => token,
            Expression::AwaitExpression(token, _) => token,
            Expression::CallExpression(token, _, _) => token,
            Expression::InExpression(token, _, _) => token,
            Expression::RangeExpression(token, _, _) => token,
            Expression::DotExpression(token, _, _) => token,
            Expression::ArrayLiteral(token, _) => token,
            Expression::IndexExpression(token, _, _) => token,
            Expression::AssignExpression(token, _, _) => token,
            Expression::NotExpression(token, _) => token,
            Expression::AsExpression(token, _) => token,
            Expression::AndExpression(token, _, _) => token,
            Expression::OrExpression(token, _, _) => token,
            Expression::NullExpression(token) => token,
            Expression::DefaultIfNullExpression(token, _, _) => token,
            Expression::FloatLiteral(token, _) => token,
            Expression::GroupedExpression(token, _) => token,
            Expression::IsExpression(token, _, _) => token,
            Expression::IdentifierWithType(token, _, _) => token,
            Expression::Type(token, _) => token,
            Expression::IIFE(token, _) => token,
            Expression::SpreadExpression(token, _) => token,
            Expression::DocCommentExpression(token, _) => token,
            Expression::Function(token, _, _, _) => token,
            Expression::Class(token, _) => token,
            Expression::Import(token, _) => token,
        }
    }

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
            Expression::CallExpression(_, _, _) => "CallExpression",
            Expression::InExpression(_, _, _) => "InExpression",
            Expression::RangeExpression(_, _, _) => "RangeExpression",
            Expression::DotExpression(_, _, _) => "DotExpression",
            Expression::ArrayLiteral(_, _) => "ArrayLiteral",
            Expression::IndexExpression(_, _, _) => "IndexExpression",
            Expression::AssignExpression(_, _, _) => "AssignExpression",
            Expression::NotExpression(_, _) => "NotExpression",
            Expression::AsExpression(_, _) => "AsExpression",
            Expression::AndExpression(_, _, _) => "AndExpression",
            Expression::OrExpression(_, _, _) => "OrExpression",
            Expression::NullExpression(_) => "NullExpression",
            Expression::DefaultIfNullExpression(_, _, _) => "DefaultIfNullExpression",
            Expression::FloatLiteral(_, _) => "FloatLiteral",
            Expression::GroupedExpression(_, _) => "GroupedExpression",
            Expression::IsExpression(_, _, _) => "IsExpression",
            Expression::IdentifierWithType(_, _, _) => "IdentifierWithType",
            Expression::Type(_, _) => "Type",
            Expression::IIFE(_, _) => "IIFE",
            Expression::SpreadExpression(_, _) => "SpreadExpression",
            Expression::DocCommentExpression(_, _) => "DocCommentExpression",
            Expression::Function(_, _, _, _) => "Function",
            Expression::Class(_, _) => "Class",
            Expression::Import(_, _) => "Import"
        }
    }

    pub fn eq(&self, other: Expression) -> bool {
        self.variant_type() == other.variant_type()
    }

    pub fn is_empty(&self) -> bool {
        self.eq(Expression::EmptyExpression)
    }
}

#[derive(Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

pub fn empty_expression() -> Expression {
    Expression::EmptyExpression
}

pub fn empty_statement() -> Statement {
    Statement::EmptyStatement
}

pub fn empty_box_exp() -> Box<Expression> {
    Box::new(empty_expression())
}
