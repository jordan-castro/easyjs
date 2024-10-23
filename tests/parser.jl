using Test

include("../src/parser/parser.jl")

input = "
x = 5
y = 10
foobar = 838383
"

lexer = PARSER.Lexer.Lex(input, 1, 1, ' ')
p = PARSER.newparser(lexer)
program = PARSER.parseprogram!(p)

@test length(program.statements) == 3

input = "
return 4
return 5
return 1212121
"

lexer = PARSER.Lexer.Lex(input, 1, 1, ' ')
p = PARSER.newparser(lexer)
program = PARSER.parseprogram!(p)

@test length(program.statements) == 3
@test PARSER.tokenliteral(program.statements[1]) == "return"
@test PARSER.tokenliteral(program.statements[2]) == "return"
@test PARSER.tokenliteral(program.statements[3]) == "return"
@test typeof(program.statements[1]) == PARSER.ReturnStatement
@test typeof(program.statements[3]) == PARSER.ReturnStatement
@test typeof(program.statements[2]) == PARSER.ReturnStatement

input = "foobar"
lexer = PARSER.Lexer.Lex(input, 1, 1, ' ')
p = PARSER.newparser(lexer)
program = PARSER.parseprogram!(p)

@test length(program.statements) == 1
@test typeof(program.statements[1]) == PARSER.ExpressionStatement
@test typeof(program.statements[1].expression) == PARSER.Identifier

input = "1"
lexer = PARSER.Lexer.Lex(input, 1, 1, ' ')
p = PARSER.newparser(lexer)
program = PARSER.parseprogram!(p)

@test length(program.statements) == 1
@test typeof(program.statements[1]) == PARSER.ExpressionStatement
@test typeof(program.statements[1].expression) == PARSER.IntegerLiteral

input = "!5
-15"
lexer = PARSER.Lexer.Lex(input, 1, 1, ' ')
p = PARSER.newparser(lexer)
program = PARSER.parseprogram!(p)

@test length(program.statements) == 2
@test typeof(program.statements[1]) == PARSER.ExpressionStatement
@test typeof(program.statements[1].expression) == PARSER.PrefixExpression
@test program.statements[1].expression.operator == "!"
@test program.statements[2].expression.operator == "-"

input = "
5 + 5
5 - 5
5 * 5
5 / 5
5 > 5
5 < 5
5 == 5
5 != 5
"
lexer = PARSER.Lexer.Lex(input, 1, 1, ' ')
p = PARSER.newparser(lexer)
program = PARSER.parseprogram!(p)

@test typeof(program.statements[1]) == PARSER.ExpressionStatement
@test typeof(program.statements[1].expression) == PARSER.InfixExpression