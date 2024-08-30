use crate::tokens::{Token, TokenType};
use errors::{ExecutionError, ExecutionErrorType};

mod errors;

#[cfg(test)]
mod tests;

pub fn eval<'src>(
    postfix_list: &Vec<Token<'src>>,
    x: Option<f64>,
) -> Result<f64, ExecutionError<'src>> {
    use ExecutionErrorType::*;
    use TokenType::*;

    let mut stack = Vec::new();
    let mut tokens_it = postfix_list.iter();
    let mut args_stack = Vec::new();

    while let Some(token) = tokens_it.next() {
        match token.token_type {
            Number(n) => stack.push(n),
            Var => match x {
                Some(value) => stack.push(value),
                None => return Err(get_execution_err(*token, MissingArgumentValue)),
            },
            Const(n) => stack.push(n),
            UM => match stack.last_mut() {
                Some(val) => *val *= -1.0,
                _ => return Err(get_execution_err(*token, UnmatchedOperator)),
            },
            Add => match (stack.pop(), stack.last_mut()) {
                (Some(val1), Some(val2)) => *val2 += val1,
                _ => return Err(get_execution_err(*token, UnmatchedOperator)),
            },
            Sub => match (stack.pop(), stack.last_mut()) {
                (Some(val1), Some(val2)) => *val2 -= val1,
                _ => return Err(get_execution_err(*token, UnmatchedOperator)),
            },
            Mul => match (stack.pop(), stack.last_mut()) {
                (Some(val1), Some(val2)) => *val2 *= val1,
                _ => return Err(get_execution_err(*token, UnmatchedOperator)),
            },
            Div => match (stack.pop(), stack.last_mut()) {
                (Some(val1), Some(val2)) => *val2 /= val1,
                _ => return Err(get_execution_err(*token, UnmatchedOperator)),
            },
            Pow => match (stack.pop(), stack.last_mut()) {
                (Some(val1), Some(val2)) => *val2 = val2.powf(val1),
                _ => return Err(get_execution_err(*token, UnmatchedOperator)),
            },
            Comma => match stack.pop() {
                Some(val) => args_stack.push(val),
                _ => return Err(get_execution_err(*token, UnmatchedOperator)),
            },
            Func { args, func } => {
                let first_arg = match stack.pop() {
                    Some(val) => val,
                    _ => return Err(get_execution_err(*token, WrongArgs)),
                };
                if args_stack.len() + 1 != args {
                    return Err(get_execution_err(*token, WrongArgs));
                }
                let mut arguments = vec![first_arg];
                args_stack.reverse();
                arguments.append(&mut args_stack.clone());
                args_stack.clear();

                stack.push(func(arguments));
            }
            LParen { .. } | RParen { .. } => (),
        }
    }

    if stack.len() > 1 {
        return Err(ExecutionError::new(0, "", MissigOperator));
    }

    Ok(stack[0])
}

fn get_execution_err(token: Token, error_type: ExecutionErrorType) -> ExecutionError {
    ExecutionError::new(token.pos, token.text, error_type)
}
