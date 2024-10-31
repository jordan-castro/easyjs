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
    if lex.ReadPosition > length(lex.Input)
        lex.CurrentChar = '\0'
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

    if l.CurrentChar == '\0'
        return newtoken(EOF, "EOF")
    end

    # to not have to instance a Token struct every condition
    type::String = ""
    # Sometimes we need to use a literal that is greater than just the current char
    custom_literal = ""
    # parse to string.
    cc = string(l.CurrentChar)
    if cc == "="
        if peekchar(l) == '='
            type = EQ
            custom_literal = cc * '='            
            readchar!(l)
        else
            type = ASSIGN
        end
    elseif cc == "."
        type = DOT # Dot expression.
    elseif cc == "+"
        type = PLUS
    elseif cc == "{"
        type = L_BRACE
    elseif cc == "}"
        type = R_BRACE
    elseif cc == "("
        type = L_PAREN
    elseif cc == ")"
        type = R_PAREN
    elseif cc == ","
        type = COMMA
    elseif cc == "-"
        type = MINUS
    elseif cc == ";"
        type = SEMICOLON
    elseif cc == "\n"
        type = EOL
    elseif cc == "!"
        if peekchar(l) == '='
            type = NOT_EQ
            custom_literal = cc * '='
            readchar!(l)
        else
            type = BANG
        end
    elseif cc == "\\"
        type = SLASH
    elseif cc == "*"
        type = ASTERISK
    elseif cc == "<"
        type = LT
    elseif cc == ">"
        type = GT
    elseif cc == ":"
        # check the next char
        next_char = peekchar(l)
        type = lookupColonSpecial(string(cc, next_char))
        
        # if the type is not a COLON, we have a special colon.
        if type != COLON
            custom_literal = cc * next_char
            # go to next char after special
            readchar!(l)
        end
    elseif cc == "/"
        if peekchar(l) == '/'
            type = COMMENT
            custom_literal = readcomment!(l)
        else
            type = ILLEGAL
        end
    elseif cc == "\""
        type = STRING
        # read the string
        custom_literal = readstring!(l)
    # elseif cc == "/"
    #     type = COMMENT
    #     custom_literal = readcomment!(l)
    else
        # check Identifiers
        if isletter(l.CurrentChar)
            # read the identifier
            custom_literal = readidentifier!(l)
            # get the currect type
            type = lookupIndent(custom_literal)
            if type == JAVASCRIPT
                # read the javascript
                custom_literal = readjavascript!(l)
            end
        elseif isdigit(l.CurrentChar)
            type = INT
            # read the number
            custom_literal = readnumber!(l)
        else
            type = ILLEGAL
        end
    end

    # set the correct literal
    lit = length(custom_literal) > 0 ? custom_literal : cc

    token = newtoken(type, lit)
    # move next.
    readchar!(l)

    return token
end

function readstring!(l::Lex)
    position = l.Position + 1

    # go next char to not get stuck in the current "
    readchar!(l)

    while l.CurrentChar != '"' && l.CurrentChar != '\0'
        readchar!(l)
    end

    # Set read position . We do this because arrays in julia are 1 based.
    # l.ReadPosition = l.Position
    return l.Input[position:l.Position - 1]
end

function readcomment!(l::Lex)
    position = l.Position + 2
    # go to next to char to not get stuck in the /
    readchar!(l)
    while l.CurrentChar != '\n' && l.CurrentChar != '\0'
        readchar!(l)
    end

    return l.Input[position:l.Position - 1]
end

"""
Get the full identifier.
"""
function readidentifier!(l::Lex)
    position = l.Position
    while isletter(l.CurrentChar) || occursin(l.CurrentChar, "_1234567890")
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
    while (l.CurrentChar == ' ' || l.CurrentChar == '\t' || l.CurrentChar == '\r' || l.CurrentChar == '\n') && l.CurrentChar != '\0'
        readchar!(l)
    end
end

"""
Peak at the next upcoming character in Input.
"""
function peekchar(l::Lex):Char
    if l.ReadPosition >= length(l.Input)
        return 0
    end
    return l.Input[l.ReadPosition]
end

"""
Read all tokens from a input...
"""
function readalltokens(input::String)
    lex = Lexer.Lex(input, 0, 1, ' ')
    tokens = []

    while lex.ReadPosition <= length(input)
        push!(tokens, Lexer.nexttoken!(lex))
    end

    return tokens
end

"""
Add a string to a position and move all characters after it forward one.
"""
function add_string(l::Lex, position::Int, str::Char)
    @assert length(str) == 1

    l.Input = l.Input[1:position-1] * str * l.Input[position:length(l.Input)]
    l.ReadPosition -= 1
end

function readjavascript!(l::Lex)
    position = l.Position
    readchar!(l) # go to {
    readchar!(l) # skip it
    braces = 1
    while l.CurrentChar != '\0'
        if l.CurrentChar == '{'
            braces += 1
        elseif l.CurrentChar == '}'
            braces -= 1
            if braces == 0
                break
            end
        end
        readchar!(l)
    end

    l.Position = l.ReadPosition

    return l.Input[position:l.Position - 1]
end

end