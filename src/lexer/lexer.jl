module Lexer

include("token.jl")

mutable struct Lex
    Input::String
    Position::Int64
    ReadPosition::Int64
    CurrentChar::Char
end

function readchar!(lex::Lex)
    if lex.ReadPosition >= length(lex.Input)
        lex.CurrentChar = ' '
    else
        lex.CurrentChar = lex.Input[lex.ReadPosition]
    end
    lex.Position = lex.ReadPosition
    lex.ReadPosition += 1
end

function nexttoken!(l::Lex)
    # skip the whistepace
    skipwhitespace!(l)

    type::String = ""
    custom_literal = ""
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
    elseif cc == "="
        type = TK.ASSIGN
    elseif cc == ":"
        type = TK.COLON
    elseif cc == "\\"
        type = TK.LOGICAL_LINE_BREAK
    elseif cc == "\n"
        type = TK.LINE_BREAK
    else
        # check Identifiers
        if isletter(l.CurrentChar)
            custom_literal = readidentifier!(l)
            type = TK.lookupIndent(custom_literal)
        elseif isdigit(l.CurrentChar)
            type = TK.INT
            custom_literal = readnumber!(l)
        else
            type = TK.ILLEGAL
        end
    end

    lit = length(custom_literal) > 0 ? custom_literal : cc

    token = TK.newtoken(type, lit)
    readchar!(l)

    return token
end

function readidentifier!(l::Lex)
    position = l.Position
    while isletter(l.CurrentChar)
        readchar!(l)
    end

    return l.Input[position:l.Position - 1]
end

function readnumber!(l::Lex)
    position = l.Position
    while isdigit(l.CurrentChar)
        readchar!(l)
    end
    return l.Input[position:l.Position - 1]
end

function skipwhitespace!(l::Lex)
    while string(l.CurrentChar) == " " || string(l.CurrentChar) == "\t" || string(l.CurrentChar) == "\r"
        readchar!(l)
    end
end
end