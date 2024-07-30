use crate::tokens::Token;
use errors::ParsingError;

mod errors;

#[cfg(test)]
mod tests;

pub struct Parser<'src> {
    source: &'src str,
}

impl<'src> Parser<'src> {
    pub fn new(source: &'src str) -> Self {
        Self { source }
    }

    pub fn build<I>(&mut self, tokens_it: I) -> Result<Vec<Token<'src>>, ParsingError<'src>>
    where I: Iterator<Item=Token<'src>>{
        use Token::*;
        let mut op_stack: Vec<Token> = Vec::new();
        let mut postfix_list: Vec<Token> = Vec::new();

        for token in tokens_it {
            match token {
                Number { .. } | Const { .. } | Var { .. } => postfix_list.push(token),
                LParen { .. } | Func { .. } => op_stack.push(token),
                RParen { pos: op_pos } => loop {
                    match op_stack.pop() {
                        Some(LParen { .. }) => break,
                        Some(item) => postfix_list.push(item),
                        None => return Err(ParsingError::new("unmatched parenthesis", self.source, ")", op_pos)),
                    }
                },
                EOF => (),
                _ => loop {
                    match op_stack.last() {
                        Some(oper) if Self::get_prec(oper) >= Self::get_prec(&token) => {
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
                LParen { pos: op_pos } => return Err(ParsingError::new("unmatched parenthesis", self.source, "(", op_pos)),
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
}

