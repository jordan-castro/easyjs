module REPL

include("../utils/version.jl")
include("../lexer/lexer.jl")

# This is shown to the user.
const PROMPT = ">> "

"""
Start the REPL.
"""
function start()
    println("EasyJS " * EASY_JS_VERSION)
    while true
        # read input
        print(PROMPT)
        input = readline()

        if input == "quit"
            break
        end

        lexer = Lexer.Lex(input, 1, 1, ' ')
        while true
            token = Lexer.nexttoken!(lexer)
            if token.Type == Lexer.EOF
                break
            end
            println("Type: " * token.Type * " Literal: " * token.Literal)
        end
    end
end

end