module PARSER

include("ast.jl")

mutable struct Parser
    l::Lexer.Lex
    c_token::Lexer.Token
    peek_token::Lexer.Token
    errors::Array{String}

    prefix_parse_fns::Dict
    infix_parse_fns::Dict
end

const LOWEST = 1
const EQUALS = 2                  # ==
const LESSGREATER = 3             # < or >
const SUM = 4                     # +
const PRODUCT = 5                 # *
const PREFIX = 6                  # -X or !X
const CALL = 7                    # my_function(X)
const DOT = 8                     # .field or .method
const JAVASCRIPT = 9              # javascript code
const LESSGREATER_OR_EQUALS = 10  # <= or >=
const BRACKET = 11                # [
const BRACE = 12                  # {

"""
This is what goes in front of the token.

It is a mapping of string => integer.
"""
const precedences = Dict(
    Lexer.EQ => EQUALS,
    Lexer.NOT_EQ => EQUALS,
    Lexer.LT => LESSGREATER,
    Lexer.GT => LESSGREATER,
    Lexer.PLUS => SUM,
    Lexer.MINUS => SUM,
    Lexer.SLASH => PRODUCT,
    Lexer.ASTERISK => PRODUCT,
    Lexer.L_PAREN => CALL,
    Lexer.DOT => DOT,
    Lexer.JAVASCRIPT => JAVASCRIPT,
    Lexer.LT_OR_EQ => LESSGREATER_OR_EQUALS,
    Lexer.GT_OR_EQ => LESSGREATER_OR_EQUALS,
    Lexer.L_BRACKET => BRACKET,
    Lexer.L_BRACE => BRACE
)

function cur_tokenis(p::Parser, type::String)
    return p.c_token.Type == type
end

function peektokenis(p::Parser, type::String)
    return p.peek_token.Type == type
end

function expectpeek!(p::Parser, type::String)
    if peektokenis(p, type)
        nexttoken!(p)
        return true
    end
    peekerror!(p, type)
    return false
end

function peekerror!(p::Parser, type::String, line_number::Int64=0)
    push!(p.errors, "expected next token to be " * type * " BUT got " * p.peek_token.Type * " instead on line " * string(line_number))
end

function peekprecedence(p::Parser)
    return get(precedences, p.peek_token.Type, LOWEST)
end

function cur_precedence(p::Parser)
    return get(precedences, p.c_token.Type, LOWEST)
end

"""
Check if the current token is a End of Statement token. i.e. ; or \n
"""
function cur_tokenis_eos(p::Parser)
    return cur_tokenis(p, Lexer.SEMICOLON) || cur_tokenis(p, Lexer.EOL)
end

"""
Check if the peek token is a end of statement token. i.e. ; or \n
"""
function peektokenis_eos(p::Parser)
    return peektokenis(p, Lexer.EOL) || peektokenis(p, Lexer.SEMICOLON)
end

