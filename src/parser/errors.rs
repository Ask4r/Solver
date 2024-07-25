use crate::colors::{bold_white, bold_yellow};
use std::fmt;

#[derive(Clone, Debug)]
pub struct ParsingError {
    source: String,
    text: String,
    pos: usize,
}

impl ParsingError {
    pub fn new(source: String, text: String, pos: usize) -> Self {
        Self { source, text, pos }
    }
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n{}",
            bold_white(format!("parsing error `{}` at {}", self.text, self.pos)),
            self.source,
            bold_yellow(" ".repeat(self.pos) + &"^".repeat(self.text.len())),
        )
    }
}
