use crate::colors::{bold_yellow, bold_red};
use crate::solver_error::SolverError;

#[derive(Clone, Debug)]
pub enum ExecutionError<'src> {
    MissingArgumentValue { text: &'src str, pos: usize },
    UnmatchedOperator { text: &'src str, pos: usize },
    MissigOperator { text: &'src str, pos: usize },
    WrongArgs { text: &'src str, pos: usize },
}

impl SolverError for ExecutionError<'_> {
    fn display_solver_error(&self, source: &str) -> String {
        let (msg, text, pos) = match self {
            Self::MissingArgumentValue { text, pos } => ("argument value is required", text, pos),
            Self::UnmatchedOperator { text, pos } => ("missing operand for", text, pos),
            Self::MissigOperator { text, pos } => ("missing operator", text, pos),
            Self::WrongArgs { text, pos } => ("missing operator", text, pos),
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

