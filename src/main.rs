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
        Commands::Eval { expr, x } => cmd_eval(expr, x),
        Commands::Root { expr, x1, x2, eps, max_iterations } =>
            cmd_root(expr, x1, x2, eps, max_iterations),
        Commands::Integral { expr, x1, x2, eps, max_iterations } =>
            cmd_integral(expr, x1, x2, eps, max_iterations),
    }
}

fn eval_expr(expr: String, x: Option<String>) -> f64 {
    let x_value = match x {
        Some(x_expr) => Some(eval_expr(x_expr, None)),
        None => None,
    };
    let tokens_it = lexer::Lexer::new(&expr).map(|res|
        res.map_err(print_err).unwrap()
    );
    let mut parser = parser::Parser::new(&expr);
    let postfix_tokens = parser.parse(tokens_it).map_err(print_err).unwrap();
    return parser.eval(&postfix_tokens, x_value).map_err(print_err).unwrap();
}

fn cmd_eval(expr: String, x: Option<String>) {
    println!("{}", eval_expr(expr, x));
}

fn cmd_root(expr: String, x1: String, x2: String,
            eps: Option<String>, max_iterations: Option<usize>) {
    const ROOT_EPS: f64 = 0.000_001;

    let x1 = eval_expr(x1, None);
    let x2 = eval_expr(x2, None);
    let eps = match eps {
        Some(eps_expr) => eval_expr(eps_expr, None),
        None => ROOT_EPS,
    };

    let tokens_it = lexer::Lexer::new(&expr).map(|res|
        res.map_err(print_err).unwrap()
    );
    let mut parser = parser::Parser::new(&expr);
    let postfix_tokens = parser.parse(tokens_it).map_err(print_err).unwrap();
    let f = |x| parser.eval(&postfix_tokens, Some(x)).map_err(print_err).unwrap();

    match root(f, x1, x2, eps, max_iterations) {
        Some(n) => println!("{}", n),
        None => println!("could not find root"),
    }
}

fn cmd_integral(expr: String, x1: String, x2: String,
                eps: Option<String>, max_iterations: Option<usize>) {
    const INTEGRAL_EPS: f64 = 0.000_001;

    let x1 = eval_expr(x1, None);
    let x2 = eval_expr(x2, None);
    let eps = match eps {
        Some(eps_expr) => eval_expr(eps_expr, None),
        None => INTEGRAL_EPS,
    };

    let tokens_it = lexer::Lexer::new(&expr).map(|res|
        res.map_err(print_err).unwrap()
    );
    let mut parser = parser::Parser::new(&expr);
    let postfix_tokens = parser.parse(tokens_it).map_err(print_err).unwrap();
    let f = |x| parser.eval(&postfix_tokens, Some(x)).map_err(print_err).unwrap();

    println!("{}", integral(f, x1, x2, eps, max_iterations));
}

fn print_err<T: std::fmt::Display>(error: T) {
    eprintln!("{}: {}", bold_red("Error"), error);
    exit(1);
}

