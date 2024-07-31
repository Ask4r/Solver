use crate::colors::{bold_white, bold_yellow};
use std::fmt;

#[derive(Clone, Debug)]
pub struct ParsingError<'src> {
    msg: &'static str,
    source: &'src str,
    text: &'src str,
    pos: usize,
}

impl<'src> ParsingError<'src> {
    pub fn new(msg: &'static str, source: &'src str, text: &'src str, pos: usize) -> Self {
        Self { msg, source, text, pos }
    }
}

impl fmt::Display for ParsingError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let brief = format!("{} `{}` at {}", self.msg, self.text, self.pos);
        let cursor = " ".repeat(self.pos) + &"^".repeat(self.text.len());
        write!(f, "{}\n{}\n{}", bold_white(brief), self.source, bold_yellow(cursor))
    }
}

#[derive(Clone, Debug)]
pub struct EvaluationError<'src> {
    msg: &'static str,
    source: &'src str,
    text: &'src str,
    pos: usize,
}

impl<'src> EvaluationError<'src> {
    pub fn new(msg: &'static str, source: &'src str, text: &'src str, pos: usize) -> Self {
        Self { msg, source, text, pos }
    }
}

impl fmt::Display for EvaluationError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let brief = format!("{} `{}` at {}", self.msg, self.text, self.pos);
        let cursor = " ".repeat(self.pos) + &"^".repeat(self.text.len());
        write!(f, "{}\n{}\n{}", bold_white(brief), self.source, bold_yellow(cursor))
    }
}