"""
Create a new parser struct. Expects a lexer object.
"""
function newparser(lexer::Lexer.Lex)
    parser = Parser(lexer, Lexer.Token("", ""), Lexer.Token("", ""), [], Dict(), Dict())

    # fill c_token and peek_token
    nexttoken!(parser)
    nexttoken!(parser)

    # register the parse_identifier function
    register_prefix!(parser, Lexer.IDENT, parse_identifier)
    register_prefix!(parser, Lexer.INT, parse_integer_literal)
    register_prefix!(parser, Lexer.BANG, parse_prefix_expression!)
    register_prefix!(parser, Lexer.MINUS, parse_prefix_expression!)
    register_prefix!(parser, Lexer.TRUE, parse_boolean)
    register_prefix!(parser, Lexer.FALSE, parse_boolean)
    register_prefix!(parser, Lexer.L_PAREN, parse_grouped_expression!)
    register_prefix!(parser, Lexer.IF, parse_if_expression!)
    register_prefix!(parser, Lexer.FUNCTION, parse_function_literal!)
    register_prefix!(parser, Lexer.STRING, parse_string_literal!)
    register_prefix!(parser, Lexer.COMMENT, parse_comment!)
    register_prefix!(parser, Lexer.JAVASCRIPT, parse_javascript_expression!)
    register_prefix!(parser, Lexer.L_BRACKET, parse_array_literal!)
    register_prefix!(parser, Lexer.L_BRACE, parse_object_literal!)

    # register infix
    register_infix!(parser, Lexer.PLUS, parse_infix_expression!)
    register_infix!(parser, Lexer.MINUS, parse_infix_expression!)
    register_infix!(parser, Lexer.SLASH, parse_infix_expression!)
    register_infix!(parser, Lexer.ASTERISK, parse_infix_expression!)
    register_infix!(parser, Lexer.EQ, parse_infix_expression!)
    register_infix!(parser, Lexer.NOT_EQ, parse_infix_expression!)
    register_infix!(parser, Lexer.LT, parse_infix_expression!)
    register_infix!(parser, Lexer.GT, parse_infix_expression!)
    register_infix!(parser, Lexer.L_PAREN, parse_call_expression!)
    register_infix!(parser, Lexer.DOT, parse_dot_expression!)
    register_infix!(parser, Lexer.JAVASCRIPT, parse_infix_expression!)
    register_infix!(parser, Lexer.LT_OR_EQ, parse_infix_expression!)
    register_infix!(parser, Lexer.GT_OR_EQ, parse_infix_expression!)
    register_infix!(parser, Lexer.L_BRACKET, parse_index_expression!)

    return parser
end

"""
Register a prefix parse function.
"""
function register_prefix!(p::Parser, token_type::String, fn)
    p.prefix_parse_fns[token_type] = fn
end

"""
Register a infix parse function.
"""
function register_infix!(p::Parser, token_type::String, fn)
    p.infix_parse_fns[token_type] = fn
end

"""
Get the next token from the lexer and update c_token and peek token.
"""
function nexttoken!(p::Parser)
    p.c_token = p.peek_token
    p.peek_token = Lexer.nexttoken!(p.l)
end

"""
Parse a statement (line).
"""
function parsestatement!(p::Parser)
    # Var statements are when the left token is a ident and the rigth token is an assign
    if p.c_token.Type == Lexer.IDENT
        # && peektokenis(p, Lexer.ASSIGN)
        if peektokenis(p, Lexer.ASSIGN)
            return parsevarstatement!(p)
        elseif peektokenis(p, Lexer.CONST_ASSIGNMENT)
            return parse_const_var_statement!(p)
        else
            return parse_expression_statement!(p) # default to expression
        end
    elseif p.c_token.Type == Lexer.RETURN
        return parsereturnstatement!(p)
    elseif p.c_token.Type == Lexer.IMPORT
        return parse_import_statement!(p)
    elseif p.c_token.Type == Lexer.JAVASCRIPT
        return JavaScriptStatement(p.c_token, p.c_token.Literal)
    else # our default (expression) statements
        return parse_expression_statement!(p)
    end
    return nothing
end

"""
Parse a program.
"""
function parseprogram!(p::Parser)::Program
    program = Program([])

    # parse until EOF
    while !cur_tokenis(p, Lexer.EOF)
        stmt = parsestatement!(p)
        # if we have a statement add it to the list.
        if stmt !== nothing
            push!(program.statements, stmt)
        end
        # go to next token
        nexttoken!(p)
    end
    return program
end

"""
Parse a var statement.

A variable statement looks like this in EJS:
`x = 5`
"""
function parsevarstatement!(p::Parser)
    token = p.c_token

    # there needs to be a equals sign...
    if !expectpeek!(p, Lexer.ASSIGN)
        return nothing
    end

    name = Identifier(token, token.Literal)

    nexttoken!(p)

    value = parse_expression!(p, LOWEST)

    if peektokenis_eos(p)
        nexttoken!(p)
    end

    return VariableStatement(token, name, value)
end

