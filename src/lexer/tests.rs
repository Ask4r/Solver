use super::*;
use Token::*;

#[test]
fn it_works() {
    let text = "-2 * sin(x)";
    let mut lex = Lexer::new(text.into());
    let tokens = lex.parse().unwrap();

    assert_eq!(tokens[0], UM { pos: 0 });
    assert_eq!(tokens[1], Number { text: "2".into(), value: 2.0, pos: 1 });
    assert_eq!(tokens[2], Star { pos: 3 });
    assert_eq!(tokens[3], Func { text: "sin".into(), func: f64::sin, pos: 5 });
    assert_eq!(tokens[4], LParen { pos: 8 });
    assert_eq!(tokens[5], Var { pos: 9 });
    assert_eq!(tokens[6], RParen { pos: 10 });
}
