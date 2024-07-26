use super::*;
use crate::lexer::Lexer;
use Token::*;

#[test]
fn it_works() {
    let text = "2 + 2 * sin(3 ^ -3)";
    let mut lexer = Lexer::new(text.into());
    let tokens = lexer.parse().unwrap();
    let mut parser = Parser::new(text.into(), tokens);
    let parsed_toks = parser.build().unwrap();

    assert_eq!(parsed_toks[0], Number { text: "2".into(), value: 2.0, pos: 0 });
    assert_eq!(parsed_toks[1], Number { text: "2".into(), value: 2.0, pos: 4 });
    assert_eq!(parsed_toks[2], Number { text: "3".into(), value: 3.0, pos: 12 });
    assert_eq!(parsed_toks[3], Number { text: "3".into(), value: 3.0, pos: 17 });
    assert_eq!(parsed_toks[4], UM { pos: 16 });
    assert_eq!(parsed_toks[5], Caret { pos: 14 });
    assert_eq!(parsed_toks[6], Func { text: "sin".into(), func: f64::sin, pos: 8 });
    assert_eq!(parsed_toks[7], Star { pos: 6 });
    assert_eq!(parsed_toks[8], Plus { pos: 2 });
}
