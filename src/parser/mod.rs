use crate::tokens::Token;
use errors::ParsingError;

mod errors;

#[cfg(test)]
mod tests;

pub struct Parser {
    source: String,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(source: String, tokens: Vec<Token>) -> Self {
        Self { source, tokens }
    }

    pub fn build(&mut self) -> Result<Vec<Token>, ParsingError> {
        use Token::*;
        let mut op_stack: Vec<Token> = Vec::new();
        let mut postfix_list: Vec<Token> = Vec::new();

        for token in self.tokens.iter() {
            match token {
                Number { .. } | Const { .. } | Var { .. } => postfix_list.push(token.clone()),
                LParen { .. } | Func { .. } => op_stack.push(token.clone()),
                RParen { pos: op_pos } => loop {
                    match op_stack.pop() {
                        Some(LParen { .. }) => break,
                        Some(item) => postfix_list.push(item),
                        None => return Err(self.error_parsing(")".into(), *op_pos)),
                    }
                },
                EOF => (),
                _ => loop {
                    match op_stack.last() {
                        Some(oper) if Self::get_prec(oper) >= Self::get_prec(&token) => {
                            postfix_list.push(oper.clone());
                            op_stack.pop();
                        }
                        _ => break op_stack.push(token.clone()),
                    }
                },
            }
        }

        op_stack.reverse();
        postfix_list.extend(op_stack);
        Ok(postfix_list)
    }

    fn get_prec(token: &Token) -> u64 {
        match token {
            Token::UM { .. } => 5,
            Token::Caret { .. } => 4,
            Token::Star { .. } | Token::Slash { .. } => 3,
            Token::Plus { .. } | Token::Minus { .. } => 2,
            Token::RParen { .. } | Token::Comma { .. } => 1,
            Token::LParen { .. } => 0,
            _ => 10,
        }
    }

    fn error_parsing(&self, text: &str, pos: usize) -> ParsingError {
        ParsingError::new(self.source.clone(), text.into(), pos)
    }
}

