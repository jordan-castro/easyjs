
"""
Our Token type object.
"""
struct Token
    Type::String
    Literal::String
end

"""
Create a new token on the fly.
"""
function newtoken(type::String, literal::String):Token
    return Token(type, literal)
end

## TYPES BELOW

const ILLEGAL = "ILLEGAL"
const EOF = "EOF"

# Identifiers
const IDENT = "IDENT" # add, foobar, x, y, ....
const INT = "INT" # 123456 (INT64)
const STRING = "STRING"
const ARRAY = "ARRAY"
const BOOLEAN = "BOOLEAN"

# Operators
const ASSIGN = "="
const COLON = ":"
const PLUS = "+"
const MINUS = "-"
const BANG = "!"
const ASTERISK = "*"
const SLASH = "/"
const LT = "<"
const GT = ">"
const EQ = "=="
const NOT_EQ = "!="
const GT_OR_EQ = ">="
const LT_OR_EQ = "<="

# Delimiters
const COMMA = ","
const LINE_BREAK = "\n"
const LOGICAL_LINE_BREAK = "\\" # '\'

# Specials
const TYPE = "TYPE"
const CONST_ASSIGNMENT = "CONST_ASSIGNMENT"

const L_PAREN = "("
const R_PAREN = ")"
const L_BRACE = "{"
const R_BRACE = "}"

# Keywords
const FUNCTION = "FUNCTION"
const IMPORT = "IMPORT"
const STRUCT = "STRUCT"
const TRUE = "TRUE"
const FALSE = "FALSE"
const IF = "IF"
const ELSE = "ELSE"
const ELSEIF = "ELSEIF"
const RETURN = "RETURN"


## specials
const colon_specials = Dict(
    "::" => TYPE,
    ":=" => CONST_ASSIGNMENT
)

## Keywords map
const keywords = Dict(
    "fn" => FUNCTION,
    "struct" => STRUCT,
    "import" => IMPORT,
    "true" => TRUE,
    "false" => FALSE,
    "if" => IF,
    "else" => ELSE,
    "elseif" => ELSEIF,
    "return" => RETURN
)

"""
Lookup helper for the identifiers.
"""
function lookupIndent(ident::String)
    return get(keywords, ident, IDENT)
end

"""
Lookup helper for the colon specials.
"""
function lookupColonSpecial(speci::String)
    return get(colon_specials, speci, COLON)
end

function pretty(token::Token)
    println("Type: " * token.Type * " Literal: " * token.Literal)
end