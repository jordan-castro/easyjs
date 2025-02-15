using Test

include("../src/parser/parser.jl")

input = "
x = 5;
y = 10;
foobar = 838383;
"

lexer = PARSER.Lexer.Lex(input, 1, 1, ' ')
p = PARSER.newparser(lexer)
program = PARSER.parseprogram!(p)

@test length(program.statements) == 3
@test program.statements[1].value.value == 5

input = "
return 4;
return 5;
return 1212121;
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

input = "!5;
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
5 + 5;
5 - 5;
5 * 5;
5 / 5;
5 > 5;
5 < 5;
5 == 5;
5 != 5;
"
lexer = PARSER.Lexer.Lex(input, 1, 1, ' ')
p = PARSER.newparser(lexer)
program = PARSER.parseprogram!(p)

@test typeof(program.statements[1]) == PARSER.ExpressionStatement
@test typeof(program.statements[1].expression) == PARSER.InfixExpression

input = "
false;
x = true;
y = false;
"
lexer = PARSER.Lexer.Lex(input, 1, 1, ' ')
p = PARSER.newparser(lexer)
program = PARSER.parseprogram!(p)

@test typeof(program.statements[1]) == PARSER.ExpressionStatement
@test typeof(program.statements[1].expression) == PARSER.Boolean
@test program.statements[1].expression.value == false
@test typeof(program.statements[2]) == PARSER.VariableStatement
# @test typeof(program.statements[2].name) == PARSER.Identifier

input = "
if (x < y) { 
    return x
} elif (y > x) { 
 return x * y 
} else { return y; }
if cc > 12 { 
    return s 
}
"
lexer = PARSER.Lexer.Lex(input, 1, 1, ' ')
p = PARSER.newparser(lexer)
program = PARSER.parseprogram!(p)

@test length(program.statements) == 2
@test typeof(program.statements[1]) == PARSER.ExpressionStatement
@test typeof(program.statements[1].expression) == PARSER.IfExpression

input = "
fn foo(x, y) {
    x + y
}
"
lexer = PARSER.Lexer.Lex(input, 1, 1, ' ')
p = PARSER.newparser(lexer)
program = PARSER.parseprogram!(p)

@test length(program.statements) == 1
@test typeof(program.statements[1]) == PARSER.ExpressionStatement
@test typeof(program.statements[1].expression) == PARSER.FunctionLiteral
@test typeof(program.statements[1].expression.name) == PARSER.Identifier
@test program.statements[1].expression.name.value == "foo"

input = "
add(1, 2)
"
lexer = PARSER.Lexer.Lex(input, 1, 1, ' ')
p = PARSER.newparser(lexer)
program = PARSER.parseprogram!(p)

@test length(program.statements) == 1
@test typeof(program.statements[1]) == PARSER.ExpressionStatement
@test typeof(program.statements[1].expression) == PARSER.CallExpression

input = "
\"hello world\"
"
lexer = PARSER.Lexer.Lex(input, 1, 1, ' ')
p = PARSER.newparser(lexer)
program = PARSER.parseprogram!(p)

@test length(program.statements) == 1
@test typeof(program.statements[1]) == PARSER.ExpressionStatement
@test typeof(program.statements[1].expression) == PARSER.StringLiteral

input = "
//comment
"
lexer = PARSER.Lexer.Lex(input, 1, 1, ' ')
p = PARSER.newparser(lexer)
program = PARSER.parseprogram!(p)

@test length(program.statements) == 1
@test typeof(program.statements[1]) == PARSER.ExpressionStatement
@test typeof(program.statements[1].expression) == PARSER.Comment
@test program.statements[1].expression.value == "comment"

input = "
import \"http\"

import \"otherfile.ej\" as ot
"
lexer = PARSER.Lexer.Lex(input, 1, 1, ' ')
p = PARSER.newparser(lexer)
program = PARSER.parseprogram!(p)

@test length(program.statements) == 2
@test typeof(program.statements[1]) == PARSER.ImportStatement
@test program.statements[1].path == "http"
@test program.statements[1].as == ""
@test typeof(program.statements[2]) == PARSER.ImportStatement
@test program.statements[2].path == "otherfile.ej"
@test program.statements[2].as == "ot"

