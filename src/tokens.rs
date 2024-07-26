#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Plus { pos: usize },
    Minus { pos: usize },
    Star { pos: usize },
    UM { pos: usize },
    Slash { pos: usize },
    Caret { pos: usize },
    LParen { pos: usize },
    RParen { pos: usize },
    Comma { pos: usize },
    Var { pos: usize },
    Number { text: String, value: f64, pos: usize },
    Const { text: String, value: f64, pos: usize },
    Func { text: String, func: fn(f64) -> f64, pos: usize },
    EOF,
}
