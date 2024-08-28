#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy)]
pub enum Token<'src> {
    Add { pos: usize },
    Sub { pos: usize },
    Mul { pos: usize },
    UM { pos: usize },
    Div { pos: usize },
    Pow { pos: usize },
    LParen { pos: usize },
    RParen { pos: usize },
    Comma { pos: usize },

    Var { pos: usize },
    Number { text: &'src str, value: f64, pos: usize },

    Abs { pos: usize },
    Acos { pos: usize },
    Acosh { pos: usize },
    Asin { pos: usize },
    Asinh { pos: usize },
    Atan { pos: usize },
    Atanh { pos: usize },
    Cbrt { pos: usize },
    Ceil { pos: usize },
    Cos { pos: usize },
    Cosh { pos: usize },
    Exp { pos: usize },
    Exp2 { pos: usize },
    Floor { pos: usize },
    Fract { pos: usize },
    Ln { pos: usize },
    Log2 { pos: usize },
    Log10 { pos: usize },
    Round { pos: usize },
    Sign { pos: usize },
    Sin { pos: usize },
    Sinh { pos: usize },
    Sqrt { pos: usize },
    Tan { pos: usize },
    Tanh { pos: usize },
    ToDeg { pos: usize },
    ToRad { pos: usize },
    Trunc { pos: usize },

    E { pos: usize },
    Pi { pos: usize },
    Eps { pos: usize },
}

impl<'src> Token<'src> {
    pub fn from_text(text: &'src str, pos: usize) -> Option<Self> {
        Some(match text {
            "x" => Self::Var { pos },

            "abs" => Self::Abs { pos },
            "acos" => Self::Acos { pos },
            "acosh" => Self::Acosh { pos },
            "asin" => Self::Asin { pos },
            "asinh" => Self::Asinh { pos },
            "atan" => Self::Atan { pos },
            "atanh" => Self::Atanh { pos },
            "cbrt" => Self::Cbrt { pos },
            "ceil" => Self::Ceil { pos },
            "cos" => Self::Cos { pos },
            "cosh" => Self::Cosh { pos },
            "exp" => Self::Exp { pos },
            "exp2" => Self::Exp2 { pos },
            "floor" => Self::Floor { pos },
            "fract" => Self::Fract { pos },
            "ln" => Self::Ln { pos },
            "log2" => Self::Log2 { pos },
            "log10" => Self::Log10 { pos },
            "round" => Self::Round { pos },
            "sign" => Self::Sign { pos },
            "sin" => Self::Sin { pos },
            "sinh" => Self::Sinh { pos },
            "sqrt" => Self::Sqrt { pos },
            "tan" => Self::Tan { pos },
            "tanh" => Self::Tanh { pos },
            "toDeg" => Self::ToDeg { pos },
            "toRad" => Self::ToRad { pos },
            "trunc" => Self::Trunc { pos },

            "e" => Self::E { pos },
            "pi" => Self::Pi { pos },
            "eps" => Self::Eps { pos },
            _ => return None,
        })
    }

    pub fn get_text(&self) -> &'src str {
        match self {
            Self::Add { .. } => "+",
            Self::Sub { .. } => "-",
            Self::Mul { .. } => "*",
            Self::UM { .. } => "-",
            Self::Div { .. } => "/",
            Self::Pow { .. } => "^",
            Self::LParen { .. } => "(",
            Self::RParen { .. } => ")",
            Self::Comma { .. } => ",",

            Self::Var { .. } => "x",
            Self::Number { text, .. } => text,

            Self::Abs { .. } => "abs",
            Self::Acos { .. } => "acos",
            Self::Acosh { .. } => "acosh",
            Self::Asin { .. } => "asin",
            Self::Asinh { .. } => "asinh",
            Self::Atan { .. } => "atan",
            Self::Atanh { .. } => "atanh",
            Self::Cbrt { .. } => "cbrt",
            Self::Ceil { .. } => "ceil",
            Self::Cos { .. } => "cos",
            Self::Cosh { .. } => "cosh",
            Self::Exp { .. } => "exp",
            Self::Exp2 { .. } => "exp2",
            Self::Floor { .. } => "floor",
            Self::Fract { .. } => "fract",
            Self::Ln { .. } => "ln",
            Self::Log2 { .. } => "log2",
            Self::Log10 { .. } => "log10",
            Self::Round { .. } => "round",
            Self::Sign { .. } => "sign",
            Self::Sin { .. } => "sin",
            Self::Sinh { .. } => "sinh",
            Self::Sqrt { .. } => "sqrt",
            Self::Tan { .. } => "tan",
            Self::Tanh { .. } => "tanh",
            Self::ToDeg { .. } => "toDeg",
            Self::ToRad { .. } => "toRad",
            Self::Trunc { .. } => "trunc",

            Self::E { .. } => "e",
            Self::Pi { .. } => "pi",
            Self::Eps { .. } => "eps",
        }
    }

    pub fn is_operand(&self) -> bool {
        match self {
            Self::Var { .. }
            | Self::Number { .. }
            | Self::E { .. }
            | Self::Pi { .. }
            | Self::Eps { .. } => true,
            _ => false,
        }
    }

    pub fn is_function(&self) -> bool {
        match self {
            Self::Abs { .. }
            | Self::Acos { .. }
            | Self::Acosh { .. }
            | Self::Asin { .. }
            | Self::Asinh { .. }
            | Self::Atan { .. }
            | Self::Atanh { .. }
            | Self::Cbrt { .. }
            | Self::Ceil { .. }
            | Self::Cos { .. }
            | Self::Cosh { .. }
            | Self::Exp { .. }
            | Self::Exp2 { .. }
            | Self::Floor { .. }
            | Self::Fract { .. }
            | Self::Ln { .. }
            | Self::Log2 { .. }
            | Self::Log10 { .. }
            | Self::Round { .. }
            | Self::Sign { .. }
            | Self::Sin { .. }
            | Self::Sinh { .. }
            | Self::Sqrt { .. }
            | Self::Tan { .. }
            | Self::Tanh { .. }
            | Self::ToDeg { .. }
            | Self::ToRad { .. }
            | Self::Trunc { .. } => true,
            _ => false,
        }
    }
}
