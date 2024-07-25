use crate::colors::{bold_white, bold_yellow};
use std::fmt;

#[derive(Clone, Debug)]
pub struct UnknownSymbolError {
    pub source: String,
    pub pos: usize,
    pub symbol: String,
}

impl fmt::Display for UnknownSymbolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n{}",
            bold_white(format!("unknown symbol `{}` at {}", self.symbol, self.pos)),
            self.source,
            bold_yellow(" ".repeat(self.pos) + &"^".repeat(self.symbol.len())),
        )
    }
}
