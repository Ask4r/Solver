use crate::colors::{bold_yellow, bold_red};
use crate::solver_error::SolverError;

#[derive(Clone, Debug)]
pub enum AnalyseError<'src> {
    WrongNumber { text: &'src str, pos: usize },
    UnknownIdent { text: &'src str, pos: usize },
    UnknownSymbol { text: &'src str, pos: usize },
}

impl SolverError for AnalyseError<'_> {
    fn display_solver_error(&self, source: &str) -> String {
        let (msg, text, pos) = match self {
            Self::WrongNumber { text, pos } => ("could not parse number", text, pos),
            Self::UnknownIdent { text, pos } => ("unknown identifier", text, pos),
            Self::UnknownSymbol { text, pos } => ("unknown symbol", text, pos),
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

