use nv::lexer::lex;
use nv::parser::par;
use nv::compiler::transpile::transpile;

fn main() {
    let vc = nv::utils::version::VERSION_CODE;
    println!("EasyJS current version to support: {vc}");

    let input = "
        x = 1

        fn add(n1,n2) {
            return n1 + n2
        }

        print := console.log

        x = 2

        for true {
            print('my her0')
        }

        for i in 0..10 {
            print(i)
        }

        for i in [0,1,2] {
            print(i)
        }

        for i < 10 {
            
        }

        if i > 10 { 
            print(i)
        } elif (i == 10) {
            print(i)
        } else {
            print(i)
        }
    ".to_string();

    let l = lex::Lex::new(input);
    let mut p = par::Parser::new(l);
    let program = p.parse_program();

    println!("{:?}", p.errors);

    let code = transpile(program, true);
    println!("{code}");
}
