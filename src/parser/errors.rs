use crate::colors::{bold_red, bold_yellow};
use crate::solver_error::SolverError;

#[derive(Clone, Debug)]
pub struct ParsingError<'src> {
    pos: usize,
    text: &'src str,
    error_type: ParsingErrorType,
}

#[derive(Clone, Debug)]
pub enum ParsingErrorType {
    UnmatchedParenthesis,
}

impl<'src> ParsingError<'src> {
    pub fn new(pos: usize, text: &'src str, error_type: ParsingErrorType) -> Self {
        Self {
            pos,
            text,
            error_type,
        }
    }
}

impl SolverError for ParsingError<'_> {
    fn display_solver_error(&self, source: &str) -> String {
        use ParsingErrorType::*;

        let msg = match self.error_type {
            UnmatchedParenthesis => "unmatched parethesis",
        };

        let brief = format!("{} `{}` at {}", msg, self.text, self.pos);
        let cursor = " ".repeat(self.pos) + &"^".repeat(self.text.len());

        format!(
            "{}: {}\n{}\n{}",
            bold_red("error"),
            brief,
            source,
            bold_yellow(cursor),
        )
    }
}
