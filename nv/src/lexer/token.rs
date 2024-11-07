/// EasyJS compiler token.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub typ:String,
    pub literal:String
}

pub const EMPTY_TOKEN:Token = Token{typ: String::new(), literal: String::new()};

/// Create a new token on the fly
pub fn new_token(typ:&str, literal:&str) -> Token {
    Token{
        typ: typ.to_owned(),
        literal: literal.to_owned()
    }
}

pub fn new_token_from_String(typ:String, literal:String) -> Token {
    Token{typ, literal}
}

// Types below

pub const ILLEGAL:&str = "ILLEGAL";
pub const EOF:&str = "EOF";

// Identifiers
pub const IDENT: &str = "IDENT"; // add, foobar, x, y, ....
pub const INT: &str = "INT"; // 123456 (INT64)
pub const STRING: &str = "STRING";
pub const ARRAY: &str = "ARRAY";
pub const BOOLEAN: &str = "BOOLEAN";
pub const BUILTIN: &str = "BUILTIN"; // our STD Library functions.

// Operators
pub const ASSIGN: &str = "=";
pub const COLON: &str = ":";
pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const BANG: &str = "!";
pub const ASTERISK: &str = "*";
pub const SLASH: &str = "/";
pub const LT: &str = "<";
pub const GT: &str = ">";
pub const EQ: &str = "==";
pub const NOT_EQ: &str = "!=";
pub const GT_OR_EQ: &str = ">=";
pub const LT_OR_EQ: &str = "<=";
pub const DOT: &str = ".";
pub const DOTDOT: &str = "..";

// Delimiters
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";
pub const EOL: &str = "\n";

// Specials
pub const TYPE: &str = "TYPE";
pub const CONST_ASSIGNMENT: &str = "CONST_ASSIGNMENT";

// Comments
pub const COMMENT: &str = "//";

pub const L_PAREN: &str = "(";
pub const R_PAREN: &str = ")";
pub const L_BRACE: &str = "{";
pub const R_BRACE: &str = "}";
pub const L_BRACKET: &str = "[";
pub const R_BRACKET: &str = "]";

// Keywords
pub const FUNCTION: &str = "FUNCTION";
pub const IMPORT: &str = "IMPORT";
pub const STRUCT: &str = "STRUCT";
pub const TRUE: &str = "TRUE";
pub const FALSE: &str = "FALSE";
pub const IF: &str = "IF";
pub const ELSE: &str = "ELSE";
pub const ELIF: &str = "ELIF";
pub const RETURN: &str = "RETURN";
pub const AS: &str = "AS";
pub const JAVASCRIPT: &str = "JAVASCRIPT";
pub const FOR: &str = "FOR";
pub const IN: &str = "IN";
pub const OF: &str = "OF";
pub const ASYNC: &str = "ASYNC";
pub const AWAIT: &str = "AWAIT";

/// Lookup the ident based on a string
pub fn lookup_ident(ident: &str) -> &'static str {
    match ident {
        "fn" => FUNCTION,
        "struct" => STRUCT,
        "import" => IMPORT,
        "true" => TRUE,
        "false" => FALSE,
        "if" => IF,
        "else" => ELSE,
        "elif" => ELIF,
        "return" => RETURN,
        "as" => AS,
        "javascript" => JAVASCRIPT,
        "in" => IN,
        "for" => FOR,
        "of" => OF,
        "async" => ASYNC,
        "await" => AWAIT,
        _ => IDENT, // Default case for unknown identifiers
    }
}

/// Lookup the colon special if any.
pub fn lookup_colon_special(cs:&str) -> &'static str {
    match cs {
        "::" => &TYPE,
        ":=" => &CONST_ASSIGNMENT,
        _ => &COLON
    }
}

impl Token {
    /// Get a pretty string rep of a token.
    pub fn pretty(self) -> String {
        let mut msg: String = "Type: ".to_owned();
        msg.push_str(&self.typ);
        msg.push_str(" Literal: ");
        msg.push_str(&self.literal);

        msg
    }
}