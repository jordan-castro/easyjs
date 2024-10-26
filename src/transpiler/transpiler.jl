module TRANSPILER

include("../parser/parser.jl")

mutable struct JSCode
    globals::Vector{String}
    fns::Vector{String}
    outballs::Vector{String}
end

function transpile(program::PARSER.Program)
    js = JSCode([], [], [])

    for stmt in program.statements
        if typeof(stmt) == PARSER.ExpressionStatement
            exp = stmt.expression
            if typeof(exp) == PARSER.IntegerLiteral
                addoutball!(js, string(exp.value))
            end 
        elseif typeof(stmt) == PARSER.VariableStatement
        elseif typeof(stmt) == PARSER.ReturnStatement
        end
    end

    return js
end

function addglobal!(js::JSCode, g::String)
    push!(js.globals, g)
end

function addfn!(js::JSCode, fn::String)
    push!(js.fns, fn)
end

function addoutball!(js::JSCode, outball::String)
    push!(js.outballs, outball)
end

end