module TK

"""
Our Token type object.
"""
struct Token
    Type::String
    Literal::String
end

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


## specials
const colon_specials = Dict(
    "::" => TYPE,
    ":=" => CONST_ASSIGNMENT
)

## Keywords map
const keywords = Dict(
    "fn" => FUNCTION,
    "struct" => STRUCT,
    "import" => IMPORT
)

function lookupIndent(ident::String)
    return get(keywords, ident, IDENT)
end

function lookupColonSpecial(speci::String)
    return get(colon_specials, speci, COLON)
end

end