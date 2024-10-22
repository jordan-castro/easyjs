module PARSER

include("ast.jl")

mutable struct Parser 
    l::Lexer.Lex
    c_token::Lexer.Token
    peek_token::Lexer.Token
    pre_token::Lexer.Token
    errors::Array{String}
end

function newparser(lexer::Lexer.Lex)
    parser = Parser(lexer, Lexer.Token("", ""), Lexer.Token("", ""), Lexer.Token("", ""), [])

    nexttoken!(parser)
    nexttoken!(parser)

    return parser
end

function nexttoken!(p::Parser)
    p.pre_token = p.c_token
    p.c_token = p.peek_token
    p.peek_token = Lexer.nexttoken!(p.l)
end

function parsestatement!(p::Parser)
    if p.c_token.Type == Lexer.ASSIGN
        return parsevarstatement!(p)
    end
    return nothing
end

function parseprogram!(p::Parser)::Program
    program = Program([])

    while !cur_tokenis(p, Lexer.EOF)
        stmt = parsestatement!(p)
        if stmt !== nothing
            push!(program.statements, stmt)
        end
        nexttoken!(p)
    end
    return program
end

function parsevarstatement!(p::Parser)
    token = p.c_token
    name = nothing
    value = nothing

    if !pretokenis(p, Lexer.IDENT)
        return nothing
    end

    name = Identifier(p.pre_token, p.pre_token.Literal)

    # if !expectpeek!(p, Lexer.ASSIGN)
    #     return nothing
    # end

    while !cur_tokenis(p, Lexer.LINE_BREAK) && !cur_tokenis(p, Lexer.EOF)
        nexttoken!(p)
    end

    return VariableStatement(token, name, Expression)
end

function pretokenis(p::Parser, type::String)
    return p.pre_token.Type == type
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

end