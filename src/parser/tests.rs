use super::*;
use crate::lexer::analyse;
use Token::*;


#[test]
fn it_works() {
    let text = "2 + 2 * sin(3 ^ -3)";
    let tokens_it = analyse(text).map(|res| res.unwrap());
    let postfix_list = parse(tokens_it).unwrap();

    assert_eq!(postfix_list[0], Number { text: "2".into(), value: 2.0, pos: 0 });
    assert_eq!(postfix_list[1], Number { text: "2".into(), value: 2.0, pos: 4 });
    assert_eq!(postfix_list[2], Number { text: "3".into(), value: 3.0, pos: 12 });
    assert_eq!(postfix_list[3], Number { text: "3".into(), value: 3.0, pos: 17 });
    assert_eq!(postfix_list[4], UM { pos: 16 });
    assert_eq!(postfix_list[5], Pow { pos: 14 });
    assert_eq!(postfix_list[6], Sin { pos: 8 });
    assert_eq!(postfix_list[7], Mul { pos: 6 });
    assert_eq!(postfix_list[8], Add { pos: 2 });
}

