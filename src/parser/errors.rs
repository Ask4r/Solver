use crate::colors::{bold_yellow, bold_red};
use crate::solver_error::SolverError;

#[derive(Clone, Debug)]
pub enum ParsingError<'src> {
    UnmatchedParenthesis { text: &'src str, pos: usize },
}

impl SolverError for ParsingError<'_> {
    fn display_solver_error(&self, source: &str) -> String {
        let (msg, text, pos) = match self {
            Self::UnmatchedParenthesis { text, pos } => ("unmatched parethesis", text, pos),
        };

        let brief = format!("{} `{}` at {}", msg, text, pos);
        let cursor = " ".repeat(*pos) + &"^".repeat(text.len());

        format!(
            "{}: {}\n{}\n{}",
            bold_red("error"),
            brief,
            source,
            bold_yellow(cursor),
        )
    }
}

