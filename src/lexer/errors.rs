use crate::colors::{bold_white, bold_yellow};
use std::fmt;

pub enum LexerError {
    EOF,
    Unknown(char),
}

#[derive(Clone, Debug)]
pub struct UnknownSymbolError<'a> {
    pub source: &'a str,
    pub pos: usize,
    pub symbol: char,
}

impl fmt::Display for UnknownSymbolError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n{}",
            bold_white(format!("unknown symbol `{}` at {}", self.symbol, self.pos)),
            self.source,
            bold_yellow(" ".repeat(self.pos) + "^"),
        )
    }
}
