#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy)]
pub struct Token<'src> {
    pub pos: usize,
    pub text: &'src str,
    pub token_type: TokenType,
}

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy)]
pub enum TokenType {
    Number(f64),
    Var,
    Add,
    Sub,
    Mul,
    UM,
    Div,
    Pow,
    LParen,
    RParen,
    Comma,
    Const(f64),
    Func {
        args: usize,
        func: fn(Vec<f64>) -> f64,
    },
}

impl<'src> Token<'src> {
    pub fn new(pos: usize, text: &'src str, token_type: TokenType) -> Self {
        Self {
            pos,
            text,
            token_type,
        }
    }
}
