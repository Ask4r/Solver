use clap::{Parser, Subcommand};
use solver_error::SolverError;
use solvers::{integral, root};
use std::{self, process::exit};

pub mod colors;
mod executor;
mod lexer;
mod parser;
pub mod solver_error;
mod solvers;
pub mod tokens;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Evaluate expression
    #[command(arg_required_else_help = true)]
    Eval {
        /// Expression to be evaluated
        expr: String,
        /// Argument `x` value. Not required for expressions without
        /// arguments (expression allowed)
        #[arg(short)]
        x: Option<String>,
    },
    /// Find root of the <EXPR> with variable `x` with `false position`
    /// root-finding algorithm on the interval between <X1> and <X2>
    #[command(arg_required_else_help = true)]
    Root {
        /// Expression with variable `x`
        expr: String,
        /// Value defining interval for root (expression allowed)
        x1: String,
        /// Same as for <X1>
        x2: String,
        /// Root eps. Defaults to 1e-6 (expression allowed)
        #[arg(long)]
        eps: Option<String>,
        /// Max iterations allowed for root evaluation. Defaults to 100,000
        #[arg(long)]
        max_iterations: Option<usize>,
    },
    /// Find definite integral of the <EXPR> with variable `x` with iterative
    /// `rectangular` method on the interval between <X1> and <X2>
    #[command(arg_required_else_help = true)]
    Integral {
        /// Expression with variable `x`
        expr: String,
        /// Value defining interval for integral (expression allowed)
        x1: String,
        /// Same as for <X1>
        x2: String,
        /// Integral eps. Defaults to 1e-6 (expression allowed)
        #[arg(long)]
        eps: Option<String>,
        /// Max iterations allowed for integral evaluation. Defaults to 100,000
        #[arg(long)]
        max_iterations: Option<usize>,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Eval { expr, x } => run_eval(expr, x),
        Commands::Root {
            expr,
            x1,
            x2,
            eps,
            max_iterations,
        } => run_root(expr, x1, x2, eps, max_iterations),
        Commands::Integral {
            expr,
            x1,
            x2,
            eps,
            max_iterations,
        } => run_integral(expr, x1, x2, eps, max_iterations),
    }
}

fn get_expr_closure(expr: &str) -> impl Fn(f64) -> f64 + '_ {
    let tokens_it =
        lexer::analyse(&expr).map(|res| res.map_err(|e| print_error(e, &expr)).unwrap());
    let postfix_tokens = parser::parse(tokens_it)
        .map_err(|e| print_error(e, &expr))
        .unwrap();
    move |x| {
        executor::eval(&postfix_tokens, Some(x))
            .map_err(|e| print_error(e, &expr))
            .unwrap()
    }
}

fn eval_expr(expr: String, x: Option<String>) -> f64 {
    let unwrap_solver = get_solver_unwrapper(&expr);
    let x_value = match x {
        Some(x_expr) => Some(eval_expr(x_expr, None)),
        None => None,
    };
    let tokens_it = lexer::analyse(&expr).map(unwrap_solver);
    let postfix_tokens = parser::parse(tokens_it)
        .map_err(|e| print_error(e, &expr))
        .unwrap();
    return executor::eval(&postfix_tokens, x_value)
        .map_err(|e| print_error(e, &expr))
        .unwrap();
}

fn run_eval(expr: String, x: Option<String>) {
    println!("{}", eval_expr(expr, x));
}

fn print_error(error: impl SolverError, source: &str) {
    eprintln!("{}", error.display_solver_error(source));
    exit(1);
}

fn get_solver_unwrapper<T, E: SolverError>(expr: &str) -> impl Fn(Result<T, E>) -> T + '_ {
    |result| result.map_err(|error| print_error(error, expr)).unwrap()
}

fn run_root(
    expr: String,
    x1: String,
    x2: String,
    eps: Option<String>,
    max_iterations: Option<usize>,
) {
    const ROOT_EPS: f64 = 0.000_001;
    let x1 = eval_expr(x1, None);
    let x2 = eval_expr(x2, None);
    let eps = match eps {
        Some(eps_expr) => eval_expr(eps_expr, None),
        None => ROOT_EPS,
    };
    match root(get_expr_closure(&expr), x1, x2, eps, max_iterations) {
        Some(n) => println!("{}", n),
        None => println!("could not find root"),
    }
}

fn run_integral(
    expr: String,
    x1: String,
    x2: String,
    eps: Option<String>,
    max_iterations: Option<usize>,
) {
    const INTEGRAL_EPS: f64 = 0.000_001;
    let x1 = eval_expr(x1, None);
    let x2 = eval_expr(x2, None);
    let eps = match eps {
        Some(eps_expr) => eval_expr(eps_expr, None),
        None => INTEGRAL_EPS,
    };
    println!(
        "{}",
        integral(get_expr_closure(&expr), x1, x2, eps, max_iterations)
    );
}
