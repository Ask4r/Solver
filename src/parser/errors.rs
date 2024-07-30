use crate::colors::{bold_white, bold_yellow};
use std::fmt;

#[derive(Clone, Debug)]
pub struct ParsingError<'src> {
    source: &'src str,
    text: &'src str,
    pos: usize,
}

impl<'src> ParsingError<'src> {
    pub fn new(source: &'src str, text: &'src str, pos: usize) -> Self {
        Self { source, text, pos }
    }
}

impl fmt::Display for ParsingError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let brief = format!("parsing error `{}` at {}", self.text, self.pos);
        let cursor = " ".repeat(self.pos) + &"^".repeat(self.text.len());
        write!(f, "{}\n{}\n{}", bold_white(brief), self.source, bold_yellow(cursor))
    }
}
