module REPL

include("../utils/version.jl")
include("../transpiler/transpiler.jl")
include("js_runtime.jl")

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
    # runtime = openjsruntime("node")
    runtime = JSRuntime.start_runtime("node")

    println(easyjsasci)
    println("EasyJS " * EASY_JS_VERSION)
    while true
        # read input
        print(PROMPT)
        input = readline()

        if input == "quit"
            # close runtime
            JSRuntime.close_runtime(runtime)
            break
        elseif length(strip(input)) == 0
            continue
        end

        lexer = TRANSPILER.PARSER.Lexer.Lex(input, 1, 1, ' ')
        parser = TRANSPILER.PARSER.newparser(lexer)
        program = TRANSPILER.PARSER.parseprogram!(parser)

        # check errors
        if length(parser.errors) > 0
            printparse_errors(parser.errors)
        else
            jscode = TRANSPILER.transpile(program)
            js_response = JSRuntime.send_command(runtime, jscode.outballs[1])
            println(strip(split(js_response,">")[2]))
            # println(JSRuntime.send_command(runtime, jscode.outballs[1]))
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