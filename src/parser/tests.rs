use super::*;
use crate::lexer::analyse;
use TokenType::*;

#[test]
fn it_works() {
    let text = "2 + 2 * sin(3 ^ -3)";
    let tokens_it = analyse(text).map(|res| res.unwrap());
    let postfix_list = parse(tokens_it).unwrap();
    println!("{:#?}", postfix_list);

    assert_eq!(
        postfix_list[0],
        Token {
            pos: 0,
            text: "2",
            token_type: Number(2.0),
        }
    );
    assert_eq!(
        postfix_list[1],
        Token {
            pos: 4,
            text: "2",
            token_type: Number(2.0)
        }
    );
    assert_eq!(
        postfix_list[2],
        Token {
            pos: 12,
            text: "3",
            token_type: Number(3.0)
        }
    );
    assert_eq!(
        postfix_list[3],
        Token {
            pos: 17,
            text: "3",
            token_type: Number(3.0)
        }
    );
    assert_eq!(
        postfix_list[4],
        Token {
            pos: 16,
            text: "-",
            token_type: UM
        }
    );
    assert_eq!(
        postfix_list[5],
        Token {
            pos: 14,
            text: "^",
            token_type: Pow
        }
    );

    assert!(matches!(
        postfix_list[6],
        Token {
            pos: 8,
            text: "sin",
            token_type: Func { args: 1, .. }
        }
    ));

    assert_eq!(
        postfix_list[7],
        Token {
            pos: 6,
            text: "*",
            token_type: Mul
        }
    );
    assert_eq!(
        postfix_list[8],
        Token {
            pos: 2,
            text: "+",
            token_type: Add
        }
    );
}
