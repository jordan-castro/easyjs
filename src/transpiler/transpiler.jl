module TRANSPILER

include("../parser/parser.jl")

mutable struct JSCode
    script::Vector{String}
end

function tostring(js::JSCode)
    return replace(join(js.script, ""), Pair(";;", ";"))
end

function transpile(program::PARSER.Program)
    js = JSCode([])

    for stmt in program.statements
        script = jsify_statement(stmt)
        if script !== nothing
            push!(js.script, script)
        end
    end

    return js
end

function jsify_statement(stmt::PARSER.Statement)
    if typeof(stmt) == PARSER.ExpressionStatement
        return jsify_expression(stmt.expression)
    elseif typeof(stmt) == PARSER.VariableStatement
        return jsify_varstatement(stmt)
    elseif typeof(stmt) == PARSER.ReturnStatement
        return jsify_return_statement(stmt)
    elseif typeof(stmt) == PARSER.BlockStatement
        return jsify_blockstatement(stmt)
    elseif typeof(stmt) == PARSER.ConstVariableStatement
        return jsify_const_varstatement(stmt)
    end
end

function jsify_const_varstatement(stmt::PARSER.ConstVariableStatement)
    return "const " * stmt.name.value * " = " * jsify_expression(stmt.value) * ";"
end

function jsify_expression(exp::PARSER.Expression)
    if typeof(exp) == PARSER.IntegerLiteral
        return string(exp.value)
    elseif typeof(exp) == PARSER.PrefixExpression
        return PARSER.tostring(exp) # we already cover this in the parser
    elseif typeof(exp) == PARSER.InfixExpression
        return PARSER.tostring(exp) # we already cover this in the parser
    elseif typeof(exp) == PARSER.IfExpression
        str = "if "
        if typeof(exp.condition) == PARSER.Identifier
            str *= "(" * jsify_expression(exp.condition) * ")"
        else 
            str *= jsify_expression(exp.condition)
        end
        str *= " {" * jsify_statement(exp.consequence) * "}"
        if exp.alternative !== nothing
            if typeof(exp.alternative) == PARSER.BlockStatement
                str *= " else {" * jsify_statement(exp.alternative) * "}"
            elseif typeof(exp.alternative) == PARSER.IfExpression
                str *= " else " * jsify_expression(exp.alternative)
            end
        end
        return str
    elseif typeof(exp) == PARSER.FunctionLiteral
        str = "const " * exp.name.value * " = ("
        for p in exp.paramaters
            str *= jsify_expression(p)
            if p !== exp.paramaters[end]
                str *= ", "
            end
        end
        str *= ") => {" * jsify_statement(exp.body) * "};"
        return str
    elseif typeof(exp) == PARSER.CallExpression
        str = ""
        if typeof(exp.fn) == PARSER.FunctionLiteral
            str = exp.fn.name.value
        else # ident...
            str = exp.fn.value
        end

        str *= "(" * join(jsify_expression.(exp.arguments), ", ") * ")"
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

function jsify_varstatement(stmt::PARSER.VariableStatement)
    return "let " * stmt.name.value * " = " * jsify_expression(stmt.value) * ";"
end

function jsify_return_statement(stmt::PARSER.ReturnStatement)
    return "return " * jsify_expression(stmt.return_value) * ";"
end

function jsify_blockstatement(stmt::PARSER.BlockStatement)
    str = ""
    for stmt in stmt.statements
        str *= jsify_statement(stmt) * "; "
    end
    return str
end

end