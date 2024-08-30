use super::*;
use crate::lexer;
use crate::parser;

#[test]
fn it_works() {
    let text = "-2 * e^sin(3.5)";

    let tokens = lexer::analyse(text).map(|res| res.unwrap());
    let postfix_tokens = parser::parse(tokens).unwrap();
    let result = eval(&postfix_tokens, None).unwrap();
    let expected = -1.40827;

    assert!(f64::abs(result - expected) < 0.000_01);
}
