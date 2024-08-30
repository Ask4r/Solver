use std::process::exit;

pub fn print_error(error: impl SolverError, source: &str) {
    eprintln!("{}", error.display_solver_error(source));
    exit(1);
}

pub trait SolverError {
    fn display_solver_error(&self, source: &str) -> String;
}
