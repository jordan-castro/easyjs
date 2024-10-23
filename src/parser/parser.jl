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
const EQUALS = 2      # ==
const LESSGREATER = 3 # < or >
const SUM = 4         # +
const PRODUCT = 5     # *
const PREFIX = 6      # -X or !X
const CALL = 7        # my_function(X)

const precedences = Dict(
    Lexer.EQ => EQUALS,
    Lexer.NOT_EQ => EQUALS,
    Lexer.LT => LESSGREATER,
    Lexer.GT => LESSGREATER,
    Lexer.PLUS => SUM,
    Lexer.MINUS => SUM,
    Lexer.SLASH => PRODUCT,
    Lexer.ASTERISK => PRODUCT
)

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

    # register infix
    register_infix!(parser, Lexer.PLUS, parse_infix_expression!)
    register_infix!(parser, Lexer.MINUS, parse_infix_expression!)
    register_infix!(parser, Lexer.SLASH, parse_infix_expression!)
    register_infix!(parser, Lexer.ASTERISK, parse_infix_expression!)
    register_infix!(parser, Lexer.EQ, parse_infix_expression!)
    register_infix!(parser, Lexer.NOT_EQ, parse_infix_expression!)
    register_infix!(parser, Lexer.LT, parse_infix_expression!)
    register_infix!(parser, Lexer.GT, parse_infix_expression!)

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
    if p.c_token.Type == Lexer.IDENT && peektokenis(p, Lexer.ASSIGN)
        return parsevarstatement!(p)
    elseif p.c_token.Type == Lexer.RETURN
        return parsereturnstatement!(p)
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

function parsevarstatement!(p::Parser)
    token = p.c_token
    name = nothing
    value = DefaultDontUseExpression()

    # there needs to be a equals sign...
    if !expectpeek!(p, Lexer.ASSIGN)
        return nothing
    end

    name = Identifier(token, token.Literal)

    while !cur_tokenis(p, Lexer.LINE_BREAK) && !cur_tokenis(p, Lexer.EOF)
        nexttoken!(p)
    end

    return VariableStatement(token, name, value)
end

function parsereturnstatement!(p::Parser)
    token = p.c_token
    return_value = DefaultDontUseExpression()

    nexttoken!(p)

    while !cur_tokenis(p, Lexer.LINE_BREAK) && !cur_tokenis(p, Lexer.EOF)
        nexttoken!(p)
    end

    return ReturnStatement(token, return_value)
end

function parse_expression_statement!(p::Parser)
    token = p.c_token
    expression = parse_expression!(p, LOWEST)

    if expression === nothing
        return nothing
    end

    # we hit the end of the line.
    if peektokenis(p, Lexer.LINE_BREAK)
        nexttoken!(p)
    end

    return ExpressionStatement(token, expression)
end

function parse_expression!(p::Parser, precedence::Int64)
    prefix = get(p.prefix_parse_fns, p.c_token.Type, nothing)
    if prefix === nothing
        
        return nothing
    end

    left_exp = prefix(p)

    while !(peektokenis(p, Lexer.LINE_BREAK) || peektokenis(p, Lexer.EOF))
        infix = get(p.infix_parse_fns, p.peek_token.Type, nothing)
        if infix === nothing
            return left_exp
        end
        nexttoken!(p)
        left_exp = infix(p, left_exp)
    end

    return left_exp
end

function parse_identifier(p::Parser)
    return Identifier(p.c_token, p.c_token.Literal)
end

function parse_integer_literal(p::Parser)
    token = p.c_token
    value = tryparse(Int64, token.Literal)
    if value === nothing
        push!(p.errors, "could not parse " * token.Literal * " as integer")
    end

    return IntegerLiteral(token, value)
end

function parse_prefix_expression!(p::Parser)
    token = p.c_token
    operator = p.c_token.Literal

    nexttoken!(p)

    right = parse_expression!(p, PREFIX)

    return PrefixExpression(token, operator, right)
end

function parse_infix_expression!(p::Parser, left::Expression)
    token = p.c_token
    operator = p.c_token.Literal

    precedence = cur_precedence(p)
    nexttoken!(p)
    right = parse_expression!(p, precedence)

    return InfixExpression(token, left, operator, right)
end

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

function peekerror!(p::Parser, type::String)
    push!(p.errors, "expected next token to be " * type * ", got " * p.peek_token.Type * " instead")
end

function peekprecedence(p::Parser)
    return get(precedences, p.peek_token.Type, LOWEST)
end

function cur_precedence(p::Parser)
    return get(precedences, p.c_token.Type, LOWEST)
end

# function noprefix_parse_fn_error!()

end