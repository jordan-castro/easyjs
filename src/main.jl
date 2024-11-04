module EasyJS

using Pkg
Pkg.activate(".")

include("utils/version.jl")
include("utils/try.jl")
include("repl/repl.jl")
include("transpiler/transpiler.jl")

using ArgParse

s = ArgParseSettings(
    description="EasyJS compiler and REPL.",
    commands_are_required=false,
    version=EASY_JS_VERSION,
    add_version=true
)

@add_arg_table s begin
    "command"
        help = "EasyJS command (compile, repl, run, or just a .ej file)"
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
    "--crash", "-c"
        help = "Crash on error."
        action = :store_true
end

args = parse_args(s)
command = args["command"]
input_file = args["inputfile"]
output_file = args["outputfile"]
pretty = args["pretty"]
runtime = args["runtime"]
crash = args["crash"]

if command == "version"
    println(EASY_JS_VERSION)
elseif command == "repl"
    REPL.start(runtime, crash)
elseif command == "compile"
    of = output_file
    if input_file === nothing
        println("Please specify an input file")
        return
    end
    if of === nothing
        of = split(input_file, ".")[1] * ".js"
    end
    input = tryread(input_file, String)
    if input == ""
        println("Please specify an input file")
        return
    end
    result = TRANSPILER.transpile_from_input(input, true)
    if typeof(result) == String
        write(of, "// Compiled with EasyJS version $EASY_JS_VERSION\n" * result)
    else
        for err in result
            println(err)
        end
    end
elseif command == "run"
    # compile and run
    input = tryread(input_file, String)
    if input == ""
        println("Please specify an input file")
        return
    end
    out = split(input_file, ".")[1] * ".js"
    result = TRANSPILER.transpile_from_input(input, true)
    if typeof(result) == String
        println("Writing env")
        write(out, "// Compiled with EasyJS version $EASY_JS_VERSION\n" * result)
        println("Compiled, running with $runtime")
        REPL.JSRuntime.run_file(runtime, out)
    else
        for err in result
            println(err)
        end
    end

else
    println("Unknown command: " * command)
end
end