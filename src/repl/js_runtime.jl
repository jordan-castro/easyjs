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

    return output[1]
end

function start_runtime(js_runtime_option::String)
    p = nothing

    if js_runtime_option == "node"
        p = open(`node -i`, "r+")
        # command = `node -i`
    elseif js_runtime_option == "deno"
        p = open(`deno repl`, "r+")
    elseif js_runtime_option == "bun"
        p = open(`bun repl -i`, "r+")
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

# p = start_runtime("node")

# # Main loop to send commands interactively
# while true
#     print("> ")
#     command = readline(stdin)
#     if command == "quit"
#         break
#     elseif strip(command) == ""
#         continue
#     end

#     # Print the REPL output from the command
#     output = send_command(p, command)
#     println(join(output, "\n"))
# end

# # Clean up
# close(p.in)
# wait(p)

end