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
    Eval { expr: String },
    /// Find root of the expression with variable `x` in the range
    /// `x1` to `x2` (expressions allowed)
    #[command(arg_required_else_help = true)]
    Root {
        expr: String,
        x1: String,
        x2: String,
    },
    /// Find definite integral of the function with variable `x`
    /// bounded by `x1` and `x2` (expressions allowed)
    #[command(arg_required_else_help = true)]
    Integral {
        expr: String,
        x1: String,
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

fn eval(expr: String) {
    println!("Text: `{}`", expr);

    let tokens_it = lexer::Lexer::new(&expr).map(|res| res.map_err(print_err).unwrap());
    let parsed_tokens = parser::Parser::new(&expr).build(tokens_it).map_err(print_err).unwrap();

    for tok in parsed_tokens {
        println!("{:?}", tok)
    }
}

fn find_root(expr: String, x1: String, x2: String) {
    println!("Expr: {}", expr);
    println!("Domain: ({};{})", x1, x2);
    println!("Root (mock): {}", root(|x| x * 2.0, -5.0, 5.0, 0.000_001).unwrap())
}

fn find_integral(expr: String, x1: String, x2: String) {
    println!("Expr: {}", expr);
    println!("Domain: ({};{})", x1, x2);
    println!("Integral (mock): {}", integral(|x| 3.0 * x * x, 0.0, 5.0, 0.000_000_1))
}

fn print_err<T: std::fmt::Display>(error: T) {
    eprintln!("{}: {}", bold_red("Error"), error);
    exit(1);
}

