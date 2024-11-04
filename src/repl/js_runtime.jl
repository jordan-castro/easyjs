module JSRuntime

using Base64, Random
import Base: close, readavailable, write

# Define EASY_JS_CONSTANT using a random string hash
EASY_JS_CONSTANT = 001101

# Define a function to send commands to the Node.js REPL
function send_command(p::Base.Process, command::String)
    command_with_marker = command * "\n$EASY_JS_CONSTANT\n"
    write(p, command_with_marker)
    flush(p)

    # Collect output from the REPL
    output = String[]
    while true
        out = readline(p.out)
        d_out = String(out)
        push!(output, d_out)
        # Check for EASY_JS_CONSTANT to break the loop
        if occursin(string(EASY_JS_CONSTANT), d_out)
            break
        end
    end

    return join(output[1:end-1], "\n")
end

function start_runtime(js_runtime_option::String)
    p = nothing

    if js_runtime_option == "node"
        p = open(`node -i`, "r+")
    elseif js_runtime_option == "deno"
        p = open(`deno repl`, "r+")
    else
        return nothing # this runtime option is not supported.
    end
    sleep(1)
    # send initial command to
    send_command(p, "const EASY_JS_CONSTANT = '$EASY_JS_CONSTANT';")

    return p
end

function close_runtime(p::Base.Process)
    close(p.in)
    wait(p)
end

function pretty_response(response::String)
    if startswith(response, ">")
        return response[2:end]
    end
    return response
end

function run_file(js_runtime_option::String, path::String)
    p = nothing
    if js_runtime_option == "node"
        p = open(`node $path`, "r")
    elseif js_runtime_option == "deno"
        p = open(`deno $path`, "r")
    elseif js_runtime_option == "bun"
        p = open(`bun $path`, "r")
    end
    sleep(1)
end

end