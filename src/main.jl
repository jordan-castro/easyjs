module EasyJS

using Pkg
Pkg.activate(".")

include("utils/version.jl")
include("repl/repl.jl")
include("transpiler/transpiler.jl")

using ArgParse

s = ArgParseSettings()
@add_arg_table s begin
    "command"
        help = "EasyJS command (compile, repl)"
        default = "repl"
    "inputfile"
        help = "EasyJS input script"
    "outputfile"
        help = "JavaScript output file name"
    "--pretty"
        help = "Enable pretty output formatting"
        action = :store_true
    "--runtime"
        help = "Choose a runtime for your repl."
        default = "node"
end

args = parse_args(s)
command = args["command"]
input_file = args["inputfile"]
output_file = args["outputfile"]
pretty = args["pretty"]
runtime = args["runtime"]

if command == "version"
    println(EASY_JS_VERSION)
elseif command == "repl"
    REPL.start(runtime)
elseif command == "compile"
    of = output_file
    if input_file === nothing
        println("Please specify an input file")
        return
    end
    if of === nothing
        of = split(input_file, ".")[1] * ".min.js"
    end
    input = read(input_file, String)
    lexer = TRANSPILER.PARSER.Lexer.Lex(input, 1, 1, ' ')
    parser = TRANSPILER.PARSER.newparser(lexer)
    program = TRANSPILER.PARSER.parseprogram!(parser)

    if length(parser.errors) > 0
        REPL.printparse_errors(parser.errors)
        return
    end

    jscode = TRANSPILER.transpile!(program)
    js = TRANSPILER.tostring(jscode, pretty)
    write(of, js)
else
    println("Unknown command: " * command)
end
end