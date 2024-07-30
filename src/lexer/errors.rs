use crate::colors::{bold_white, bold_yellow};
use std::fmt;


#[derive(Clone, Debug)]
pub struct AnalyseError<'src> {
    msg: &'static str,
    source: &'src str,
    occurance: &'src str,
    pos: usize,
}

impl<'src> AnalyseError<'src> {
    pub fn new(msg: &'static str, source: &'src str, occurance: &'src str, pos: usize) -> Self {
        Self { msg, source, occurance, pos }
    }
}

impl fmt::Display for AnalyseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let brief = format!("{} `{}` at {}", self.msg, self.occurance, self.pos);
        let cursor = " ".repeat(self.pos) + &"^".repeat(self.occurance.len());
        write!(f, "{}\n{}\n{}", bold_white(brief), self.source, bold_yellow(cursor))
    }
}
