#[cfg(test)]
mod tests {
    use easyjs::lexer::token::new_token;
    use easyjs::lexer::{lex, token};
    use easyjs::parser::ast::{Expression, Program};
    use easyjs::parser::{ast, par};

    #[test]
    fn test_vars() {
        let input = "
            x = 5;
            y = 10;
            foobar = 979797;
            nonsemi = 1
        "
        .to_string();
        let l = lex::Lex::new(input);
        let mut p = par::Parser::new(l);
        let program = p.parse_program();

        assert_eq!(p.errors.len(), 0);
        assert_eq!(program.statements.len(), 4);
    }

    #[test]
    fn test_const_vars() {
        let input = "
            x := 5;
            y := 1
            t := 'that'
        "
        .to_string();
        let l = lex::Lex::new(input);
        let mut p = par::Parser::new(l);
        let program = p.parse_program();

        assert_eq!(p.errors.len(), 0);
        assert_eq!(program.statements.len(), 3);

        match program.statements.first().unwrap() {
            ast::Statement::ConstVariableStatement(tk, name, val) => {
                assert_eq!(tk.to_owned().pretty(), new_token("IDENT", "x").pretty());
            }
            _ => panic!("it is not a const var"),
        }
    }

    #[test]
    fn test_math() {
        let input = "
            1 + 9
            2 - 9
            3 / 9
            4 * 9
        "
        .to_string();
        let l: lex::Lex = lex::Lex::new(input);
        let mut p = par::Parser::new(l);
        let program = p.parse_program();

        println!("{:?}", p.errors);
        println!("{:#?}", program.statements);

        assert_eq!(p.errors.len(), 0);
        assert_eq!(program.statements.len(), 4);
    }

