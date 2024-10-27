module TRANSPILER

include("../parser/parser.jl")

mutable struct JSCode
    script::Vector{String}
end

function tostring(js::JSCode)
    return join(js.script, "")
end

function transpile(program::PARSER.Program)
    js = JSCode([])

    for stmt in program.statements
        script = jsify_statement(stmt)
        if script !== nothing
            push!(js.script, script)
        end
    end
    
    # for stmt in program.statements
    #     if typeof(stmt) == PARSER.ExpressionStatement
    #         exp = stmt.expression
    #         if typeof(exp) == PARSER.IntegerLiteral
    #             addoutball!(js, string(exp.value))
    #         end 
    #     elseif typeof(stmt) == PARSER.VariableStatement
    #     elseif typeof(stmt) == PARSER.ReturnStatement
    #     end
    # end

    return js
end

function jsify_statement(stmt::PARSER.Statement)
    if typeof(stmt) == PARSER.ExpressionStatement
        
    end
end

end