function parse_const_var_statement!(p::Parser)
    token = p.c_token

    # there needs to be a equals sign...
    if !expectpeek!(p, Lexer.CONST_ASSIGNMENT)
        return nothing
    end

    name = Identifier(token, token.Literal)

    nexttoken!(p)

    value = parse_expression!(p, LOWEST)

    if value === nothing
        return nothing
    end

    if peektokenis_eos(p)
        nexttoken!(p)
    end

    return ConstVariableStatement(token, name, value)
end

"""
Parse a return statement.

This is specefic to when there is the `return` keyword. Like:

`return x`
"""
function parsereturnstatement!(p::Parser)
    token = p.c_token

    nexttoken!(p)

    return_value = parse_expression!(p, LOWEST)

    if peektokenis_eos(p)
        nexttoken!(p)
    end

    return ReturnStatement(token, return_value)
end

"""
An expression statement is anything really. A function, a single variable, math, if statement.
"""
function parse_expression_statement!(p::Parser)
    token = p.c_token
    expression = parse_expression!(p, LOWEST)

    if expression === nothing
        return nothing
    end

    # we hit the end of the line.
    if peektokenis_eos(p)
        nexttoken!(p)
    end

    return ExpressionStatement(token, expression)
end

"""
Parse the exact expression (is it a Identifier, PrefixExpression, InfixExpression?)
"""
function parse_expression!(p::Parser, precedence::Int64)
    prefix = get(p.prefix_parse_fns, p.c_token.Type, nothing)
    if prefix === nothing
        push!(p.errors, "No prefix parse function for " * p.c_token.Type * " found")
        return nothing
    end

    left_exp = prefix(p)

    while !(peektokenis_eos(p) || peektokenis(p, Lexer.EOF)) && precedence < peekprecedence(p)
        infix = get(p.infix_parse_fns, p.peek_token.Type, nothing)
        if infix === nothing
            return left_exp
        end
        nexttoken!(p)
        left_exp = infix(p, left_exp)
    end

    return left_exp
end

"""
Parsing the identifier is pretty easy huh?
"""
function parse_identifier(p::Parser)
    return Identifier(p.c_token, p.c_token.Literal)
end

"""
When parsing a integer literal we have to parse the string.
"""
function parse_integer_literal(p::Parser)
    token = p.c_token
    value = tryparse(Int64, token.Literal)
    if value === nothing
        push!(p.errors, "could not parse " * token.Literal * " as integer")
    end

    return IntegerLiteral(token, value)
end

"""
Prefix : comes before. `prefix --> !5 `
"""
function parse_prefix_expression!(p::Parser)
    token = p.c_token
    operator = p.c_token.Literal

    nexttoken!(p)

    right = parse_expression!(p, PREFIX)

    return PrefixExpression(token, operator, right)
end

"""
Infix : comes after. `5 + <-- infix`
"""
function parse_infix_expression!(p::Parser, left::Expression)
    token = p.c_token
    operator = p.c_token.Literal

    precedence = cur_precedence(p)
    nexttoken!(p)
    right = parse_expression!(p, precedence)

    return InfixExpression(token, left, operator, right)
end

function parse_boolean(p::Parser)
    return Boolean(p.c_token, cur_tokenis(p, Lexer.TRUE))
end

function parse_if_expression!(p::Parser)
    token = p.c_token
    condition = nothing
    alternative = nothing

    # Check for (
    if !peektokenis(p, Lexer.L_PAREN)
        # this expression is a batman if statement.
        return parse_if_expression_without_parenthesis!(p)
    end

    # update tokens.
    nexttoken!(p)
    nexttoken!(p)

    condition = parse_expression!(p, LOWEST)

    # do we have a result?
    if condition === nothing
        return nothing
    end

    # expect our )
    if !expectpeek!(p, Lexer.R_PAREN)
        return nothing
    end

    # we need to have a { no matter what.
    if !expectpeek!(p, Lexer.L_BRACE) # --> {
        return nothing
    end

    consequence = parse_block_statement!(p)

    if peektokenis(p, Lexer.ELSE)
        nexttoken!(p)
        if !expectpeek!(p, Lexer.L_BRACE)  # Ensure `{` after `else`
            return nothing
        end
        alternative = parse_block_statement!(p)

    elseif peektokenis(p, Lexer.ELIF)
        nexttoken!(p)  # Advance to `elif`
        alternative = parse_if_expression!(p)
    end

    return IfExpression(token, condition, consequence, alternative)
