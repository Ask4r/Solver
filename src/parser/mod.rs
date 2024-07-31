use crate::tokens::Token;
use errors::{EvaluationError, ParsingError};

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

    pub fn parse<I>(&mut self, tokens_it: I) -> Result<Vec<Token<'src>>, ParsingError<'src>>
    where I: Iterator<Item=Token<'src>>{
        use Token::*;
        let mut op_stack: Vec<Token> = Vec::new();
        let mut postfix_list: Vec<Token> = Vec::new();

        for token in tokens_it {
            match token {
                Number { .. } | Const { .. } | Var { .. } => postfix_list.push(token),
                LParen { .. } | Func { .. } => op_stack.push(token),
                RParen { pos } => loop {
                    match op_stack.pop() {
                        Some(LParen { .. }) => break,
                        Some(item) => postfix_list.push(item),
                        None => return Err(ParsingError::new("unmatched parenthesis", self.source, ")", pos)),
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
                LParen { pos } => return Err(ParsingError::new("unmatched parenthesis", self.source, "(", pos)),
                _ => postfix_list.push(op),
            }
        }

        Ok(postfix_list)
    }

    pub fn eval(&self, postfix_list: &Vec<Token<'src>>, x: Option<f64>) -> Result<f64, EvaluationError<'src>> {
        use Token::*;

        let mut stack = Vec::new();
        let mut tokens_it = postfix_list.iter();

        while let Some(tok) = tokens_it.next() {
            match tok {
                Number { value, .. } | Const { value, .. } => stack.push(*value),
                Var { .. } => match x {
                    Some(value) => stack.push(value),
                    None => return Err(EvaluationError::new("argument value is required", self.source, self.source, 0)),
                }
                UM { pos } => match stack.last_mut() {
                    Some(val) => *val *= -1.0,
                    _ => return Err(EvaluationError::new("unmatched operator", self.source, "-", *pos)),
                }
                Add { pos } => match (stack.pop(), stack.last_mut()) {
                    (Some(val1), Some(val2)) => *val2 += val1,
                    _ => return Err(EvaluationError::new("unmatched operator", self.source, "+", *pos)),
                }
                Sub { pos } => match (stack.pop(), stack.last_mut()) {
                    (Some(val1), Some(val2)) => *val2 -= val1,
                    _ => return Err(EvaluationError::new("unmatched operator", self.source, "-", *pos)),
                }
                Mul { pos } => match (stack.pop(), stack.last_mut()) {
                    (Some(val1), Some(val2)) => *val2 *= val1,
                    _ => return Err(EvaluationError::new("unmatched operator", self.source, "*", *pos)),
                }
                Div { pos } => match (stack.pop(), stack.last_mut()) {
                    (Some(val1), Some(val2)) => *val2 /= val1,
                    _ => return Err(EvaluationError::new("unmatched operator", self.source, "/", *pos)),
                }
                Pow { pos } => match (stack.pop(), stack.last_mut()) {
                    (Some(val1), Some(val2)) => *val2 = val2.powf(val1),
                    _ => return Err(EvaluationError::new("unmatched operator", self.source, "^", *pos)),
                }
                Func { text, func, pos } => match stack.last_mut() {
                    Some(val) => *val = func(*val),
                    _ => return Err(EvaluationError::new("expected arg for func", self.source, text, *pos)),
                },
                Comma { .. } => unimplemented!(),
                LParen { .. } | RParen { .. } | EOF => (),
            }
        }

        if stack.len() > 1 {
            return Err(EvaluationError::new("missing operator", self.source, self.source, 0));
        }

        Ok(stack[0])
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

