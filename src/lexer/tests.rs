use super::*;
use Token::*;

#[test]
fn it_works() {
    let text = "-2 * sin(x)";
    let mut tokens_it = Lexer::new(text).map(|res| res.unwrap());

    assert_eq!(tokens_it.next().unwrap(), UM { pos: 0 });
    assert_eq!(tokens_it.next().unwrap(), Number { text: "2", value: 2.0, pos: 1 });
    assert_eq!(tokens_it.next().unwrap(), Mul { pos: 3 });
    assert_eq!(tokens_it.next().unwrap(), Func { text: "sin", func: f64::sin, pos: 5 });
    assert_eq!(tokens_it.next().unwrap(), LParen { pos: 8 });
    assert_eq!(tokens_it.next().unwrap(), Var { pos: 9 });
    assert_eq!(tokens_it.next().unwrap(), RParen { pos: 10 });
}
