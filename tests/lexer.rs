#[cfg(test)]
mod tests {
    use easyjs::lexer::{lex, token};

    #[test]
    fn test_lexer() {
        let input = "
            1 2 3
            !><!=>=<=
            ,(i){}[i]
            +-*/
            //this is a comment
            fn
            :
            ::
            :=
            =
            ==
            ;
            0..10
        ";
        let results = vec![
            token::new_token(token::INT, "1"),
            token::new_token(token::INT, "2"),
            token::new_token(token::INT, "3"),
            token::new_token(token::BANG, "!"),
            token::new_token(token::GT, ">"),
            token::new_token(token::LT, "<"),
            token::new_token(token::NOT_EQ, "!="),
            token::new_token(token::GT_OR_EQ, ">="),
            token::new_token(token::LT_OR_EQ, "<="),
            token::new_token(token::COMMA, ","),
            token::new_token(token::L_PAREN, "("),
            token::new_token(token::IDENT, "i"),
            token::new_token(token::R_PAREN, ")"),
            token::new_token(token::L_BRACE, "{"),
            token::new_token(token::R_BRACE, "}"),
            token::new_token(token::L_BRACKET, "["),
            token::new_token(token::IDENT, "i"),
            token::new_token(token::R_BRACKET, "]"),
            token::new_token(token::PLUS, "+"),
            token::new_token(token::MINUS, "-"),
            token::new_token(token::ASTERISK, "*"),
            token::new_token(token::SLASH, "/"),
            token::new_token(token::COMMENT, "this is a comment"),
            token::new_token(token::FUNCTION, "fn"),
            token::new_token(token::COLON, ":"),
            token::new_token(token::TYPE, "::"),
            token::new_token(token::CONST_ASSIGNMENT, ":="),
            token::new_token(token::ASSIGN, "="),
            token::new_token(token::EQ, "=="),
            token::new_token(token::SEMICOLON, ";"),
            token::new_token(token::INT, "0"),
            token::new_token(token::DOTDOT, ".."),
            token::new_token(token::INT, "10"),
            token::new_token(token::EOF, "\0")
        ];

        assert_eq!(lex::read_all_tokens(input.to_string()), results);
    }
    
}