module AST

include("lexer.jl")

function readalltokens(input::String)
    lex = Lexer.Lex(input, 0, 1, ' ')
    tokens = []

    while lex.ReadPosition < length(input)
        push!(tokens, Lexer.nexttoken!(lex))
    end

    for token in tokens
        println(token)
    end
end

readalltokens("
    x = 1
    y = 2

    z := 3

    fn test() {

    }

    n::int = x + y + z
")

end