end

"""
if condition {
"""
function parse_if_expression_without_parenthesis!(p::Parser)
    # push!(p.errors, "Batman if expressions have not been implemented yet, please use () in the meanwhile.")
    token = p.c_token
    nexttoken!(p) # hop off the "if" token.
    condition = parse_expression!(p, LOWEST)
    alternative = nothing

    if condition === nothing
        return nothing
    end

    # otherwise check for a starting bracket
    if !expectpeek!(p, Lexer.L_BRACE)
        return nothing
    end

    consequence = parse_block_statement!(p)

    if peektokenis(p, Lexer.ELSE)
        nexttoken!(p)
        if !expectpeek!(p, Lexer.L_BRACE)  # Ensure `{` after `else`
            return nothing
        end
        alternative = parse_block_statement!(p)

    elseif peektokenis(p, Lexer.ELIF)
        nexttoken!(p)  # Advance to `elif`
        alternative = parse_if_expression!(p)
    end

    return IfExpression(token, condition, consequence, alternative)
end

"""
()
"""
function parse_grouped_expression!(p::Parser, stop_at=Lexer.R_PAREN)
    nexttoken!(p)
    expression = parse_expression!(p, LOWEST)
    if !expectpeek!(p, stop_at)
        return nothing
    end
    return expression
end

function parse_block_statement!(p::Parser)
    token = p.c_token
    statements = []

    nexttoken!(p)

    while !cur_tokenis(p, Lexer.R_BRACE) && !cur_tokenis(p, Lexer.EOF)
        stmt = parsestatement!(p)
        if stmt !== nothing
            # append to the statmens
            push!(statements, stmt)
        end
        nexttoken!(p)
    end

    return BlockStatement(token, statements)
end

function parse_function_paramaters!(p::Parser)
    identifiers = []

    # if we have no params
    if peektokenis(p, Lexer.R_PAREN)
        nexttoken!(p)
        return identifiers
    end

    nexttoken!(p)

    ident = Identifier(p.c_token, p.c_token.Literal)
    push!(identifiers, ident)

    while peektokenis(p, Lexer.COMMA)
        nexttoken!(p)
        nexttoken!(p)
        ident = Identifier(p.c_token, p.c_token.Literal)
        push!(identifiers, ident)
    end

    if !expectpeek!(p, Lexer.R_PAREN)
        return nothing # what what what? there MUST be a r )
    end

    return identifiers
end

function parse_function_literal!(p::Parser)
    token = p.c_token

    if peektokenis(p, Lexer.L_PAREN) # it could be a lambda?
        return parse_lambda_literal!(p)
    end

    # otherwise this is for sure a function.
    if !expectpeek!(p, Lexer.IDENT)
        return nothing
    end
    name = Identifier(p.c_token, p.c_token.Literal)

    if !expectpeek!(p, Lexer.L_PAREN) # needs to have a (
        return nothing
    end

    parameters = parse_function_paramaters!(p)

    if !expectpeek!(p, Lexer.L_BRACE)
        return nothing
    end

    body = parse_block_statement!(p)

    return FunctionLiteral(token, name, parameters, body)
end

function parse_call_arguments!(p::Parser)
    args = []

    if peektokenis(p, Lexer.R_PAREN)
        nexttoken!(p)
        return args
    end

    nexttoken!(p)
    args = push!(args, parse_expression!(p, LOWEST))

    while peektokenis(p, Lexer.COMMA)
        nexttoken!(p)
        nexttoken!(p)

        args = push!(args, parse_expression!(p, LOWEST))
    end

    if !expectpeek!(p, Lexer.R_PAREN)
        return nothing
    end

    return args
end

function parse_call_expression!(p::Parser, fn::Expression)
    token = p.c_token
    arguments = parse_call_arguments!(p)
    return CallExpression(token, fn, arguments)
end

function parse_string_literal!(p::Parser)
    return StringLiteral(p.c_token, p.c_token.Literal)
