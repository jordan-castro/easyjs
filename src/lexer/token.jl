"""
Our Token type object.
"""
struct Token
    Type::String
    Literal::String
end

## TYPES BELOW

const ILLEGAL = "ILLEGAL"
const EOF = "EOF"

# Identifiers
const IDENT = "IDENT" # add, foobar, x, y, ....
const INT = "INT" # 123456 (INT64)

# Operators
const ASSIGN = "="
const PLUS = "+"

# Delimiters
const COMMA = ","
const LINE_BREAK = "\n"
const LOGICAL_LINE_BREAK = "\\" # '\'

const L_PAREN = "("
const R_PAREN = ")"
const L_BRACE = "{"
const R_BRACE = "}"

# Keywords
const FUNCTION = "FUNCTION"
