#[derive(Clone, Debug, PartialEq)]
pub enum TextToken<'a> {
    Space {
        pos: usize,
    },
    Plus {
        pos: usize,
    },
    Minus {
        pos: usize,
    },
    Star {
        pos: usize,
    },
    Slash {
        pos: usize,
    },
    Caret {
        pos: usize,
    },
    LParen {
        pos: usize,
    },
    RParen {
        pos: usize,
    },
    Comma {
        pos: usize,
    },
    Number {
        text: &'a str,
        value: f64,
        pos: usize,
    },
    Word {
        text: &'a str,
        pos: usize,
    },
}
