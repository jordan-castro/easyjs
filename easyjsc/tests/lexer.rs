#[cfg(test)]
mod tests {
    use easyjsc::lexer::{lex::{self, read_all_tokens}, token};

    #[test]
    fn test_lexer() {
        let input = r#"
            1 2 3
            !><!=>=<=
            ,(i){}[i]
            +-*/
            //this is a comment
            fn
            :
            =
            ==
            ;
            0..10
            .%""''
            |&
            ???
            struct
            true false if else elif return as javascript{ } in for async await not enum self native
            macro and or pub is import match with break continue null name
        "#;

        let results = vec![
            token::new_token(token::INT, "1", "", 1, 1),
            token::new_token(token::INT, "2", "", 1, 2),
            token::new_token(token::INT, "3", "", 1, 3),
            token::new_token(token::BANG, "!", "", 2, 1),
            token::new_token(token::GT, ">", "", 0, 0),
            token::new_token(token::LT, "<", "", 0, 0),
            token::new_token(token::NOT_EQ, "!=", "", 0, 0),
            token::new_token(token::GT_OR_EQ, ">=", "", 0, 0),
            token::new_token(token::LT_OR_EQ, "<=", "", 0, 0),
            token::new_token(token::COMMA, ",", "", 0, 0),
            token::new_token(token::L_PAREN, "(", "", 0, 0),
            token::new_token(token::IDENT, "i", "", 0, 0),
            token::new_token(token::R_PAREN, ")", "", 0 ,0),
            token::new_token(token::L_BRACE, "{", "", 0, 0),
            token::new_token(token::R_BRACE, "}", "", 0, 0),
            token::new_token(token::L_BRACKET, "[", "", 0, 0),
            token::new_token(token::IDENT, "i", "", 0, 0),
            token::new_token(token::R_BRACKET, "]", "", 0, 0),
            token::new_token(token::PLUS, "+", "", 0, 0),
            token::new_token(token::MINUS, "-", "", 0, 0),
            token::new_token(token::ASTERISK, "*", "", 0 ,0),
            token::new_token(token::SLASH, "/", "", 0, 0),
            token::new_token(token::COMMENT, "//this is a comment", "", 5, 1),
            token::new_token(token::FUNCTION, "fn", "", 0, 0),
            token::new_token(token::COLON, ":", "", 0, 0),
            token::new_token(token::ASSIGN, "=", "", 0, 0),
            token::new_token(token::EQ, "==", "", 0, 0),
            token::new_token(token::SEMICOLON, ";", "", 0, 0),
            token::new_token(token::INT, "0", "", 0, 0),
            token::new_token(token::DOTDOT, "..", "", 0, 0),
            token::new_token(token::INT, "10", "", 0, 0),
            token::new_token(token::DOT, ".", "", 0, 0),
            token::default_token(token::MODULUS, "%"),
            token::default_token(token::STRING, ""),
            token::default_token(token::STRING, ""),
            token::default_token(token::BITWISE_OR, "|"),
            token::default_token(token::BITWISE_AND, "&"),
            token::default_token(token::DOUBLE_QUESTION_MARK, "??"),
            token::default_token(token::QUESTION_MARK, "?"),
            token::default_token(token::STRUCT, "struct"),
            token::default_token(token::TRUE, "true"),
            token::default_token(token::FALSE, "false"),
            token::default_token(token::IF, "if"),
            token::default_token(token::ELSE, "else"),
            token::default_token(token::ELIF, "elif"),
            token::default_token(token::RETURN, "return"),
            token::default_token(token::AS, "as"),
            token::default_token(token::JAVASCRIPT, ""),
            token::default_token(token::IN, "in"),
            token::default_token(token::FOR, "for"),
            token::default_token(token::ASYNC, "async"),
            token::default_token(token::AWAIT, "await"),
            token::default_token(token::NOT, "not"),
            token::default_token(token::ENUM, "enum"),
            token::default_token(token::SELF, "self"),
            token::default_token(token::NATIVE, "native"),
            token::default_token(token::MACRO, "macro"),
            token::default_token(token::AND_SYMBOL, "and"),
            token::default_token(token::OR_SYMBOL, "or"),
            token::default_token(token::PUB, "pub"),
            token::default_token(token::IS, "is"),
            token::default_token(token::MATCH, "match"),
            token::default_token(token::WITH, "with"),
            token::default_token(token::BREAK, "break"),
            token::default_token(token::CONTINUE, "continue"),
            token::default_token(token::NULL, "null"),
            token::default_token(token::IDENT, "name"),
            token::new_token(token::EOF, "\0", "", 0, 0),
        ];

        let tokens = read_all_tokens(input.to_string());
        println!("{:#?}", tokens);
        assert_eq!(tokens.len() ,results.len(), "Lenghts don't match");

        for i in 0..tokens.len() {
            let t = tokens.get(i).unwrap();
            let r = results.get(i);
            assert_ne!(r, None, "Result does not exist");
            let r = r.unwrap();

            assert_eq!(t.typ, r.typ);
            assert_eq!(t.literal, r.literal);
        }

    }
}
