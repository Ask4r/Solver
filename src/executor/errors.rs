use crate::colors::{bold_red, bold_yellow};
use crate::solver_error::SolverError;

#[derive(Clone, Debug)]
pub struct ExecutionError<'src> {
    text: &'src str,
    pos: usize,
    error_type: ExecutionErrorType,
}

#[derive(Clone, Debug)]
pub enum ExecutionErrorType {
    MissingArgumentValue,
    UnmatchedOperator,
    MissigOperator,
    WrongArgs,
}

impl<'src> ExecutionError<'src> {
    pub fn new(pos: usize, text: &'src str, error_type: ExecutionErrorType) -> Self {
        Self {
            pos,
            text,
            error_type,
        }
    }
}

impl SolverError for ExecutionError<'_> {
    fn display_solver_error(&self, source: &str) -> String {
        use ExecutionErrorType::*;

        let msg = match self.error_type {
            MissingArgumentValue => "argument value is required",
            UnmatchedOperator => "missing operand for",
            MissigOperator => "missing operator",
            WrongArgs => "wrong arguments for",
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
