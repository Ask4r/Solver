use crate::tokens::Token;
use errors::ExecutionError;

mod errors;

#[cfg(test)]
mod tests;

pub fn eval<'src>(postfix_list: &Vec<Token<'src>>, x: Option<f64>) -> Result<f64, ExecutionError<'src>> {
        use Token::*;
        use ExecutionError::*;

        let mut stack = Vec::new();
        let mut tokens_it = postfix_list.iter();

        while let Some(tok) = tokens_it.next() {
            match tok {
                Number { value, .. } => stack.push(*value),
                Var { .. } => match x {
                    Some(value) => stack.push(value),
                    None => return Err(MissingArgumentValue { text: "x", pos: 0 }),
                }
                tok if tok.is_const() => stack.push(tok.get_const()),
                UM { pos } => match stack.last_mut() {
                    Some(val) => *val *= -1.0,
                    _ => return Err(UnmatchedOperator { text: "-", pos: *pos }),
                }
                Add { pos } => match (stack.pop(), stack.last_mut()) {
                    (Some(val1), Some(val2)) => *val2 += val1,
                    _ => return Err(UnmatchedOperator { text: "+", pos: *pos }),
                }
                Sub { pos } => match (stack.pop(), stack.last_mut()) {
                    (Some(val1), Some(val2)) => *val2 -= val1,
                    _ => return Err(UnmatchedOperator { text: "-", pos: *pos }),
                }
                Mul { pos } => match (stack.pop(), stack.last_mut()) {
                    (Some(val1), Some(val2)) => *val2 *= val1,
                    _ => return Err(UnmatchedOperator { text: "*", pos: *pos }),
                }
                Div { pos } => match (stack.pop(), stack.last_mut()) {
                    (Some(val1), Some(val2)) => *val2 /= val1,
                    _ => return Err(UnmatchedOperator { text: "/", pos: *pos }),
                }
                Pow { pos } => match (stack.pop(), stack.last_mut()) {
                    (Some(val1), Some(val2)) => *val2 = val2.powf(val1),
                    _ => return Err(UnmatchedOperator { text: "^", pos: *pos }),
                }
                tok if tok.is_func() => (),
                Func { text, func, pos } => match stack.last_mut() {
                    Some(val) => *val = func(*val),
                    _ => return Err(WrongArgs { text, pos: *pos }),
                },
                Comma { .. } => unimplemented!(),
                LParen { .. } | RParen { .. } => (),
            }
        }

        if stack.len() > 1 {
            return Err(MissigOperator { text: "", pos: 0 });
        }

        Ok(stack[0])
    }

