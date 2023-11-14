#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub(crate) enum TokenType {
    INT,
    FLOAT,
    ADD,
    SUB,
    MUL,
    DIV,
    EXP,
    LN,
    LOG,
    SIN,
    COS,
    OpenParen,
    CloseParen
}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) value: Option<String>,
}


pub(crate) struct Tokenizer {
    input: Vec<char>,
    current_index: usize,
    tokens: Vec<Token>,
}


impl Tokenizer {
    const BASE: u32 = 10;
    pub(crate) fn new(input: &str) -> Tokenizer{
        Tokenizer{input:input.chars().collect(), current_index:0,tokens: vec![]}
    }
    fn tokenize_function(&mut self) {
        let mut chars: Vec<char> = vec![];
        while self.peek().is_some() && self.peek().unwrap().is_alphabetic() {
            chars.push(*self.consume().unwrap());
        }
        let token_type_string: String = chars.iter().collect::<String>();
        match token_type_string.as_str() {
            "log" => self.tokens.push(Token { token_type: TokenType::LOG, value: None }),
            "ln" => self.tokens.push(Token { token_type: TokenType::LN, value: None }),
            "sin" => self.tokens.push(Token { token_type: TokenType::SIN, value: None }),
            "cos" => self.tokens.push(Token { token_type: TokenType::COS, value: None }),
            _ => panic!("Incorrect token: {}", token_type_string),
        }
    }
    fn tokenize_number(&mut self) {
        let mut num = vec![];
        let mut is_float = false;
        while self.peek().is_some() {
            let current_char = *self.peek().unwrap();
            if current_char.is_digit(Tokenizer::BASE) {
                num.push(*self.consume().unwrap());
            }
            else if current_char == '.' {
                if is_float {
                    panic!("Cannot include more than one `.` in expression");
                }
                is_float = true;
                num.push(*self.consume().unwrap());
            } else if current_char == '_' {
                _ = self.consume();
            }
            else {
                break;
            }
        }
        let resulting_string: String = num.iter().collect();
        if is_float {
            self.tokens.push(Token{token_type: TokenType::FLOAT, value: Some(resulting_string) });
        } else {
            self.tokens.push(Token{token_type: TokenType::INT, value: Some(resulting_string) });
        }
    }
    pub(crate) fn tokenize(&mut self) -> Vec<Token> {
        while self.peek().is_some() {
            match self.peek().unwrap() {
                c if c.is_alphabetic() => self.tokenize_function(),
                c if c.is_digit(Tokenizer::BASE) => self.tokenize_number(),
                c if c.is_whitespace() => _ = self.consume(),
                '+' => {
                    self.consume();
                    self.tokens.push(Token{ token_type: TokenType::ADD, value: None })
                },
                '-' => {
                    self.consume();
                    self.tokens.push(Token{ token_type: TokenType::SUB, value: None })
                },
                '*' => {
                    self.consume();
                    self.tokens.push(Token{ token_type: TokenType::MUL, value: None })
                },
                '/' => {
                    self.consume();
                    self.tokens.push(Token{ token_type: TokenType::DIV, value: None })
                },
                '^' => {
                    self.consume();
                    self.tokens.push(Token{ token_type: TokenType::EXP, value: None })
                }, '(' => {
                    self.consume();
                    self.tokens.push(Token{ token_type: TokenType::OpenParen, value: None })
                }, ')' => {
                    self.consume();
                    self.tokens.push(Token{ token_type: TokenType::CloseParen, value: None })
                },
                _ => panic!("Unknown character while parsing: `{}`", self.peek().unwrap()),
            }
        }
        self.tokens.iter().cloned().collect()
    }

    fn peek(&self) -> Option<&char> {
        self.input.get(self.current_index)
    }
    fn consume(&mut self) -> Option<&char> {
        let res = self.input.get(self.current_index);
        self.current_index += 1;
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_function_valid() {
        let mut tokenizer = Tokenizer { input: "log".chars().collect(), current_index: 0, tokens: Vec::new() };
        tokenizer.tokenize_function();
        assert_eq!(tokenizer.tokens.len(), 1);
        assert_eq!(tokenizer.tokens[0].token_type,  TokenType::LOG);
        assert_eq!(tokenizer.tokens[0].value,  None);

        tokenizer = Tokenizer { input: "ln".chars().collect(), current_index: 0, tokens: Vec::new() };
        tokenizer.tokenize_function();
        assert_eq!(tokenizer.tokens.len(), 1);
        assert_eq!(tokenizer.tokens[0].token_type,  TokenType::LN);
        assert_eq!(tokenizer.tokens[0].value,  None);

        tokenizer = Tokenizer { input: "sin".chars().collect(), current_index: 0, tokens: Vec::new() };
        tokenizer.tokenize_function();
        assert_eq!(tokenizer.tokens.len(), 1);
        assert_eq!(tokenizer.tokens[0].token_type,  TokenType::SIN);
        assert_eq!(tokenizer.tokens[0].value,  None);

        tokenizer = Tokenizer { input: "cos".chars().collect(), current_index: 0, tokens: Vec::new() };
        tokenizer.tokenize_function();
        assert_eq!(tokenizer.tokens.len(), 1);
        assert_eq!(tokenizer.tokens[0].token_type,  TokenType::COS);
        assert_eq!(tokenizer.tokens[0].value,  None);
    }

    #[test]
    #[should_panic(expected = "Incorrect token")]
    fn test_tokenize_function_invalid() {
        let mut tokenizer = Tokenizer { input: "abc".chars().collect(), current_index: 0, tokens: Vec::new() };
        tokenizer.tokenize_function();
    }

    #[test]
    fn test_tokenize_number_valid() {
        let mut tokenizer = Tokenizer { input: "123".chars().collect(), current_index: 0, tokens: Vec::new() };
        tokenizer.tokenize_number();
        assert_eq!(tokenizer.tokens, vec![Token { token_type: TokenType::INT, value: Some("123".to_string()) }]);

        tokenizer = Tokenizer { input: "123.45".chars().collect(), current_index: 0, tokens: Vec::new() };
        tokenizer.tokenize_number();
        assert_eq!(tokenizer.tokens, vec![Token { token_type: TokenType::FLOAT, value: Some("123.45".to_string()) }]);
    }

    #[test]
    #[should_panic(expected = "Cannot include more than one `.` in expression")]
    fn test_tokenize_number_invalid() {
        let mut tokenizer = Tokenizer { input: "123.45.67".chars().collect(), current_index: 0, tokens: Vec::new() };
        tokenizer.tokenize_number();
    }

    #[test]
    fn test_tokenize_mixed_input() {
        let mut tokenizer = Tokenizer { input: "sin45+log".chars().collect(), current_index: 0, tokens: Vec::new() };
        tokenizer.tokenize();
        assert_eq!(tokenizer.tokens, vec![
            Token { token_type: TokenType::SIN, value: None },
            Token { token_type: TokenType::INT, value: Some("45".to_string()) },
            Token { token_type: TokenType::ADD, value: None },
            Token { token_type: TokenType::LOG, value: None }
        ]);
    }

    #[test]
    fn test_peek_and_consume() {
        let mut tokenizer = Tokenizer { input: "test".chars().collect(), current_index: 0, tokens: Vec::new() };
        assert_eq!(tokenizer.peek(), Some(&'t'));
        assert_eq!(tokenizer.consume(), Some(&'t'));
        assert_eq!(tokenizer.peek(), Some(&'e'));
    }

}
