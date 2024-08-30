use crate::colors::{bold_red, bold_yellow};
use crate::solver_error::SolverError;

#[derive(Clone, Debug)]
pub struct AnalyseError<'src> {
    pos: usize,
    text: &'src str,
    error_type: AnalyseErrorType,
}

#[derive(Clone, Debug)]
pub enum AnalyseErrorType {
    WrongNumber,
    UnknownIdent,
    UnknownSymbol,
}

impl<'src> AnalyseError<'src> {
    pub fn new(pos: usize, text: &'src str, error_type: AnalyseErrorType) -> Self {
        Self {
            pos,
            text,
            error_type,
        }
    }
}

impl SolverError for AnalyseError<'_> {
    fn display_solver_error(&self, source: &str) -> String {
        use AnalyseErrorType::*;

        let msg = match self.error_type {
            WrongNumber => "could not parse number",
            UnknownIdent => "unknown identifier",
            UnknownSymbol => "unknown symbol",
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
