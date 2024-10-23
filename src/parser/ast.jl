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

function tostring(exp::DefaultDontUseExpression)
    print("Error this expression should not be used.")
end

function tokenliteral(exp::DefaultDontUseExpression)
    print("Error this expression should not be used.")
end

# Define a concrete struct for Program
mutable struct Program
    statements::Vector{Statement}
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

struct Identifier <: Expression
    token::Lexer.Token # <-- The IDENT token
    value::String
end

function tostring(id::Identifier)
    return id.value
end

function tokenliteral(exp::Identifier)
    return exp.token.Literal
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

function tostring(vs::VariableStatement)
    return vs.token.Literal * " = " * vs.value
end

function tokenliteral(vs::VariableStatement)
    return vs.token.Literal
end

"""
return variable
"""
struct ReturnStatement <: Statement
    token::Lexer.Token # <-- The Return Token
    return_value::Expression # <-- the value.
end

function tostring(rs::ReturnStatement)
    return rs.token.Literal * " " * rs.return_value
end

function tokenliteral(rs::ReturnStatement)
    return rs.token.Literal
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

function tostring(irs::ImpliedReturnStatement)
    return irs.token.Literal
end

function tokenliteral(irs::ImpliedReturnStatement)
    return irs.token.Literal
end

struct ExpressionStatement <: Statement
    token::Lexer.Token # <-- The first token of the expression
    expression::Expression # <-- Expression is a Node (I just don't want to add a new type for that shit.)
end

function tostring(exps::ExpressionStatement)
    return tostring(exps.expression)
end

function tokenliteral(exps::ExpressionStatement)
    return exps.token.Literal
end

struct IntegerLiteral <: Expression
    token::Lexer.Token
    value::Int64
end

function tostring(il::IntegerLiteral)
    return il.token.Literal
end

function tokenliteral(il::IntegerLiteral)
    return il.token.Literal
end

struct PrefixExpression <: Expression
    token::Lexer.Token
    operator::String
    right::Expression # <-- Expression
end

function tostring(pe::PrefixExpression)
    return "(" * pe.operator * tostring(pe.right) * ")"
end

function tokenliteral(pe::PrefixExpression)
    return pe.token.Literal
end

struct InfixExpression <: Expression
    token::Lexer.Token
    left::Expression
    operator::String
    right::Expression
end

function tostring(ie::InfixExpression) 
    return "(" * tostring(ie.left) * " " * ie.operator * " " * tostring(ie.right) * ")"
end