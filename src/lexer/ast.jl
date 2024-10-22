module AST

include("lexer.jl")

function readalltokens(input::String)
    lex = Lexer.Lex(input * " ", 0, 1, ' ')
    tokens = []

    while lex.ReadPosition < length(input)
        push!(tokens, Lexer.nexttoken!(lex))
    end

    return tokens
end


end