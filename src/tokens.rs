#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy)]
pub enum Token<'a> {
    Add { pos: usize },
    Sub { pos: usize },
    Mul { pos: usize },
    UM { pos: usize },
    Div { pos: usize },
    Pow { pos: usize },
    LParen { pos: usize },
    RParen { pos: usize },
    Comma { pos: usize },
    Var { pos: usize },
    Number { text: &'a str, value: f64, pos: usize },
    Const { text: &'a str, value: f64, pos: usize },
    Func { text: &'a str, func: fn(f64) -> f64, pos: usize },
    EOF,
}