end

function parse_comment!(p::Parser)
    return Comment(p.c_token, p.c_token.Literal)
end

function parse_import_statement!(p::Parser)
    token = p.c_token

    if !expectpeek!(p, Lexer.STRING)
        return nothing
    end

    path = p.c_token.Literal
    as = ""

    # check for as
    if peektokenis(p, Lexer.AS)
        nexttoken!(p)
        if !expectpeek!(p, Lexer.IDENT)
            return nothing
        end
        as = p.c_token.Literal
    end

    return ImportStatement(token, path, as)
end

function parse_dot_expression!(p::Parser, left::Expression)
    token = p.c_token

    if !expectpeek!(p, Lexer.IDENT)
        return nothing
    end

    right = parse_expression!(p, LOWEST)

    if right === nothing
        return nothing
    end

    return DotExpression(token, left, right)
end

function parse_javascript_expression!(p::Parser)
    return JavaScriptExpression(p.c_token, p.c_token.Literal)
end

function parse_lambda_literal!(p::Parser)
    token = p.c_token

    if !expectpeek!(p, Lexer.L_PAREN)
        return nothing
    end
    # parse the ()
    parameters = parse_function_paramaters!(p)

    if !expectpeek!(p, Lexer.L_BRACE)
        return nothing
    end

    body = parse_block_statement!(p)

    if body === nothing
        return nothing
    end

    return LambdaLiteral(token, parameters, body)
end

function parse_array_arguments!(p::Parser)
    args = []

    if peektokenis(p, Lexer.R_BRACKET) # -> ]
        nexttoken!(p) # ]
        return args
    end

    nexttoken!(p) # Expression
    push!(args, parse_expression!(p, LOWEST))

    while peektokenis(p, Lexer.COMMA)
        nexttoken!(p) # ,
        nexttoken!(p) # Expression

        el = parse_expression!(p, LOWEST)
        if el === nothing
            continue
        end

        push!(args, el)
    end

    if !expectpeek!(p, Lexer.R_BRACKET) # ]
        return nothing
    end

    return args
end

function parse_array_literal!(p::Parser)
    token = p.c_token # [
    elements = parse_array_arguments!(p)
    return ArrayLiteral(token, elements)
end

function parse_index_expression!(p::Parser, left::Expression)
    token = p.c_token # [

    if peektokenis(p, Lexer.R_BRACKET) # -> ]
        return nothing
    end

    index = parse_expression!(p, LOWEST) # [0] | [some_ident] | [some_fn()] | [some.dot.expression]

    if index === nothing
        return nothing
    end

    if peektokenis(p, Lexer.ASSIGN) # -> =
        nexttoken!(p) # =
        nexttoken!(p) # Expression
        right = parse_expression!(p, LOWEST)

        if right === nothing
            return nothing
        else
            return IndexExpression(token, left, index, right)
        end
    end
    return IndexExpression(token, left, index, EmptyExpression())
end

function parse_object_literal!(p::Parser)
    println("parse_object_literal!")
    token = p.c_token # {
    elements = Dict{Expression,Expression}()

    if peektokenis(p, Lexer.R_BRACE) # -> }
        return ObjectLiteral(token, elements)
    end

    braces = 1
    while !peektokenis(p, Lexer.EOF)
        nexttoken!(p) # Expression or { or }

        if cur_tokenis(p, Lexer.L_BRACE) # {
            braces += 1
        elseif cur_tokenis(p, Lexer.R_BRACE) # }
            braces -= 1
            if braces == 0
                break
            end
        else
            key = parse_expression!(p, LOWEST)
            if !expectpeek!(p, Lexer.COLON)
                println("no colon")
                return nothing
            end
            nexttoken!(p) # :
            value = parse_expression!(p, LOWEST)
            
            if key === nothing || value === nothing
                return nothing
            end
            elements[key] = value
        end
        nexttoken!(p)
    end

    if !expectpeek!(p, Lexer.R_BRACE) # }
        return nothing
    end

    return ObjectLiteral(token, elements)
end

end