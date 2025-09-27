use crate::lexer::token as tk;

pub enum NodeType {
    Statement,
    Expression,
}

#[derive(Clone, Debug)]
pub enum Statement {
    EmptyStatement, // there was an issue
    VariableStatement(
        tk::Token,
        Box<Expression>,
        Option<Box<Expression>>,
        Box<Expression>,
        bool,
    ), // variable = expression (bool = should_infer)
    ReturnStatement(tk::Token, Box<Expression>), // return expression
    ImportStatement(tk::Token, String, Option<Box<Expression>>), // import 'path.ej' (or) import 'path'
    ExpressionStatement(tk::Token, Box<Expression>), // token expression
    BlockStatement(tk::Token, Box<Vec<Statement>>), // { statements }
    // token identifier = value
    // ConstVariableStatement(tk::Token, Box<Expression>, Option<Box<Expression>>, Box<Expression>, bool),
    // for condition { body }
    ForStatement(tk::Token, Box<Expression>, Box<Statement>),
    // javascript{}
    JavaScriptStatement(tk::Token, String),
    /// ```easyjs
    /// struct Person[name,age] with GreetMixin, FarewellMixin {
    ///     MAX_AGE = 150 // static variables
    ///
    ///     fn greet(self) { // methods
    ///     }
    ///
    ///     fn ask_question(question) { // static methods
    ///     }
    /// }
    /// ```
    StructStatement(
        tk::Token,
        Box<Expression>,
        Option<Box<Vec<Expression>>>,
        Option<Box<Vec<Expression>>>,
        Box<Vec<Statement>>,
        Box<Vec<Expression>>,
    ),

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

    /// A native statement
    NativeStatement(tk::Token, Box<Vec<Statement>>),

    /// A enum statement
    EnumStatement(tk::Token, String, Box<Vec<Expression>>),

    /// A break statement
    BreakStatement(tk::Token),

    /// A continue statement
    ContinueStatement(tk::Token),

    /// Declaring the macro (@, ident, arguments, body as BlockStatment)
    MacroStatement(
        tk::Token,
        Box<Expression>,
        Box<Vec<Expression>>,
        Box<Statement>,
    ),
}

impl Statement {
    /// Get the token of the `Statement`.
    pub fn get_token(&self) -> &tk::Token {
        match self {
            Statement::EmptyStatement => {
                // No token stored here, so panic or handle as needed
                panic!("EmptyStatement has no token")
            }
            Statement::VariableStatement(token, _, _, _, _) => token,
            Statement::ReturnStatement(token, _) => token,
            Statement::ImportStatement(token, _, _) => token,
            Statement::ExpressionStatement(token, _) => token,
            Statement::BlockStatement(token, _) => token,
            Statement::ForStatement(token, _, _) => token,
            Statement::JavaScriptStatement(token, _) => token,
            Statement::StructStatement(token, _, _, _, _, _) => token,
            Statement::ExportStatement(token, _) => token,
            Statement::AsyncBlockStatement(token, _) => token,
            Statement::MatchStatement(token, _, _) => token,
            Statement::NativeStatement(token, _) => token,
            Statement::EnumStatement(token, _, _) => token,
            Statement::BreakStatement(token) => token,
            Statement::ContinueStatement(token) => token,
            Statement::MacroStatement(token, _, _, _) => token,
        }
    }

    pub fn variant_type(&self) -> String {
        match self {
            Statement::EmptyStatement => "EmptyStatement",
            Statement::VariableStatement(_, _, _, _, _) => "VariableStatement",
            Statement::ReturnStatement(_, _) => "ReturnStatement",
            Statement::ExpressionStatement(_, _) => "ExpressionStatement",
            Statement::ImportStatement(_, _, _) => "ImportStatement",
            Statement::BlockStatement(_, _) => "BlockStatement",
            Statement::ForStatement(_, _, _) => "ForStatement",
            Statement::JavaScriptStatement(_, _) => "JavaScriptStatement",
            Statement::StructStatement(_, _, _, _, _, _) => "StructStatement",
            Statement::ExportStatement(_, _) => "ExportStatement",
            Statement::AsyncBlockStatement(_, _) => "AsyncBlockStatement",
            Statement::MatchStatement(_, _, _) => "MatchStatement",
            Statement::NativeStatement(_, _) => "NativeStatement",
            Statement::EnumStatement(_, _, _) => "EnumStatement",
            Statement::BreakStatement(_) => "BreakStatement",
            Statement::ContinueStatement(_) => "ContinueStatement",
            Statement::MacroStatement(_, _, _, _) => "MacroDecleration",
        }
        .to_string()
    }

    pub fn eq(&self, other: Statement) -> bool {
        self.variant_type() == other.variant_type()
    }

    pub fn is_empty(&self) -> bool {
        self.eq(Statement::EmptyStatement)
    }

