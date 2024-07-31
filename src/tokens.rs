#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy)]
pub enum Token<'src> {
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
    Number { text: &'src str, value: f64, pos: usize },
    Const { text: &'src str, value: f64, pos: usize },
    Func { text: &'src str, func: fn(f64) -> f64, pos: usize },
    EOF,
}

impl<'src> Token<'src> {
    pub fn to_text(&self) -> &'src str {
        match self {
            Self::Add { .. } => "+",
            Self::Sub { .. } => "-",
            Self::Mul { .. } => "*",
            Self::UM { .. } => "-",
            Self::Div { .. } => "/",
            Self::Pow { .. } => "^",
            Self::LParen { .. } => "(",
            Self::RParen { .. } => ")",
            Self::Comma { .. } => ",",
            Self::Var { .. } => "x",
            Self::Number { text, .. }
            | Self::Const { text, .. }
            | Self::Func { text, .. } => text,
            Self::EOF => "",
        }
    }
}
