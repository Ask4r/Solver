use crate::lexer::Token;
use errors::ParsingError;

mod errors;

pub struct Parser {
    source: String,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(source: String, tokens: Vec<Token>) -> Self {
        Self { source, tokens }
    }

    pub fn build(&mut self) -> Result<Vec<Token>, ParsingError> {
        use Token::*;
        let mut op_stack: Vec<Token> = Vec::new();
        let mut postfix_list: Vec<Token> = Vec::new();

        for token in self.tokens.iter() {
            match token {
                Number { .. } | Const { .. } | Var { .. } => postfix_list.push(token.clone()),
                LParen { .. } | Func { .. } => op_stack.push(token.clone()),
                RParen { pos: op_pos } => loop {
                    match op_stack.pop() {
                        Some(LParen { .. }) => break,
                        Some(item) => postfix_list.push(item),
                        None => return Err(self.error_parsing(")".into(), *op_pos)),
                    }
                },
                EOF => (),
                _ => loop {
                    match op_stack.last() {
                        Some(oper) if Self::get_prec(oper) >= Self::get_prec(&token) => {
                            postfix_list.push(oper.clone());
                            op_stack.pop();
                        }
                        _ => break op_stack.push(token.clone()),
                    }
                },
            }
        }

        op_stack.reverse();
        postfix_list.extend(op_stack);
        Ok(postfix_list)
    }

    fn get_prec(token: &Token) -> u64 {
        match token {
            Token::UM { .. } => 5,
            Token::Caret { .. } => 4,
            Token::Star { .. } | Token::Slash { .. } => 3,
            Token::Plus { .. } | Token::Minus { .. } => 2,
            Token::RParen { .. } | Token::Comma { .. } => 1,
            Token::LParen { .. } => 0,
            _ => 10,
        }
    }

    fn error_parsing(&self, text: &str, pos: usize) -> ParsingError {
        ParsingError::new(self.source.clone(), text.into(), pos)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexer::Lexer;
    use Token::*;

    #[test]
    fn it_works() {
        let text = "2 + 2 * sin(3 ^ -3)";
        let mut lexer = Lexer::new(text.into());
        let tokens = lexer.parse().unwrap();
        let mut parser = Parser::new(text.into(), tokens);
        let parsed_toks = parser.build().unwrap();

        assert_eq!(parsed_toks[0], Number { text: "2".into(), value: 2.0, pos: 0 });
        assert_eq!(parsed_toks[1], Number { text: "2".into(), value: 2.0, pos: 4 });
        assert_eq!(parsed_toks[2], Number { text: "3".into(), value: 3.0, pos: 12 });
        assert_eq!(parsed_toks[3], Number { text: "3".into(), value: 3.0, pos: 17 });
        assert_eq!(parsed_toks[4], UM { pos: 16 });
        assert_eq!(parsed_toks[5], Caret { pos: 14 });
        assert_eq!(parsed_toks[6], Func { text: "sin".into(), func: f64::sin, pos: 8 });
        assert_eq!(parsed_toks[7], Star { pos: 6 });
        assert_eq!(parsed_toks[8], Plus { pos: 2 });
    }
}