    #[test]
    fn test_if() {
        let input = "
        if x > 1 {
            return 'this'
        } elif (x == 0) {
        
        } else {
         
        } 
        "
        .to_string();

        let l: lex::Lex = lex::Lex::new(input);
        let mut p = par::Parser::new(l);
        let program = p.parse_program();

        println!("{:?}", p.errors);
        println!("{:#?}", program.statements);

        assert_eq!(p.errors.len(), 0);
        assert_eq!(program.statements.len(), 1);

        match program.statements.first().unwrap() {
            ast::Statement::ExpressionStatement(token, expression) => match expression.as_ref() {
                ast::Expression::IfExpression(token, condition, consequence, elseif, other) => {}
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    #[test]
    fn test_for() {
        let input = "
            for true {}
            for (true) {}
            for i < 10 {}
            for (i in []) {}
            for i in 0..10 {}
        "
        .to_string();

        let l: lex::Lex = lex::Lex::new(input);
        let mut p = par::Parser::new(l);
        let program = p.parse_program();

        println!("{:?}", p.errors);
        println!("{:#?}", program.statements);

        assert_eq!(p.errors.len(), 0);
        assert_eq!(program.statements.len(), 5);
    }

    #[test]
    fn test_fn() {
        let input = "
        fn foo() {
            return 'bar'
        }

        fn foo_args(arg1, arg2) {
            return arg1 + arg2
        }
        "
        .to_string();

        let l: lex::Lex = lex::Lex::new(input);
        let mut p = par::Parser::new(l);
        let program = p.parse_program();

        println!("{:?}", p.errors);
        println!("{:#?}", program.statements);

        assert_eq!(p.errors.len(), 0);
        assert_eq!(program.statements.len(), 2);
    }

    #[test]
    fn test_lambda() {
        let input = "
            add := fn(n1, n2) {
                return n1 + n2
            }

            minus_one := fn(n1) {
                return n1 - 1
            }

            print := fn() {

            }
        "
        .to_string();

        let l: lex::Lex = lex::Lex::new(input);
        let mut p = par::Parser::new(l);
        let program = p.parse_program();

        println!("{:?}", p.errors);
        println!("{:#?}", program.statements);

        assert_eq!(p.errors.len(), 0);
        assert_eq!(program.statements.len(), 3);
    }

    #[test]
    fn test_dot_expression() {
        let input = "
            ident.call()
            ident.call_with_args(arg1, arg2)
            ident.property
        "
        .to_string();

        let l: lex::Lex = lex::Lex::new(input);
        let mut p = par::Parser::new(l);
        let program = p.parse_program();

        println!("{:?}", p.errors);
        println!("{:#?}", program.statements);

        assert_eq!(p.errors.len(), 0);
        assert_eq!(program.statements.len(), 3);
    }

    #[test]
    fn test_call_expression() {
        let input = "
            call()
            call(x1,x2)
            call(x1)
        "
        .to_string();

        let l: lex::Lex = lex::Lex::new(input);
        let mut p = par::Parser::new(l);
        let program = p.parse_program();

        println!("{:?}", p.errors);
        println!("{:#?}", program.statements);

        assert_eq!(p.errors.len(), 0);
        assert_eq!(program.statements.len(), 3);
    }

    #[test]
    fn test_import_statement() {
        let input = "
            import \"io\" 
            import \"json\" as json
            from 'json' import to_json
            from 'json' import def to_json, from_json, is_json
        "
        .to_string();

        let l: lex::Lex = lex::Lex::new(input);
        let mut p = par::Parser::new(l);
        let program = p.parse_program();

        println!("{:?}", p.errors);
        println!("{:#?}", program.statements);

        assert_eq!(p.errors.len(), 0);
        assert_eq!(program.statements.len(), 4);
    }

    #[test]
    fn test_js() {
        let input = "
            javascript{ 
                let x = 0;
            }
            x = javascript{
                let y = 0;
            }
        "
        .to_string();

        let l: lex::Lex = lex::Lex::new(input);
        let mut p = par::Parser::new(l);
        let program = p.parse_program();

        println!("{:?}", p.errors);
        println!("{:#?}", program.statements);

        assert_eq!(p.errors.len(), 0);
        assert_eq!(program.statements.len(), 2);
    }

    #[test]
    fn test_array() {
        let input = "
        x := [0,1,2,3]
        y = ['1', '2', '3']
        z = []
        "
        .to_string();

        let l: lex::Lex = lex::Lex::new(input);
        let mut p = par::Parser::new(l);
        let program = p.parse_program();

        println!("{:?}", p.errors);
        println!("{:#?}", program.statements);

        assert_eq!(p.errors.len(), 0);
        assert_eq!(program.statements.len(), 3);
    }

    #[test]
    fn test_indexing() {
        let input: String = "
        x[0]
        z[1]
        y[0] = 1
        "
        .to_string();

        let l: lex::Lex = lex::Lex::new(input);
        let mut p = par::Parser::new(l);
        let program = p.parse_program();

        println!("{:?}", p.errors);
        println!("{:#?}", program.statements);

        assert_eq!(p.errors.len(), 0);
        assert_eq!(program.statements.len(), 3);
    }

    #[test]
    fn test_objects() {
        let input: String = "
            {
                name: 'jordan',
                age: 22
            }
            {}
            ee := {
                'hey': 'world'
            }
        "
        .to_string();

        let l: lex::Lex = lex::Lex::new(input);
        let mut p = par::Parser::new(l);
        let program = p.parse_program();

        println!("{:?}", p.errors);
        println!("{:#?}", program.statements);

        assert_eq!(p.errors.len(), 0);
        assert_eq!(program.statements.len(), 3);
    }

    #[test]
    fn test_async() {
        let input: String = "
        async fn test() {}
        async fn() {}
        "
        .to_string();

        let l: lex::Lex = lex::Lex::new(input);
        let mut p = par::Parser::new(l);
        let program = p.parse_program();

        println!("{:?}", p.errors);
        println!("{:#?}", program.statements);

        assert_eq!(p.errors.len(), 0);
        assert_eq!(program.statements.len(), 2);
    }

    #[test]
    fn test_await() {
        let input: String = "
        x = await 1
        "
        .to_string();

        let l: lex::Lex = lex::Lex::new(input);
        let mut p = par::Parser::new(l);
        let program = p.parse_program();

        println!("{:?}", p.errors);
        println!("{:#?}", program.statements);

        assert_eq!(p.errors.len(), 0);
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_assign_expression() {
        let input = "
            x = {name: 1}
            x.name = 2
        ".to_string();

        let l: lex::Lex = lex::Lex::new(input);
        let mut p = par::Parser::new(l);
        let program = p.parse_program();

        println!("{:?}", p.errors);
        println!("{:#?}", program.statements);

        assert_eq!(p.errors.len(), 0);
        assert_eq!(program.statements.len(), 2);
    }
}