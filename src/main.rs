use clap::{Parser, Subcommand};
use colors::bold_red;
use solvers::{root, integral};
use std::{self, process::exit};

pub mod colors;
mod lexer;
mod parser;
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
    },
    /// Find root of the expression with variable `x` with `false position`
    /// root-finding algorithm in the range `x1` to `x2` (expressions allowed)
    #[command(arg_required_else_help = true)]
    Root {
        /// Expression with variable `x`
        expr: String,
        /// Expression defining interval for root
        x1: String,
        /// / ------------- ^^^^^ ------------- /
        x2: String,
    },
    /// Find definite integral of the function with variable `x`
    /// bounded by `x1` and `x2` (expressions allowed)
    #[command(arg_required_else_help = true)]
    Integral {
        /// Expression with variable `x`
        expr: String,
        /// Expression defining interval for integral
        x1: String,
        /// / --------------- ^^^^^ --------------- /
        x2: String,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Eval { expr } => eval(expr),
        Commands::Root { expr, x1, x2 } => find_root(expr, x1, x2),
        Commands::Integral { expr, x1, x2 } => find_integral(expr, x1, x2),
    }
}

fn exec_expr(expr: String) -> f64 {
    let tokens_it = lexer::Lexer::new(&expr).map(|res|
        res.map_err(print_err).unwrap()
    );
    let mut parser = parser::Parser::new(&expr);
    let postfix_tokens = parser.parse(tokens_it).map_err(print_err).unwrap();
    return parser.eval(&postfix_tokens).map_err(print_err).unwrap();
}

fn eval(expr: String) {
    println!("{}", exec_expr(expr));
}

fn find_root(expr: String, x1: String, x2: String) {
    const ROOT_EPS: f64 = 0.000_001;

    let x1 = exec_expr(x1);
    let x2 = exec_expr(x2);

    let tokens_it = lexer::Lexer::new(&expr).map(|res|
        res.map_err(print_err).unwrap()
    );
    let mut parser = parser::Parser::new(&expr);
    let postfix_tokens = parser.parse(tokens_it).map_err(print_err).unwrap();
    let f = |x| parser.calc(&postfix_tokens, x).map_err(print_err).unwrap();

    match root(f, x1, x2, ROOT_EPS) {
        Some(n) => println!("{}", n),
        None => println!("could not find root"),
    }
}

fn find_integral(expr: String, x1: String, x2: String) {
    const INTEGRAL_EPS: f64 = 0.000_001;

    let x1 = exec_expr(x1);
    let x2 = exec_expr(x2);

    let tokens_it = lexer::Lexer::new(&expr).map(|res|
        res.map_err(print_err).unwrap()
    );
    let mut parser = parser::Parser::new(&expr);
    let postfix_tokens = parser.parse(tokens_it).map_err(print_err).unwrap();
    let f = |x| parser.calc(&postfix_tokens, x).map_err(print_err).unwrap();

    println!("{}", integral(f, x1, x2, INTEGRAL_EPS));
}

fn print_err<T: std::fmt::Display>(error: T) {
    eprintln!("{}: {}", bold_red("Error"), error);
    exit(1);
}

