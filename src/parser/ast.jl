include("../lexer/lexer.jl")

"""
Building block in EasyJS language.
"""
abstract type Node end

"""
A special type of Node.
"""
abstract type Statement <: Node end

"""
For organizing. Really this is not necessary.
"""
abstract type Expression <: Node end

# A default expression to not have errors
struct DefaultDontUseExpression <: Expression
end

struct Identifier <: Expression
    token::Lexer.Token # <-- The IDENT token
    value::String
end

"""
x = 4
str = ""
b = false
"""
struct VariableStatement <: Statement # <-- Variable Decleration 
    token::Lexer.Token # <-- IDENT this is the variable identification
    name::Identifier # <-- This is also the IDENT variable Decleration, but we keep this to work with Thorston Balls implementation...
    value::Expression # <-- The value of the variable
end

"""
return variable
"""
struct ReturnStatement <: Statement
    token::Lexer.Token # <-- The Return Token
    return_value::Expression # <-- the value.
end

"""
```
... <-- this is fn code.

x <-- This is how you can also return a value in EasyJS

}
```
"""
struct ImpliedReturnStatement <: Statement
    token::Lexer.Token # <-- This is the actual value. (can be ident, int, etc)
end

struct ExpressionStatement <: Statement
    token::Lexer.Token # <-- The first token of the expression
    expression::Expression # <-- Expression is a Node (I just don't want to add a new type for that shit.)
end

struct PrefixExpression <: Expression
    token::Lexer.Token
    operator::String
    right::Expression # <-- Expression
end

struct IntegerLiteral <: Expression
    token::Lexer.Token
    value::Int64
end

struct InfixExpression <: Expression
    token::Lexer.Token
    left::Expression
    operator::String
    right::Expression
end

struct Boolean <: Expression
    token::Lexer.Token
    value::Bool
end

"""
A {} block
"""
struct BlockStatement <: Statement
    token::Lexer.Token # <-- The { token
    statements::Array{Statement} # <-- A list of statements within this block.
end

struct IfExpression <: Expression
    token::Lexer.Token # <-- The 'if' token
    condition::Expression
    consequence::BlockStatement
    alternative::Union{BlockStatement, Nothing, IfExpression} # <-- A IfExpression can either be by itself, have a else, or a elseif.
end

struct FunctionLiteral <: Expression
    token::Lexer.Token # <-- The 'fn' token.
    name::Identifier # <-- The name of the function.
    paramaters::Array{Identifier}
    body::Union{BlockStatement, Nothing}
end

struct CallExpression <: Expression
    token::Lexer.Token # <-- The '(' token
    fn::Expression # <-- Identifier or FunctionLiteral
    arguments::Array{Expression}
end

# Define a concrete struct for Program
mutable struct Program
    statements::Vector{Statement}
end

function tostring(exp::DefaultDontUseExpression)
    print("Error this expression should not be used.")
end

function tokenliteral(exp::Expression)
    return exp.token.Literal
end

function tokenliteral(stmt::Statement)
    return stmt.token.Literal
end

function tostring(program::Program)
    p = ""
    for statement in program.statements
        p = p * tostring(statement) * "\n"
    end

    return p
end

# Implement the tokenliteral method for Program
function tokenliteral(program::Program)
    if !isempty(program.statements)
        return tokenliteral(program.statements[1])  # Call tokenliteral for the first statement
    end
    return ""
end

function tostring(id::Identifier)
    return id.value
end

function tostring(vs::VariableStatement)
    return vs.token.Literal * " = " * tostring(vs.value)
end

function tostring(rs::ReturnStatement)
    return rs.token.Literal * " " * tostring(rs.return_value)
end

function tostring(irs::ImpliedReturnStatement)
    return irs.token.Literal
end

function tostring(exps::ExpressionStatement)
    return tostring(exps.expression)
end

function tostring(il::IntegerLiteral)
    return il.token.Literal
end

function tostring(pe::PrefixExpression)
    return "(" * pe.operator * tostring(pe.right) * ")"
end

function tostring(ie::InfixExpression) 
    return "(" * tostring(ie.left) * " " * ie.operator * " " * tostring(ie.right) * ")"
end

function tostring(b::Boolean)
    return b.token.Literal
end

function tostring(block::BlockStatement)
    str = ""
    for stmt in block.statements
        str *=  tostring(stmt)
    end

    return str
end

function tostring(i::IfExpression)
    str =  "if " * tostring(i.condition) * " " * tostring(i.consequence)

    if i.alternative isa BlockStatement || i.alternative isa IfExpression
        str *= " else " * tostring(i.alternative)
    end

    return str
end

function tostring(fn::FunctionLiteral)
    params = []

    for p in fn.paramaters
        push!(params, tostring(p))
    end

    str = tokenliteral(fn)
    str *= " " * tostring(fn.name)
    str *= "("
    str *= join(params, ", ")
    str *= ")"
    str *= tostring(fn.body)

    return str
end

function tostring(c::CallExpression)
    args = []
    for a in c.arguments
        push!(args, tostring(a))
    end

    return tostring(c.fn) * "(" * join(args, ", ") * ")"    
end