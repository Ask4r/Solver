# Solver: Your Mathematical Expression Companion

Solver is a command-line tool designed to help you solve various mathematical expressions with ease. It provides a simple and intuitive interface for:

- Evaluating expressions: Calculate the value of complex expressions, including constants like pi and e, trigonometric functions, and more.
- Finding roots of equations: Determine the points where a given function intersects the x-axis within a specified interval.
- Calculating definite integrals:  Compute the area under a function's curve between two given limits of integration.

## Features

- Easy-to-use command-line interface: Enter your expressions and commands directly in the terminal.
- Supports a wide range of mathematical functions:  Trigonometric functions, logarithms, exponentials, and more.
- Flexible input: Define the range for root finding and integration limits with ease.
- Clear output:  Get accurate results displayed in a user-friendly format.
- Useful error messages: Know how to fix errors when something goes wrong.

## Getting Started

### Installation
   - Prerequisites: Ensure you have Cargo installed on your system.
   - Download the code:  Clone this repository or download the source code as a zip file.
   - Run: Open your terminal and navigate to the project directory. Execute the following command: `cargo run -- --help`

### Usage
Help:
```
$ solver --help

Solver for mathematical expressions

Usage: solver <COMMAND>

Commands:
  eval      Evaluate expression
  root      Find root of the <EXPR> with variable `x` with `false position` root-finding algorithm on the interval between <X1> and <X2>
  integral  Find definite integral of the <EXPR> with variable `x` with iterative `rectangular` method on the interval between <X1> and <X2>
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
Evaluating expressions:
```
solver eval "pi + sin(-7/2*pi) + e ^ (-1)" 
```
Finding roots:
```
solver root "sqrt(x - 1) - e * ln(x)" "pi/3" "pi/2"
```
Calculating definite integrals:
```
solver integral "e ^ (-(x^2))" "3" "10^4"
```

## Examples

```
$ solver eval "pi + sin(-7/2*pi) + e ^ (-1)"
4.509472094761236

$ solver root "sqrt(x - 1) - e * ln(x)" "pi/3" "pi/2"
1.156838140254635

$ solver integral "e ^ (-(x^2))" "3" "10^4"
0.000019935106561998955
```

Enjoy solving mathematical expressions with ease!
