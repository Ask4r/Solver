use super::*;

#[test]
fn it_works() {
    let text = "-2 * e^sin(3.5)";

    let mut lexer = Lexer::new(text);
    let mut parser = Parser::new(text);
    let mut executor = Executor::new(text);

    let tokens_it = Lexer::new(text).map(|res| res.unwrap());
    let mut parser = Parser::new(text);
    let postfix_list = parser.parse(tokens_it).unwrap();

    let result = parser.eval(&postfix_list, None).unwrap();
    let expected = -1.40827;

    assert!(f64::abs(result - expected) < 0.000_01);
}
