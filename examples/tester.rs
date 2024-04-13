use lexer_test::gen::test::ExprParser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("must pass an expression to be evaluated")
    }

    let input = &args[1];
    let expr = ExprParser::new().parse(input).unwrap();
    println!("{expr:?}");
}

