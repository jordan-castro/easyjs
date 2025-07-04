/// EasyJS compiler token.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    /// The type of token (in string)
    pub typ: String,
    /// The token literal
    pub literal: String,
    /// The file name this token is in.
    pub file_name: String,
    /// The line number of the token.
    pub line_number: i32,
    /// The col number the token starts
    pub col_number: i32,
}

pub const EMPTY_TOKEN: Token = Token {
    typ: String::new(),
    literal: String::new(),
    file_name: String::new(),
    line_number: -1,
    col_number: -1,
};

/// Create a new token on the fly
pub fn new_token(
    typ: &str,
    literal: &str,
    file_name: &str,
    line_number: i32,
    col_number: i32,
) -> Token {
    Token {
        typ: typ.to_owned(),
        literal: literal.to_owned(),
        file_name: file_name.to_owned(),
        line_number,
        col_number,
    }
}

// Types below

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";

// Identifiers
pub const IDENT: &str = "IDENT"; // add, foobar, x, y, ....
pub const INT: &str = "INT"; // 123456 (INT64)
pub const STRING: &str = "STRING";
pub const ARRAY: &str = "ARRAY";
pub const BOOLEAN: &str = "BOOLEAN";
pub const FLOAT: &str = "FLOAT";

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
pub const BITWISE_AND: &str = "&";
pub const BITWISE_OR: &str = "|";
pub const AND_SYMBOL: &str = "&&";
pub const OR_SYMBOL: &str = "||";
pub const QUESTION_MARK: &str = "?";
pub const DOUBLE_QUESTION_MARK: &str = "??";
pub const MODULUS: &str = "%";
pub const PLUS_EQUALS: &str = "+=";
pub const MINUS_EQUALS: &str = "-=";
pub const SLASH_EQUALS: &str = "/=";
pub const ASTERISK_EQUALS: &str = "*=";
pub const SPREAD: &str = "...";

// Delimiters
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";
pub const EOL: &str = "EOL";

// Specials
pub const TYPE_ASSIGNMENT: &str = "TYPE_ASSIGNMENT";

// Types
// pub const INT_32_TYPE: &str = "INT_32_TYPE";
// pub const INT_64_TYPE: &str = "INT_64_TYPE";
// pub const FLOAT_32_TYPE: &str = "FLOAT_32_TYPE";
// pub const FLOAT_64_TYPE: &str = "FLOAT_64_TYPE";
// pub const BOOLEAN_TYPE: &str = "BOOLEAN_TYPE";
// pub const STRING_TYPE: &str = "STRING_TYPE";
// pub const CUSTOM_TYPE: &str = "CUSTOM_TYPE";

// Comments
pub const COMMENT: &str = "//";
pub const DOC_COMMENT: &str = "///";

pub const L_PAREN: &str = "(";
pub const R_PAREN: &str = ")";
pub const L_BRACE: &str = "{";
pub const R_BRACE: &str = "}";
pub const L_BRACKET: &str = "[";
pub const R_BRACKET: &str = "]";

pub const MACRO_SYMBOL: &str = "MACRO_SYMBOL";
// pub const DECORATOR: &str = "DECORATOR";

// Keywords
pub const FUNCTION: &str = "FUNCTION";
// pub const USE: &str = "USE";
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
pub const NOT: &str = "NOT";
// pub const FROM: &str = "FROM";
pub const SELF: &str = "SELF";
pub const NATIVE: &str = "NATIVE";
pub const MACRO: &str = "MACRO";
pub const NEW: &str = "NEW";
pub const IS: &str = "IS";
// pub const VAR: &str = "VAR";
pub const PUB: &str = "PUB";
pub const MATCH: &str = "MATCH";
pub const WITH: &str = "WITH";
pub const IMPORT: &str = "IMPORT";
pub const ENUM: &str = "ENUM";

// Builtin methods
pub const BUILTIN: &str = "BUILTIN";

/// Lookup the ident based on a string
pub fn lookup_ident(ident: &str) -> &'static str {
    match ident {
        "fn" => FUNCTION,
        "struct" => STRUCT,
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
        "not" => NOT,
        "enum" => ENUM,
        // "from" => FROM,
        "self" => SELF,
        "native" => NATIVE,
        "macro" => MACRO,
        "and" => AND_SYMBOL,
        "or" => OR_SYMBOL,
        "new" => NEW,
        "pub" => PUB,
        "is" => IS,
        "import" => IMPORT,
        "match" => MATCH,
        "with" => WITH,
        _ => IDENT, // Default case for unknown identifiers
    }
}

/// Lookup the colon special if any.
pub fn lookup_colon_special(cs: &str) -> &'static str {
    match cs {
        ":=" => &TYPE_ASSIGNMENT,
        _ => &COLON,
    }
}

impl Token {
    /// Get a pretty string rep of a token.
    pub fn pretty_print(&self) {
        println!("{:#?}", self);
    }
}
