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

const L_PAREN = "("
const R_PAREN = ")"
const L_BRACE = "{"
const R_BRACE = "}"

# Keywords
const FUNCTION = "FUNCTION"
const IMPORT = "IMPORT"
const STRUCT = "STRUCT"


## specials
const specials = Dict(
    "\n" => LINE_BREAK,
    "\\" => LOGICAL_LINE_BREAK
)

## Keywords map
const keywords = Dict(
    "fn" => FUNCTION,
    "struct" => STRUCT,
    "import" => IMPORT
)

function lookupIndent(ident::String)
    for keyword in keywords
        if keyword.first == ident
            return keyword.second
        end
    end
    return IDENT
end

function lookupSpecial(speci::String)
    for special in specials
        if special.first == speci
            return special.second
        end
    end
end

end