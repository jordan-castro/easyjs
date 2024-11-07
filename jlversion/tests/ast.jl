using Test

include("../src/parser/parser.jl")

input = "
x = 5
y = 10
foobar = 838383

return 4
"

lexer = PARSER.Lexer.Lex(input, 1, 1, ' ')
p = PARSER.newparser(lexer)
program = PARSER.parseprogram!(p)

# TODO: finish