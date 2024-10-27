module TRANSPILER

include("../parser/parser.jl")

mutable struct JSCode
    script::Vector{String} # <-- Line by Line JS.
    variables::Vector{String} # <-- All variables that have been declared.
    fns::Vector{String} # <-- All functions that have been declared.
end

"""
Update JSCode.
"""
function update!(js::JSCode, scripts::Vector{String}, variables::Vector{String}, fns::Vector{String})
    for script in js.script
        if script in scripts
            continue
        end
        push!(scripts, script)
    end
    for var in js.variables
        if var in variables
            continue
        end
        push!(variables, var)
    end
    for fn in js.fns
        if fn in fns
            continue
        end
        push!(fns, fn)
    end
end

function tostring(js::JSCode, pretty::Bool = false)
    joiner = pretty ? "\n" : ""
    return replace(join(js.script, joiner), Pair(";;", ";"))
end

function transpile!(program::PARSER.Program, js::JSCode = JSCode([], [], []))
    for stmt in program.statements
        script = jsify_statement!(js, stmt)
        if script !== nothing
            push!(js.script, script)
        end
    end

    return js
end

function jsify_statement!(js::JSCode, stmt::PARSER.Statement)
    if typeof(stmt) == PARSER.ExpressionStatement
        return jsify_expression!(js, stmt.expression)
    elseif typeof(stmt) == PARSER.VariableStatement
        return jsify_varstatement!(js, stmt)
    elseif typeof(stmt) == PARSER.ReturnStatement
        return jsify_return_statement!(js, stmt)
    elseif typeof(stmt) == PARSER.BlockStatement
        return jsify_blockstatement!(js, stmt)
    elseif typeof(stmt) == PARSER.ConstVariableStatement
        return jsify_const_varstatement!(js, stmt)
    end
end

function jsify_const_varstatement!(js::JSCode, stmt::PARSER.ConstVariableStatement)
    return "const " * stmt.name.value * " = " * jsify_expression!(js, stmt.value) * ";"
end

function jsify_expression!(js::JSCode, exp::PARSER.Expression)
    if typeof(exp) == PARSER.IntegerLiteral
        return string(exp.value)
    elseif typeof(exp) == PARSER.PrefixExpression
        return PARSER.tostring(exp) # we already cover this in the parser
    elseif typeof(exp) == PARSER.InfixExpression
        return PARSER.tostring(exp) # we already cover this in the parser
    elseif typeof(exp) == PARSER.IfExpression
        str = "if "
        if typeof(exp.condition) == PARSER.Identifier
            str *= "(" * jsify_expression!(js, exp.condition) * ")"
        else 
            str *= jsify_expression!(js, exp.condition)
        end
        str *= " {" * jsify_statement!(js, exp.consequence) * "}"
        if exp.alternative !== nothing
            if typeof(exp.alternative) == PARSER.BlockStatement
                str *= " else {" * jsify_statement!(js, exp.alternative) * "}"
            elseif typeof(exp.alternative) == PARSER.IfExpression
                str *= " else " * jsify_expression!(js, exp.alternative)
            end
        end
        return str
    elseif typeof(exp) == PARSER.FunctionLiteral
        str = "const " * exp.name.value * " = ("
        for p in exp.paramaters
            str *= jsify_expression!(js, p)
            if p !== exp.paramaters[end]
                str *= ", "
            end
        end
        str *= ") => {" * jsify_statement!(js, exp.body) * "};"
        return str
    elseif typeof(exp) == PARSER.CallExpression
        str = ""
        if typeof(exp.fn) == PARSER.FunctionLiteral
            str = exp.fn.name.value
        else # ident...
            str = exp.fn.value
        end

        str *= "("
        for p in exp.arguments
            str *= jsify_expression!(js, p)
            if p !== exp.arguments[end]
                str *= ", "
            end
        end
        str *= ");"
        return str
    elseif typeof(exp) == PARSER.Boolean
        if exp.value
            return "true"
        else
            return "false"
        end
    elseif typeof(exp) == PARSER.Identifier
        return exp.value
    end
end

function jsify_varstatement!(js::JSCode, stmt::PARSER.VariableStatement)
    # check if js.variables contains varname
    if stmt.name.value in js.variables
        return stmt.name.value * " = " * jsify_expression!(js, stmt.value) * ";"
    else
        push!(js.variables, stmt.name.value)
        return "let " * stmt.name.value * " = " * jsify_expression!(js, stmt.value) * ";"
    end
end

function jsify_return_statement!(js::JSCode, stmt::PARSER.ReturnStatement)
    return "return " * jsify_expression!(js, stmt.return_value) * ";"
end

function jsify_blockstatement!(js::JSCode, stmt::PARSER.BlockStatement)
    str = ""
    for stmt in stmt.statements
        str *= jsify_statement!(js, stmt) * "; "
    end
    return str
end

end