    pub fn is_native(&self) -> bool {
        self.variant_type() == "NativeStatement"
    }

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
    // fn function_name paramaters {}
    FunctionLiteral(
        tk::Token,
        Box<Expression>,
        Box<Vec<Expression>>,
        Option<Box<Expression>>,
        Box<Statement>,
    ),
    // fn(params) {statement} OR fn(params) stmt
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
    // []
    ArrayLiteral(tk::Token, Box<Vec<Expression>>),
    // [i]
    IndexExpression(tk::Token, Box<Expression>, Box<Expression>),
    // {}
    ObjectLiteral(tk::Token, Vec<Vec<Box<Expression>>>),
    /// = something else
    AssignExpression(tk::Token, Box<Expression>, Box<Expression>),
    /// not expression
    NotExpression(tk::Token, Box<Expression>),
    /// left as right
    AsExpression(tk::Token, Box<Expression>),
    /// Macro (@, ident, arguments, body)
    MacroExpression(tk::Token, Box<Expression>, Box<Vec<Expression>>),
    /// And expression
    AndExpression(tk::Token, Box<Expression>, Box<Expression>),
    /// Or expression
    OrExpression(tk::Token, Box<Expression>, Box<Expression>),
    /// Null Expression ?
    NullExpression(tk::Token),
    /// Default if null exp ??
    DefaultIfNullExpression(tk::Token, Box<Expression>, Box<Expression>),
    /// new Class
    NewClassExpression(tk::Token, Box<Expression>),
    /// Float literal 0.0
    FloatLiteral(tk::Token, f64),
    /// Grouped Expression ()
    GroupedExpression(tk::Token, Box<Expression>),
    /// left is right (typeof(left) == right)
    IsExpression(tk::Token, Box<Expression>, Box<Expression>),
    /// Builtin function call
    BuiltinCall(tk::Token, Box<Vec<Expression>>),
    /// Identifier with type
    IdentifierWithType(tk::Token, String, Box<Expression>),
    /// Type expression
    Type(tk::Token, String),
    /// IIFE
    ///
    /// var a = fn { return 1 } // a = 1
    IIFE(tk::Token, Box<Statement>),
    /// ...variable
    SpreadExpression(tk::Token, Box<Expression>),
    /// Doc comment '///'
    DocCommentExpression(tk::Token, Vec<String>),
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
            Expression::FunctionLiteral(token, _, _, _, _) => token,
            Expression::LambdaLiteral(token, _, _) => token,
            Expression::CallExpression(token, _, _) => token,
            Expression::InExpression(token, _, _) => token,
            Expression::OfExpression(token, _, _) => token,
            Expression::RangeExpression(token, _, _) => token,
            Expression::DotExpression(token, _, _) => token,
            Expression::DotIfExpression(token, _, _) => token,
            Expression::ArrayLiteral(token, _) => token,
            Expression::IndexExpression(token, _, _) => token,
            Expression::ObjectLiteral(token, _) => token,
            Expression::AssignExpression(token, _, _) => token,
            Expression::NotExpression(token, _) => token,
            Expression::AsExpression(token, _) => token,
            Expression::MacroExpression(token, _, _) => token,
            Expression::AndExpression(token, _, _) => token,
            Expression::OrExpression(token, _, _) => token,
            Expression::NullExpression(token) => token,
            Expression::DefaultIfNullExpression(token, _, _) => token,
            Expression::NewClassExpression(token, _) => token,
            Expression::FloatLiteral(token, _) => token,
            Expression::GroupedExpression(token, _) => token,
            Expression::IsExpression(token, _, _) => token,
            Expression::BuiltinCall(token, _) => token,
            Expression::IdentifierWithType(token, _, _) => token,
            Expression::Type(token, _) => token,
            Expression::IIFE(token, _) => token,
            Expression::SpreadExpression(token, _) => token,
            Expression::DocCommentExpression(token, _) => token
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
            Expression::FunctionLiteral(_, _, _, _, _) => "FunctionLiteral",
            Expression::LambdaLiteral(_, _, _) => "LambdaLiteral",
            Expression::CallExpression(_, _, _) => "CallExpression",
            Expression::InExpression(_, _, _) => "InExpression",
            Expression::OfExpression(_, _, _) => "OfExpression",
            Expression::RangeExpression(_, _, _) => "RangeExpression",
            Expression::DotExpression(_, _, _) => "DotExpression",
            Expression::DotIfExpression(_, _, _) => "DotIfExpression",
            Expression::ArrayLiteral(_, _) => "ArrayLiteral",
            Expression::IndexExpression(_, _, _) => "IndexExpression",
            Expression::ObjectLiteral(_, _) => "ObjectLiteral",
            Expression::AssignExpression(_, _, _) => "AssignExpression",
            Expression::NotExpression(_, _) => "NotExpression",
            Expression::AsExpression(_, _) => "AsExpression",
            Expression::MacroExpression(_, _, _) => "MacroExpression",
            Expression::AndExpression(_, _, _) => "AndExpression",
            Expression::OrExpression(_, _, _) => "OrExpression",
            Expression::NullExpression(_) => "NullExpression",
            Expression::DefaultIfNullExpression(_, _, _) => "DefaultIfNullExpression",
            Expression::NewClassExpression(_, _) => "NewClassExpression",
            Expression::FloatLiteral(_, _) => "FloatLiteral",
            Expression::GroupedExpression(_, _) => "GroupedExpression",
            Expression::IsExpression(_, _, _) => "IsExpression",
            Expression::BuiltinCall(_, _) => "BuiltinCall",
            Expression::IdentifierWithType(_, _, _) => "IdentifierWithType",
            Expression::Type(_, _) => "Type",
            Expression::IIFE(_, _) => "IIFE",
            Expression::SpreadExpression(_, _) => "SpreadExpression",
            Expression::DocCommentExpression(_, _) => "DocCommentExpression"
        }
    }

    pub fn eq(&self, other: Expression) -> bool {
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

pub fn empty_box_exp() -> Box<Expression> {
    Box::new(empty_expression())
}
