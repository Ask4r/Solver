use std::process::exit;

pub fn print_error(error: impl SolverError, source: &str) {
    eprintln!("{}", error.display_solver_error(source));
    exit(1);
}

fn get_solver_unwrapper<T, E: SolverError>(expr: &str) -> impl Fn(Result<T, E>) -> T + '_ {
    move |result| result.map_err(|error| print_error(error, expr)).unwrap()
}

pub trait SolverError {
    fn display_solver_error(&self, source: &str) -> String;
}
