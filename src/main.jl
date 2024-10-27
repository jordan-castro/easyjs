module EasyJS

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
end

args = parse_args(s)
command = args["command"]
input_file = args["inputfile"]
output_file = args["outputfile"]

if command == "version"
    println(EASY_JS_VERSION)
elseif command == "repl"
    REPL.start()
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
    jscode = TRANSPILER.transpile(program)
    write(of, TRANSPILER.tostring(jscode))
end

# easyjs easyfile.ej output.min.js

# include("repl/repl.jl")
# REPL.start()

end