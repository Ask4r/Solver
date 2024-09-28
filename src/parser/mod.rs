use crate::tokens::{Token, TokenType};
use errors::{ParsingError, ParsingErrorType};

mod errors;

#[cfg(test)]
mod tests;

pub fn parse<'src, I>(tokens: I) -> Result<Vec<Token<'src>>, ParsingError<'src>>
where
    I: Iterator<Item = Token<'src>>,
{
    use ParsingErrorType::*;
    use TokenType::*;

    let mut tokens_it = tokens.peekable();
    let mut operator_stack: Vec<Token> = Vec::new();
    let mut postfix_list: Vec<Token> = Vec::new();
    let mut prev_token: Option<Token> = None;

    while let Some(token) = tokens_it.next() {
        match token.token_type {
            Number(_) | Var | Const(_) => postfix_list.push(token),
            Func { .. } => match tokens_it.peek() {
                Some(Token {
                    token_type: LParen, ..
                }) => operator_stack.push(token),
                _ => {
                    return Err(ParsingError::new(
                        token.pos,
                        token.text,
                        UnmatchedParenthesis,
                    ))
                }
            },
            LParen => operator_stack.push(token),
            RParen => loop {
                match operator_stack.pop() {
                    Some(Token {
                        token_type: LParen, ..
                    }) => break,
                    Some(tok) => postfix_list.push(tok),
                    None => return Err(ParsingError::new(token.pos, ")", UnmatchedParenthesis)),
                }
            },
            _ => {
                let actual_token = match token.token_type {
                    Sub if !matches!(
                        prev_token,
                        Some(Token {
                            token_type: Number(_) | Var | Const(_) | RParen,
                            ..
                        })
                    ) =>
                    {
                        Token::new(token.pos, token.text, UM)
                    }
                    _ => token,
                };
                loop {
                    match operator_stack.last() {
                        Some(operator) if get_prec(operator) >= get_prec(&actual_token) => {
                            postfix_list.push(operator_stack.pop().unwrap());
                        }
                        _ => break operator_stack.push(actual_token),
                    }
                }
                prev_token = Some(actual_token);
                continue;
            }
        };
        prev_token = Some(token);
    }

    while let Some(op) = operator_stack.pop() {
        match op.token_type {
            LParen => return Err(ParsingError::new(op.pos, "(", UnmatchedParenthesis)),
            _ => postfix_list.push(op),
        }
    }

    Ok(postfix_list)
}

fn get_prec(token: &Token) -> u64 {
    use TokenType::*;
    match token.token_type {
        UM => 5,
        Pow => 4,
        Mul | Div => 3,
        Add | Sub => 2,
        RParen | Comma => 1,
        LParen => 0,
        _ => 10,
    }
}
