use core::f64;

use crate::tokens::TokenType;

pub fn parse_number(text: &str) -> Option<TokenType> {
    match text.parse::<f64>() {
        Ok(n) => Some(TokenType::Number(n)),
        Err(_) => None,
    }
}

pub fn parse_ident(text: &str) -> Option<TokenType> {
    if let Some(token) = parse_var(text) {
        return Some(token);
    }
    if let Some(token) = parse_const(text) {
        return Some(token);
    }
    if let Some(token) = parse_func(text) {
        return Some(token);
    }
    return None;
}

fn parse_var(text: &str) -> Option<TokenType> {
    if text == "x" {
        return Some(TokenType::Var);
    }
    return None;
}

fn parse_const(text: &str) -> Option<TokenType> {
    use TokenType::*;
    return Some(match text {
        "e" => Const(f64::consts::E),
        "pi" => Const(f64::consts::PI),
        "eps" => Const(0.000_001),
        _ => return None,
    });
}

fn parse_func(text: &str) -> Option<TokenType> {
    use TokenType::*;
    return Some(match text {
        "abs" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::abs(args[0]),
        },
        "acos" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::acos(args[0]),
        },
        "acosh" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::acosh(args[0]),
        },
        "asin" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::asin(args[0]),
        },
        "asinh" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::asinh(args[0]),
        },
        "atan" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::atan(args[0]),
        },
        "atan2" => Func {
            args: 2,
            func: |args: Vec<f64>| f64::atan2(args[0], args[1]),
        },
        "atanh" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::atanh(args[0]),
        },
        "cbrt" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::cbrt(args[0]),
        },
        "ceil" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::ceil(args[0]),
        },
        "clamp" => Func {
            args: 3,
            func: |args: Vec<f64>| f64::clamp(args[0], args[1], args[2]),
        },
        "cos" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::cos(args[0]),
        },
        "cosh" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::cosh(args[0]),
        },
        "exp" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::exp(args[0]),
        },
        "exp2" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::exp2(args[0]),
        },
        "floor" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::floor(args[0]),
        },
        "fract" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::fract(args[0]),
        },
        "hypot" => Func {
            args: 2,
            func: |args: Vec<f64>| f64::hypot(args[0], args[1]),
        },
        "ln" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::ln(args[0]),
        },
        "log2" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::log2(args[0]),
        },
        "log10" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::log10(args[0]),
        },
        "max" => Func {
            args: 2,
            func: |args: Vec<f64>| f64::max(args[0], args[1]),
        },
        "min" => Func {
            args: 2,
            func: |args: Vec<f64>| f64::min(args[0], args[1]),
        },
        "mul_add" => Func {
            args: 3,
            func: |args: Vec<f64>| f64::mul_add(args[0], args[1], args[2]),
        },
        "pow" => Func {
            args: 2,
            func: |args: Vec<f64>| f64::powf(args[0], args[1]),
        },
        "recip" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::recip(args[0]),
        },
        "round" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::round(args[0]),
        },
        "sign" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::signum(args[0]),
        },
        "sin" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::sin(args[0]),
        },
        "sinh" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::sinh(args[0]),
        },
        "sqrt" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::sqrt(args[0]),
        },
        "tan" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::tan(args[0]),
        },
        "tanh" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::tanh(args[0]),
        },
        "toDeg" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::to_degrees(args[0]),
        },
        "toRad" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::to_radians(args[0]),
        },
        "trunc" => Func {
            args: 1,
            func: |args: Vec<f64>| f64::trunc(args[0]),
        },
        _ => return None,
    });
}
