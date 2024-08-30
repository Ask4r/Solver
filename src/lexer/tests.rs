use core::panic;

use super::*;
use TokenType::*;

#[test]
fn it_works() {
    let text = "-2 * sin(x)";
    let mut tokens = analyse(text).map(|res| res.unwrap());

    assert_eq!(
        tokens.next().unwrap(),
        Token {
            pos: 0,
            text: "-",
            token_type: Sub,
        }
    );
    assert_eq!(
        tokens.next().unwrap(),
        Token {
            pos: 1,
            text: "2",
            token_type: Number(2.0),
        }
    );
    assert_eq!(
        tokens.next().unwrap(),
        Token {
            pos: 3,
            text: "*",
            token_type: Mul,
        }
    );

    assert!(matches!(
        tokens.next().unwrap(),
        Token {
            pos: 5,
            text: "sin",
            token_type: Func { args: 1, .. }
        }
    ));

    assert_eq!(
        tokens.next().unwrap(),
        Token {
            pos: 8,
            text: "(",
            token_type: LParen,
        }
    );
    assert_eq!(
        tokens.next().unwrap(),
        Token {
            pos: 9,
            text: "x",
            token_type: Var,
        }
    );
    assert_eq!(
        tokens.next().unwrap(),
        Token {
            pos: 10,
            text: ")",
            token_type: RParen,
        }
    );
}

#[test]
fn funcs_calls() {
    let text1 = "sin(x)";
    let mut tokens1 = analyse(text1).map(|res| res.unwrap());
    let sin_token = tokens1.next().unwrap();
    let sin_func = match sin_token.token_type {
        Func { args: 1, func } => func,
        _ => panic!("wrong function parsing"),
    };
    let args1 = vec![0.0];
    assert_eq!(sin_func(args1), 0.0);

    let text2 = "max(1, 2)";
    let mut tokens2 = analyse(text2).map(|res| res.unwrap());
    let max_token = tokens2.next().unwrap();
    let max_func = match max_token.token_type {
        Func { args: 2, func } => func,
        _ => panic!("wrong function parsing"),
    };
    let args2 = vec![1.0, 2.0];
    assert_eq!(max_func(args2), 2.0);
}
