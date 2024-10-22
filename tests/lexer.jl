using Test

include("../src/lexer/lexer.jl")

program = "five = 5
ten = 10
fn add(x, y) {
    x + y
}
result = add(five, ten)
!-/*5
5 < 10 > 5
if 5 < 10 {
    return true
} else {
    false
}
10 == 10
10 != 9
"

tokens = Lexer.readalltokens(program)

shouldbe = [
    Lexer.Token(Lexer.IDENT, "five"),  # five
    Lexer.Token(Lexer.ASSIGN, "="),    # =
    Lexer.Token(Lexer.INT, "5"),       # 5
    Lexer.Token(Lexer.LINE_BREAK, "\n"),
    Lexer.Token(Lexer.IDENT, "ten"),   # ten
    Lexer.Token(Lexer.ASSIGN, "="),    # =
    Lexer.Token(Lexer.INT, "10"),      # 10
    Lexer.Token(Lexer.LINE_BREAK, "\n"),

    Lexer.Token(Lexer.FUNCTION, "fn"), # fn
    Lexer.Token(Lexer.IDENT, "add"),   # add
    Lexer.Token(Lexer.L_PAREN, "("),   # (
    Lexer.Token(Lexer.IDENT, "x"),     # x
    Lexer.Token(Lexer.COMMA, ","),     # ,
    Lexer.Token(Lexer.IDENT, "y"),     # y
    Lexer.Token(Lexer.R_PAREN, ")"),   # )
    Lexer.Token(Lexer.L_BRACE, "{"),   # {
    Lexer.Token(Lexer.LINE_BREAK, "\n"),

    Lexer.Token(Lexer.IDENT, "x"),     # x
    Lexer.Token(Lexer.PLUS, "+"),      # +
    Lexer.Token(Lexer.IDENT, "y"),     # y
    Lexer.Token(Lexer.LINE_BREAK, "\n"),

    Lexer.Token(Lexer.R_BRACE, "}"),   # }
    Lexer.Token(Lexer.LINE_BREAK, "\n"),

    Lexer.Token(Lexer.IDENT, "result"),# result
    Lexer.Token(Lexer.ASSIGN, "="),    # =
    Lexer.Token(Lexer.IDENT, "add"),   # add
    Lexer.Token(Lexer.L_PAREN, "("),   # (
    Lexer.Token(Lexer.IDENT, "five"),  # five
    Lexer.Token(Lexer.COMMA, ","),     # ,
    Lexer.Token(Lexer.IDENT, "ten"),   # ten
    Lexer.Token(Lexer.R_PAREN, ")"),   # )
    Lexer.Token(Lexer.LINE_BREAK, "\n"),

    Lexer.Token(Lexer.BANG, "!"),      # !
    Lexer.Token(Lexer.MINUS, "-"),     # -
    Lexer.Token(Lexer.SLASH, "/"),     # /
    Lexer.Token(Lexer.ASTERISK, "*"),  # *
    Lexer.Token(Lexer.INT, "5"),       # 5
    Lexer.Token(Lexer.LINE_BREAK, "\n"),

    Lexer.Token(Lexer.INT, "5"),       # 5
    Lexer.Token(Lexer.LT, "<"),        # <
    Lexer.Token(Lexer.INT, "10"),      # 10
    Lexer.Token(Lexer.GT, ">"),        # >
    Lexer.Token(Lexer.INT, "5"),       # 5
    Lexer.Token(Lexer.LINE_BREAK, "\n"),

    Lexer.Token(Lexer.IF, "if"),       # if
    Lexer.Token(Lexer.INT, "5"),       # 5
    Lexer.Token(Lexer.LT, "<"),        # <
    Lexer.Token(Lexer.INT, "10"),      # 10
    Lexer.Token(Lexer.L_BRACE, "{"),   # {
    Lexer.Token(Lexer.LINE_BREAK, "\n"),
    Lexer.Token(Lexer.RETURN, "return"),# return
    Lexer.Token(Lexer.TRUE, "true"),   # true
    Lexer.Token(Lexer.LINE_BREAK, "\n"),
    Lexer.Token(Lexer.R_BRACE, "}"),   # }

    Lexer.Token(Lexer.ELSE, "else"),   # else
    Lexer.Token(Lexer.L_BRACE, "{"),   # {
    Lexer.Token(Lexer.LINE_BREAK, "\n"),

    Lexer.Token(Lexer.FALSE, "false"), # false
    Lexer.Token(Lexer.LINE_BREAK, "\n"),
    Lexer.Token(Lexer.R_BRACE, "}"),   # }
    Lexer.Token(Lexer.LINE_BREAK, "\n"),

    Lexer.Token(Lexer.INT, "10"),      # 10
    Lexer.Token(Lexer.EQ, "=="),       # ==
    Lexer.Token(Lexer.INT, "10"),      # 10
    Lexer.Token(Lexer.LINE_BREAK, "\n"),

    Lexer.Token(Lexer.INT, "10"),      # 10
    Lexer.Token(Lexer.NOT_EQ, "!="),   # !=
    Lexer.Token(Lexer.INT, "9"),       # 9
    Lexer.Token(Lexer.LINE_BREAK, "\n"),

    Lexer.Token(Lexer.EOF, "EOF")      # EOF
]

for (i, token) in enumerate(tokens)
    expected = shouldbe[i]
    @test token.Type == expected.Type 
    @test token.Literal == expected.Literal 
end

println("Lexer tests passed")