module Lexer

include("token.jl")

"""
Our Lexer structure.

`Input` : Is the code for this specific Lexer.

`Position` : The current position of the Lexer in Input.

`ReadPosition` : The current ReadPosition i.e. one after where the Position is at.

`CurrentChar` : The current char being read by Input[ReadPostion]
"""
mutable struct Lex
    Input::String
    Position::Int64
    ReadPosition::Int64
    CurrentChar::Char
end

"""
Read the current Input[ReadPosition] character.

Will update `Position` and `ReadPosition`.
"""
function readchar!(lex::Lex)
    if lex.ReadPosition >= length(lex.Input)
        lex.CurrentChar = ' '
    else
        lex.CurrentChar = lex.Input[lex.ReadPosition]
    end
    lex.Position = lex.ReadPosition
    lex.ReadPosition += 1
end

"""
Get the next token from the Input.
"""
function nexttoken!(l::Lex)
    # skip the whistepace
    skipwhitespace!(l)

    # to not have to instance a TK.Token struct every condition
    type::String = ""
    # Sometimes we need to use a literal that is greater than just the current char
    custom_literal = ""
    # parse to string.
    cc = string(l.CurrentChar)
    if cc == "="
        type = TK.ASSIGN
    elseif cc == "+"
        type = TK.PLUS
    elseif cc == "{"
        type = TK.L_BRACE
    elseif cc == "}"
        type = TK.R_BRACE
    elseif cc == "("
        type = TK.L_PAREN
    elseif cc == ")"
        type = TK.R_PAREN
    elseif cc == ":"
        # check the next char
        next_char = l.Input[l.ReadPosition]
        type = TK.lookupColonSpecial(string(cc, next_char))
        
        # if the type is not a COLON, we have a special colon.
        if type != TK.COLON
            custom_literal = cc * next_char
            # go to next char after special
            readchar!(l)
        end
    elseif cc == "\\"
        type = TK.LOGICAL_LINE_BREAK
    elseif cc == "\n"
        type = TK.LINE_BREAK
    else
        # check Identifiers
        if isletter(l.CurrentChar)
            # read the identifier
            custom_literal = readidentifier!(l)
            # get the currect type
            type = TK.lookupIndent(custom_literal)
        elseif isdigit(l.CurrentChar)
            type = TK.INT
            # read the number
            custom_literal = readnumber!(l)
        else
            type = TK.ILLEGAL
        end
    end

    # set the correct literal
    lit = length(custom_literal) > 0 ? custom_literal : cc

    token = TK.newtoken(type, lit)
    # move next.
    readchar!(l)

    return token
end

"""
Get the full identifier.
"""
function readidentifier!(l::Lex)
    position = l.Position
    while isletter(l.CurrentChar)
        readchar!(l)
    end

    # Set read position . We do this because arrays in julia are 1 based.
    l.ReadPosition = l.Position
    return l.Input[position:l.Position-1]
end

"""
Get the full number
"""
function readnumber!(l::Lex)
    position = l.Position
    while isdigit(l.CurrentChar)
        readchar!(l)
    end
    # Set the read position. We do this because arrays in julia are 1 based.
    l.ReadPosition = l.Position
    return l.Input[position:l.Position-1]
end

"""
We don't care about whitespace. Except for \n
"""
function skipwhitespace!(l::Lex)
    while string(l.CurrentChar) == " " || string(l.CurrentChar) == "\t" || string(l.CurrentChar) == "\r"
        readchar!(l)
    end
end
end