module TRANSPILER

include("../parser/parser.jl")
include("import.jl")

mutable struct JSCode
    script::Vector{String} # <-- Line by Line JS.
    variables::Vector{String} # <-- All variables that have been declared.
    fns::Vector{String} # <-- All functions that have been declared.
    import_paths::Vector{String} # <-- All paths that have been imported.
    imports::Vector{String} # <-- All imported code.
end

function transpile_from_input(input::String)
    l = PARSER.Lexer.Lex(input, 1, 1, ' ')
    p = PARSER.newparser(l)
    program = PARSER.parseprogram!(p)

    return tostring(transpile!(program))
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
    src = ""

    for im in js.imports
        src *= im * ";"
    end

    joiner = pretty ? "\n" : ""
    return src *= replace(join(js.script, joiner), Pair(";;", ";"))
end

function transpile!(program::PARSER.Program, js::JSCode = JSCode([], [], [], [], []))
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
    elseif typeof(stmt) == PARSER.ImportStatement
        return  jsify_import_statement!(js, stmt)
    end
end

function jsify_const_varstatement!(js::JSCode, stmt::PARSER.ConstVariableStatement)
    return "const " * stmt.name.value * " = " * jsify_expression!(js, stmt.value) * ";"
end

function jsify_expression!(js::JSCode, exp::PARSER.Expression)
    if typeof(exp) == PARSER.IntegerLiteral
        return string(exp.value)
    elseif typeof(exp) == PARSER.StringLiteral
        return "\"" * exp.value * "\""
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
                str *= ","
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
                str *= ","
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
    else
        return ""
    end
end

function jsify_varstatement!(js::JSCode, stmt::PARSER.VariableStatement)
    # check if js.variables contains varname
    if stmt.name.value in js.variables
        return stmt.name.value * "=" * jsify_expression!(js, stmt.value) * ";"
    else
        push!(js.variables, stmt.name.value)
        return "let " * stmt.name.value * "=" * jsify_expression!(js, stmt.value) * ";"
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

function jsify_import_statement!(js::JSCode, stmt::PARSER.ImportStatement)
    # check if js.imports contains path
    if stmt.path in js.import_paths
        # already been imported do nothing
        return nothing
    end

    # add to imports
    push!(js.import_paths, stmt.path)

    import_type = EJImport.file_type(stmt.path)

    if import_type == EJImport.EJ
    end

    # # check what kind of import this is.
    # if !occursin(".", stmt.path)
    #     # this is a STD library import
    #     return ""
    # end

    # # if this is a .ej file
    # if endswith(stmt.path, ".ej")
    #     f = open(stmt.path, "r")
    #     contents = read(f, String)
    #     close(f)

    #     # transpile to JS
    #     code = transpile_from_input(contents)
    #     push!(js.imports, code)
    # end

    return nothing
end

end