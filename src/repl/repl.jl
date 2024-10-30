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
    jscode = TRANSPILER.JSCode([], [], [], [])
    jshistory = TRANSPILER.JSCode([], [], [], [])
    while true
        jscode.import_paths = jshistory.import_paths
        # read input
        print(PROMPT)
        input = readline()

        if input == "quit"
            break
        elseif occursin("EASY_JS_PARSE_INTO=", input)
            # get file.
            file = split(input, "=")[2]
            write(file, TRANSPILER.tostring(jshistory, true))
            continue
        elseif length(strip(input)) == 0
            continue
        end

        # try
            lexer = TRANSPILER.PARSER.Lexer.Lex(input, 1, 1, ' ')
            parser = TRANSPILER.PARSER.newparser(lexer)
            program = TRANSPILER.PARSER.parseprogram!(parser)

            # check errors
            if length(parser.errors) > 0
                printparse_errors(parser.errors)
            else
                jscode.script = []
                TRANSPILER.transpile!(program, jscode)
                TRANSPILER.transpile!(program, jshistory)
                js_response = JSRuntime.send_command(runtime, TRANSPILER.tostring(jscode))
                println(strip(split(js_response,">")[2]))
            end
        # catch e 
        #     println(e)
        #     continue
        # end
    end
    # close runtime
    JSRuntime.close_runtime(runtime)

end

function printparse_errors(errors::Array{String})
    println("EASY JS parse errors:")
    for e in errors
        println("\t" * e)
    end
end

end