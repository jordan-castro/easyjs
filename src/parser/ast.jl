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
A special type of Node. A little different than Statement.
"""
abstract type Expression <: Node end

# Define a method that any Node must implement
function tokenliteral(node::Node)
    error("tokenliteral not implemented for this Node type")
end

# Define a method that any Expression must implement
function tokenliteral(exp::Expression)
    error("tokenliteral not implemented for this Expression type")
end

# Define a method that any Statement must implement
function tokenliteral(state::Statement)
    error("tokenliteral not implemented for this Statement type")
end

# Define a method that any expression must implement
function expressionNode(exp::Expression)
    error("experssionNode not implemented for this Expression type")
end

# Define a concrete struct for Program
mutable struct Program
    statements::Vector{Statement}
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

function tokenliteral(exp::Identifier)
    return exp.token.Literal
end

struct VariableStatement <: Statement # <-- Variable Decleration 
    token::Lexer.Token # <-- "="
    name::Identifier
    value::Expression
end

function tokenliteral(vs::VariableStatement)
    return vs.token.Literal
end

struct ReturnStatement <: Statement
    token::Lexer.Token
    return_value::Expression
end