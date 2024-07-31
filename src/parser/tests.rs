use std::f64;

use super::*;
use crate::lexer::Lexer;
use Token::*;


#[test]
fn it_works() {
    let text = "-2 * e^sin(3.5)";
    let tokens_it = Lexer::new(text).map(|res| res.unwrap());
    let mut parser = Parser::new(text);

    let postfix_list = parser.parse(tokens_it).unwrap();
    let result = parser.eval(&postfix_list, None).unwrap();
    let expected = -1.40827;

    assert!(f64::abs(result - expected) < 0.000_01);
}

#[test]
fn parsing_works() {
    let text = "2 + 2 * sin(3 ^ -3)";
    let tokens_it = Lexer::new(text).map(|res| res.unwrap());
    let postfix_list = Parser::new(text).parse(tokens_it).unwrap();

    assert_eq!(postfix_list[0], Number { text: "2".into(), value: 2.0, pos: 0 });
    assert_eq!(postfix_list[1], Number { text: "2".into(), value: 2.0, pos: 4 });
    assert_eq!(postfix_list[2], Number { text: "3".into(), value: 3.0, pos: 12 });
    assert_eq!(postfix_list[3], Number { text: "3".into(), value: 3.0, pos: 17 });
    assert_eq!(postfix_list[4], UM { pos: 16 });
    assert_eq!(postfix_list[5], Pow { pos: 14 });
    assert_eq!(postfix_list[6], Func { text: "sin".into(), func: f64::sin, pos: 8 });
    assert_eq!(postfix_list[7], Mul { pos: 6 });
    assert_eq!(postfix_list[8], Add { pos: 2 });
}

