using Test

include("../src/lexer/ast.jl")

println("x = 123")
tokens = AST.readalltokens("x = 123")
@test length(tokens) == 3
@test tokens[1] == AST.Lexer.TK.newtoken(AST.Lexer.TK.IDENT, "x")
@test tokens[2] == AST.Lexer.TK.newtoken(AST.Lexer.TK.ASSIGN, "=")
@test tokens[3] == AST.Lexer.TK.newtoken(AST.Lexer.TK.INT, "123")