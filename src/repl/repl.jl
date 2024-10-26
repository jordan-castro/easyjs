module REPL

include("../utils/version.jl")
include("../parser/parser.jl")

# This is shown to the user.
const PROMPT = ">> "

const easyjsasci = "    ___       ___       ___       ___            ___       ___   
   /\\  \\     /\\  \\     /\\  \\     /\\__\\          /\\  \\     /\\  \\  
  /::\\  \\   /::\\  \\   /::\\  \\   |::L__L        _\\:\\  \\   /::\\  \\ 
 /::\\:\\__\\ /::\\:\\__\\ /\\:\\:\\__\\  |:::\\__\\      /\\/::\\__\\ /\\:\\:\\__\\
 \\:\\:\\/  / \\/\\::/  / \\:\\:\\/__/  /:;;/__/      \\::/\\/__/ \\:\\:\\/__/
  \\:\\/  /    /:/  /   \\::/  /   \\/__/          \\/__/     \\::/  / 
   \\/__/     \\/__/     \\/__/                              \\/__/  "

"""
Start the REPL.
"""
function start()
    println(easyjsasci)
    println("EasyJS " * EASY_JS_VERSION)
    while true
        # read input
        print(PROMPT)
        input = readline()

        if input == "quit"
            break
        end

        lexer = PARSER.Lexer.Lex(input, 1, 1, ' ')
        parser = PARSER.newparser(lexer)
        program = PARSER.parseprogram!(parser)

        # check errors
        if length(parser.errors) > 0
            printparse_errors(parser.errors)
        else
            println(PARSER.tostring(program))
        end
    end
end

function printparse_errors(errors::Array{String})
    println("EASY JS parse errors:")
    for e in errors
        println("\t" * e)
    end
end

end