input = "
console.log(\"hello world\")
"
lexer = PARSER.Lexer.Lex(input, 1, 1, ' ')
p = PARSER.newparser(lexer)
program = PARSER.parseprogram!(p)
@test length(program.statements) == 1
@test typeof(program.statements[1]) == PARSER.ExpressionStatement
@test typeof(program.statements[1].expression) == PARSER.DotExpression

input = "
javascript{
    const x = 1 + 1;
    const obj = {
        name: \"jj\"
    };
}

name = javascript{
    \"jj\";
}
"
lexer = PARSER.Lexer.Lex(input, 1, 1, ' ')
p = PARSER.newparser(lexer)
program = PARSER.parseprogram!(p)

@test length(program.statements) == 2
@test typeof(program.statements[1]) == PARSER.JavaScriptStatement
@test typeof(program.statements[2]) == PARSER.VariableStatement

input = "
    boo := fn() {
        console.log(\"yeyo\")     
    }

    far := fn(n1,n2) {
        return n1 + n2
    }
"
lexer = PARSER.Lexer.Lex(input, 1, 1, ' ')
p = PARSER.newparser(lexer)
program = PARSER.parseprogram!(p)

@test length(program.statements) == 2
@test typeof(program.statements[1]) == PARSER.ConstVariableStatement
@test typeof(program.statements[1].value) == PARSER.LambdaLiteral
@test program.statements[1].value.token.Type == PARSER.Lexer.FUNCTION
@test typeof(program.statements[1].value.body) == PARSER.BlockStatement
@test length(p.errors) == 0

input = "
    array := [1,2,3]
    empty_array = []

    // indexing on an array and replacing it's value.
    array[0] = 1
"
lexer = PARSER.Lexer.Lex(input, 1, 1, ' ')
p = PARSER.newparser(lexer)
program = PARSER.parseprogram!(p)

@test length(program.statements) == 4
@test typeof(program.statements[1]) == PARSER.ConstVariableStatement
@test typeof(program.statements[1].value) == PARSER.ArrayLiteral
@test program.statements[1].value.token.Type == PARSER.Lexer.L_BRACKET
@test typeof(program.statements[4]) == PARSER.ExpressionStatement
@test typeof(program.statements[4].expression) == PARSER.IndexExpression

input = "
    {}
    {
        name: \"jj\",
        age: 1
    }

    obj_foo := fn() {
        return {
            \"yeyo\": 1
        }
    }
"
lexer = PARSER.Lexer.Lex(input, 1, 1, ' ')
p = PARSER.newparser(lexer)
program = PARSER.parseprogram!(p)

@test length(program.statements) == 3

input = "
    async fn hello() {
        console.log(\"hello world\")
    }

    say_eyo := async fn() {
        console.log(\"eyo\")
    }
"
lexer = PARSER.Lexer.Lex(input, 1, 1, ' ')
p = PARSER.newparser(lexer)
program = PARSER.parseprogram!(p)

@test length(program.statements) == 2
@test typeof(program.statements[1]) == PARSER.ExpressionStatement
@test typeof(program.statements[2]) == PARSER.ConstVariableStatement
@test typeof(program.statements[1].expression) == PARSER.AsyncExpression
@test typeof(program.statements[2].value) == PARSER.AsyncExpression

input = "
    for i in 0..10 {}
    for true {}
    for i < 10 {}
"
lexer = PARSER.Lexer.Lex(input, 1, 1, ' ')
p = PARSER.newparser(lexer)
program = PARSER.parseprogram!(p)

if length(p.errors) > 0
    println(p.errors)
end
@test length(p.errors) == 0
@test length(program.statements) == 3
@test typeof(program.statements[1]) == PARSER.ForStatement
@test typeof(program.statements[1].condition) == PARSER.InExpression
@test typeof(program.statements[1].condition.left) == PARSER.Identifier
@test typeof(program.statements[1].condition.right) == PARSER.RangeExpression

input = "
    x = await 1
"
lexer = PARSER.Lexer.Lex(input, 1, 1, ' ')
p = PARSER.newparser(lexer)
program = PARSER.parseprogram!(p)

if length(p.errors) > 0
    println(p.errors)
end
@test length(p.errors) == 0
@test length(program.statements) == 1

input = "\n"
tokens = PARSER.Lexer.readalltokens(input)

@test length(tokens) == 1
@test tokens[1] == PARSER.Lexer.EOL