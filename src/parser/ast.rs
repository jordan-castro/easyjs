use crate::lexer::token as tk;

pub enum NodeType {
    Statement,
    Expression,
}

#[derive(Clone, Debug)]
pub enum Statement {
    EmptyStatement,                                                 // there was an issue
    VariableStatement(tk::Token, Box<Expression>, Option<Box<Expression>>, Box<Expression>), // variable = expression
    ReturnStatement(tk::Token, Box<Expression>),                    // return expression
    /// use prefix:path
    UseStatement(tk::Token, Box<Expression>, Box<Expression>),
    /// use {container optional(as something)} from prefix:path
    UseFromStatement(tk::Token, Box<Vec<Expression>>, Box<Expression>, Box<Expression>),
    ExpressionStatement(tk::Token, Box<Expression>), // token expression
    BlockStatement(tk::Token, Box<Vec<Statement>>),  // { statements }
    // token identifier = value
    ConstVariableStatement(tk::Token, Box<Expression>, Option<Box<Expression>>, Box<Expression>),
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
    StructStatement(tk::Token, Box<Expression>, Option<Box<Vec<Expression>>>, Option<Box<Vec<Expression>>>, Box<Vec<Statement>>, Box<Vec<Expression>>),

    /// export fn
    /// export struct
    /// export var
    /// export const
    ExportStatement(tk::Token, Box<Statement>),
    
    /// Async block statement
    /// 
    /// async {
    ///   await this()
    ///   await that()
    ///   await thisotherthing()
    /// }
    AsyncBlockStatement(tk::Token, Box<Statement>),

    /// Doc comment statement
    DocCommentStatement(tk::Token, Vec<String>),

    /// Match Statement
    MatchStatement(tk::Token, Box<Expression>, Box<Vec<(Expression, Statement)>>),

    /// A native statement
    NativeStatement(tk::Token, Box<Vec<Statement>>)
}

impl Statement {
    pub fn variant_type(&self) -> String {
        match self {
            Statement::EmptyStatement => "EmptyStatement",
            Statement::VariableStatement(_, _, _, _) => "VariableStatement",
            Statement::ReturnStatement(_, _) => "ReturnStatement",
            Statement::UseStatement(_, _, _) => "UseStatement",
            Statement::UseFromStatement(_, _, _, _) => "UseFromStatement",
            Statement::ExpressionStatement(_, _) => "ExpressionStatement",
            Statement::BlockStatement(_, _) => "BlockStatement",
            Statement::ConstVariableStatement(_, _, _, _) => "ConstVarStatement",
            Statement::ForStatement(_, _, _) => "ForStatement",
            Statement::JavaScriptStatement(_, _) => "JavaScriptStatement",
            Statement::StructStatement(_, _, _, _, _, _) => "StructStatement",
            Statement::ExportStatement(_, _) => "ExportStatement",
            Statement::AsyncBlockStatement(_, _) => "AsyncBlockStatement",
            Statement::DocCommentStatement(_, _) => "DocCommentStatement",
            Statement::MatchStatement(_, _, _) => "MatchStatement",
            Statement::NativeStatement(_, _) => "NativeStatement",
        }.to_string()
    }

    pub fn eq(&self, other: Statement) -> bool {
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
        Option<Box<Expression>>,
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
    AsExpression(tk::Token, Box<Expression>, Box<Expression>),
    // /// Macro ($, ident, arguments, body)
    // MacroExpression(tk::Token, Box<Expression>, Box<Vec<Expression>>),
    // /// Declaring the macro ($, ident, arguments, body as BlockStatment)
    // MacroDecleration(
    //     tk::Token,
    //     Box<Expression>,
    //     Box<Vec<Expression>>,
    //     Box<Statement>,
    // ),
    /// And expression
    AndExpression(
        tk::Token,
        Box<Expression>,
        Box<Expression>,
    ),
    /// Or expression
    OrExpression(
        tk::Token,
        Box<Expression>,
        Box<Expression>,
    ),
    /// Null Expression ?
    NullExpression(tk::Token, Box<Expression>, Box<Expression>),
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
            Expression::AsExpression(_, _, _) => "AsExpression",
            // Expression::MacroExpression(_, _, _) => "MacroExpression",
            // Expression::MacroDecleration(_, _, _, _) => "MacroDecleration",
            Expression::AndExpression(_, _, _) => "AndExpression",
            Expression::OrExpression(_, _, _) => "OrExpression",
            Expression::NullExpression(_, _, _) => "NullExpression",
            Expression::DefaultIfNullExpression(_, _, _) => "DefaultIfNullExpression",
            Expression::NewClassExpression(_, _) => "NewClassExpression",
            Expression::FloatLiteral(_, _) => "FloatLiteral",
            Expression::GroupedExpression(_, _) => "GroupedExpression",
            Expression::IsExpression(_, _, _) => "IsExpression",
            Expression::BuiltinCall(_, _) => "BuiltinCall",
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

// pub fn empty_box_stmt