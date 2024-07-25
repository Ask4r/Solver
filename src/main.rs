use colors::{bold_cyan, bold_green, bold_red, bold_white, cyan};
use getopts::Options;
use lexer::Lexer;
use parser::Parser;
use std::fmt;

pub mod colors;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Show this information");
    opts.optopt("e", "eval", "Evaluate the expression", "EXPR");
    opts.optopt("r", "root", "Find root of the expression between x1 and x2.", "EXPR");
    opts.optopt("i", "integral", "Find integral of the expression bounded by x1 and x2.", "EXPR");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!("{}", e.to_string()),
    };

    match matches {
        m if m.opt_present("h") => print_help(&program),
        m if m.opt_present("e") => match m.opt_str("e") {
            None => print_error("Flag -e requires an argument <EXPR>"),
            Some(expr) => eval(expr),
        },
        m if m.opt_present("r") => unimplemented!("Roots are unimplemented"),
        m if m.opt_present("i") => unimplemented!("Integrals are unimplemented"),
        _ => eprintln!("Try `{program} -h`"),
    }
}

fn print_error<T: fmt::Display>(error: T) {
    eprintln!("{}{}{}", bold_red("Error"), bold_white(": "), error)
}

fn print_help(program: &str) {
    println!("Solver for mathematical expressions\n");
    println!("{} {} {}\n", bold_green("Usage:"), bold_cyan(program), cyan("[OPTIONS] [ARGS]"));
    println!("{}", bold_green("Arguments:"));
    println!( "  {}  {}\n", cyan("[ARGS]"), "Arguments for a solver to run");
    println!("{}", bold_green("Options:"));
    print!("  {}, {}", bold_cyan("-h"), bold_cyan("--help"));
    println!("                    Show this information");
    print!("  {}, {} {}", bold_cyan("-e"), bold_cyan("--eval"), cyan("<EXPR>"));
    println!("             Evaluate the expression");
    print!("  {}, {} {}", bold_cyan("-r"), bold_cyan("--root"), cyan("<EXPR[x1;x2]>"));
    println!("      Find root of the expression between x1 and x2");
    print!("  {}, {} {}", bold_cyan("-i"), bold_cyan("--integral"), cyan("<EXPR[x1;x2]>"));
    println!("  Find integral of the expression bounded by x1 and x2");
}

fn eval(expr: String) {
    println!("Text: `{}`", expr);

    let mut lexer = Lexer::new(expr.clone());
    let tokens = match lexer.parse() {
        Ok(tokens) => tokens,
        Err(e) => return print_error(e),
    };

    let mut parser = Parser::new(expr.clone(), tokens);
    let parsed_tokens = match parser.build() {
        Ok(tokens) => tokens,
        Err(e) => return print_error(e),
    };

    println!();
    for tok in parsed_tokens {
        println!("{:?}", tok)
    }
}
