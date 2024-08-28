use crate::tokens::Token;
use errors::ParsingError;

mod errors;

#[cfg(test)]
mod tests;

pub fn parse<'src>(
    tokens_it: impl Iterator<Item = Token<'src>>,
) -> Result<Vec<Token<'src>>, ParsingError<'src>> {
    use ParsingError::*;
    use Token::*;
    let mut op_stack: Vec<Token> = Vec::new();
    let mut postfix_list: Vec<Token> = Vec::new();

    for token in tokens_it {
        match token {
            tok if tok.is_operand() => postfix_list.push(token),
            tok if tok.is_function() || matches!(tok, LParen { .. }) => op_stack.push(token),
            RParen { pos } => loop {
                match op_stack.pop() {
                    Some(LParen { .. }) => break,
                    Some(item) => postfix_list.push(item),
                    None => return Err(UnmatchedParenthesis { text: ")", pos }),
                }
            },
            _ => loop {
                match op_stack.last() {
                    Some(oper) if get_prec(oper) >= get_prec(&token) => {
                        postfix_list.push(*oper);
                        op_stack.pop();
                    }
                    _ => break op_stack.push(token),
                }
            },
        }
    }

    while let Some(op) = op_stack.pop() {
        match op {
            LParen { pos } => return Err(UnmatchedParenthesis { text: "(", pos }),
            _ => postfix_list.push(op),
        }
    }

    Ok(postfix_list)
}

fn get_prec(token: &Token) -> u64 {
    match token {
        Token::UM { .. } => 5,
        Token::Pow { .. } => 4,
        Token::Mul { .. } | Token::Div { .. } => 3,
        Token::Add { .. } | Token::Sub { .. } => 2,
        Token::RParen { .. } | Token::Comma { .. } => 1,
        Token::LParen { .. } => 0,
        _ => 10,
    }
}
