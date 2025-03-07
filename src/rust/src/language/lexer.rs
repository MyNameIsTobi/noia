use std::iter::Peekable;
use std::str::Chars;
use super::error::LexerError;
use super::types::{Token, Position, TokenWithPosition};

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    position: Position,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().peekable(),
            position: Position { line: 1, column: 0 },
            current_char: None,
        };
        lexer.advance();
        lexer
    }

    fn advance(&mut self) {
        self.current_char = self.input.next();
        if let Some('\n') = self.current_char {
            self.position.line += 1;
            self.position.column = 0;
        } else {
            self.position.column += 1;
        }
    }

    #[allow(dead_code)]
    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if !c.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn read_identifier(&mut self) -> Result<TokenWithPosition, LexerError> {
        let current_pos = self.position;
        let mut identifier = String::new();
        
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.advance();
            } else {
                break;
            }
        }
        
        let token = match identifier.as_str() {
            "var" => Token::Var,
            "fn" => Token::Fn,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "switch" => Token::Switch,
            "case" => Token::Case,
            "default" => Token::Default,
            "out" => Token::Out,
            "return" => Token::Return,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Identifier(identifier),
        };
        
        Ok(TokenWithPosition {
            token,
            position: current_pos,
        })
    }

    fn read_number(&mut self) -> Result<TokenWithPosition, LexerError> {
        let start_pos = self.position;
        let mut number = String::new();
        let mut has_decimal = false;

        while let Some(c) = self.current_char {
            if c.is_digit(10) {
                number.push(c);
                self.advance();
            } else if c == '.' {
                if has_decimal {
                    return Err(LexerError::InvalidNumber(number, start_pos.line, start_pos.column));
                }
                has_decimal = true;
                number.push(c);
                self.advance();
            } else {
                break;
            }
        }

        match number.parse::<f64>() {
            Ok(n) => Ok(TokenWithPosition {
                token: Token::Number(n),
                position: start_pos,
            }),
            Err(_) => Err(LexerError::InvalidNumber(number, start_pos.line, start_pos.column)),
        }
    }

    fn read_string(&mut self) -> Result<TokenWithPosition, LexerError> {
        let start_pos = self.position;
        self.advance(); // Skip opening quote
        let mut string = String::new();

        while let Some(c) = self.current_char {
            match c {
                '"' => {
                    self.advance(); // Skip closing quote
                    return Ok(TokenWithPosition {
                        token: Token::String(string),
                        position: start_pos,
                    });
                }
                '\\' => {
                    self.advance();
                    if let Some(next_char) = self.current_char {
                        match next_char {
                            'n' => string.push('\n'),
                            't' => string.push('\t'),
                            'r' => string.push('\r'),
                            '\\' => string.push('\\'),
                            '"' => string.push('"'),
                            _ => string.push(next_char),
                        }
                        self.advance();
                    }
                }
                _ => {
                    string.push(c);
                    self.advance();
                }
            }
        }

        Err(LexerError::UnterminatedString(start_pos.line, start_pos.column))
    }

    pub fn next_token(&mut self) -> Result<TokenWithPosition, LexerError> {
        self.skip_whitespace();
        
        let current_pos = self.position;
        
        if let Some(c) = self.current_char {
            match c {
                'a'..='z' | 'A'..='Z' | '_' => {
                    self.read_identifier()
                },
                '0'..='9' => self.read_number(),
                '"' => self.read_string(),
                '+' => {
                    self.advance();
                    Ok(TokenWithPosition {
                        token: Token::Plus,
                        position: current_pos,
                    })
                }
                '-' => {
                    self.advance();
                    Ok(TokenWithPosition {
                        token: Token::Minus,
                        position: current_pos,
                    })
                }
                '*' => {
                    self.advance();
                    Ok(TokenWithPosition {
                        token: Token::Multiply,
                        position: current_pos,
                    })
                }
                '/' => {
                    self.advance();
                    Ok(TokenWithPosition {
                        token: Token::Divide,
                        position: current_pos,
                    })
                }
                '=' => {
                    self.advance();
                    if let Some('=') = self.current_char {
                        self.advance();
                        Ok(TokenWithPosition {
                            token: Token::Equals,
                            position: current_pos,
                        })
                    } else {
                        Ok(TokenWithPosition {
                            token: Token::Assign,
                            position: current_pos,
                        })
                    }
                }
                '!' => {
                    self.advance();
                    if let Some('=') = self.current_char {
                        self.advance();
                        Ok(TokenWithPosition {
                            token: Token::NotEquals,
                            position: current_pos,
                        })
                    } else {
                        Ok(TokenWithPosition {
                            token: Token::Not,
                            position: current_pos,
                        })
                    }
                }
                '>' => {
                    self.advance();
                    if let Some('=') = self.current_char {
                        self.advance();
                        Ok(TokenWithPosition {
                            token: Token::GreaterEquals,
                            position: current_pos,
                        })
                    } else {
                        Ok(TokenWithPosition {
                            token: Token::Greater,
                            position: current_pos,
                        })
                    }
                }
                '<' => {
                    self.advance();
                    if let Some('=') = self.current_char {
                        self.advance();
                        Ok(TokenWithPosition {
                            token: Token::LessEquals,
                            position: current_pos,
                        })
                    } else {
                        Ok(TokenWithPosition {
                            token: Token::Less,
                            position: current_pos,
                        })
                    }
                }
                '&' => {
                    self.advance();
                    if let Some('&') = self.current_char {
                        self.advance();
                        Ok(TokenWithPosition {
                            token: Token::And,
                            position: current_pos,
                        })
                    } else {
                        Err(LexerError::InvalidCharacter('&', current_pos.line, current_pos.column))
                    }
                }
                '|' => {
                    self.advance();
                    if let Some('|') = self.current_char {
                        self.advance();
                        Ok(TokenWithPosition {
                            token: Token::Or,
                            position: current_pos,
                        })
                    } else {
                        Err(LexerError::InvalidCharacter('|', current_pos.line, current_pos.column))
                    }
                }
                '(' => {
                    self.advance();
                    Ok(TokenWithPosition {
                        token: Token::LeftParen,
                        position: current_pos,
                    })
                }
                ')' => {
                    self.advance();
                    Ok(TokenWithPosition {
                        token: Token::RightParen,
                        position: current_pos,
                    })
                }
                '{' => {
                    self.advance();
                    Ok(TokenWithPosition {
                        token: Token::LeftBrace,
                        position: current_pos,
                    })
                }
                '}' => {
                    self.advance();
                    Ok(TokenWithPosition {
                        token: Token::RightBrace,
                        position: current_pos,
                    })
                }
                ',' => {
                    self.advance();
                    Ok(TokenWithPosition {
                        token: Token::Comma,
                        position: current_pos,
                    })
                }
                ':' => {
                    self.advance();
                    Ok(TokenWithPosition {
                        token: Token::Colon,
                        position: current_pos,
                    })
                }
                ';' => {
                    self.advance();
                    Ok(TokenWithPosition {
                        token: Token::Semicolon,
                        position: current_pos,
                    })
                }
                _ => Err(LexerError::InvalidCharacter(
                    c,
                    current_pos.line,
                    current_pos.column,
                )),
            }
        } else {
            Ok(TokenWithPosition {
                token: Token::EOF,
                position: current_pos,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keywords() {
        let input = "var fn if else while for switch case default out return";
        let mut lexer = Lexer::new(input);

        let expected = vec![
            Token::Var,
            Token::Fn,
            Token::If,
            Token::Else,
            Token::While,
            Token::For,
            Token::Switch,
            Token::Case,
            Token::Default,
            Token::Out,
            Token::Return,
            Token::EOF,
        ];

        for expected_token in expected {
            let result = lexer.next_token().unwrap();
            assert_eq!(result.token, expected_token);
        }
    }

    #[test]
    fn test_identifiers() {
        let input = "abc x123 _test test_var";
        let mut lexer = Lexer::new(input);

        let expected = vec![
            Token::Identifier("abc".to_string()),
            Token::Identifier("x123".to_string()),
            Token::Identifier("_test".to_string()),
            Token::Identifier("test_var".to_string()),
            Token::EOF,
        ];

        for expected_token in expected {
            let result = lexer.next_token().unwrap();
            assert_eq!(result.token, expected_token);
        }
    }

    #[test]
    fn test_numbers() {
        let input = "42 3.14 0.123 42.0";
        let mut lexer = Lexer::new(input);

        let expected = vec![
            Token::Number(42.0),
            Token::Number(3.14),
            Token::Number(0.123),
            Token::Number(42.0),
            Token::EOF,
        ];

        for expected_token in expected {
            let result = lexer.next_token().unwrap();
            assert_eq!(result.token, expected_token);
        }
    }

    #[test]
    fn test_strings() {
        let input = r#""Hello" "World\n" "Test \"quoted\"" "\\backslash""#;
        let mut lexer = Lexer::new(input);

        let expected = vec![
            Token::String("Hello".to_string()),
            Token::String("World\n".to_string()),
            Token::String("Test \"quoted\"".to_string()),
            Token::String("\\backslash".to_string()),
            Token::EOF,
        ];

        for expected_token in expected {
            let result = lexer.next_token().unwrap();
            assert_eq!(result.token, expected_token);
        }
    }

    #[test]
    fn test_operators() {
        let input = "+ - * / = == != > < >= <= && || !";
        let mut lexer = Lexer::new(input);

        let expected = vec![
            Token::Plus,
            Token::Minus,
            Token::Multiply,
            Token::Divide,
            Token::Assign,
            Token::Equals,
            Token::NotEquals,
            Token::Greater,
            Token::Less,
            Token::GreaterEquals,
            Token::LessEquals,
            Token::And,
            Token::Or,
            Token::Not,
            Token::EOF,
        ];

        for expected_token in expected {
            let result = lexer.next_token().unwrap();
            assert_eq!(result.token, expected_token);
        }
    }

    #[test]
    fn test_delimiters() {
        let input = "( ) { } , ;";
        let mut lexer = Lexer::new(input);

        let expected = vec![
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::RightBrace,
            Token::Comma,
            Token::Semicolon,
            Token::EOF,
        ];

        for expected_token in expected {
            let result = lexer.next_token().unwrap();
            assert_eq!(result.token, expected_token);
        }
    }

    #[test]
    fn test_complex_expression() {
        let input = "if (x > 5) { print(\"Hello\"); }";
        let mut lexer = Lexer::new(input);

        let expected = vec![
            Token::If,
            Token::LeftParen,
            Token::Identifier("x".to_string()),
            Token::Greater,
            Token::Number(5.0),
            Token::RightParen,
            Token::LeftBrace,
            Token::Identifier("print".to_string()),
            Token::LeftParen,
            Token::String("Hello".to_string()),
            Token::RightParen,
            Token::Semicolon,
            Token::RightBrace,
            Token::EOF,
        ];

        for expected_token in expected {
            let result = lexer.next_token().unwrap();
            assert_eq!(result.token, expected_token);
        }
    }

    #[test]
    fn test_error_invalid_character() {
        let input = "@";
        let mut lexer = Lexer::new(input);
        let result = lexer.next_token();
        assert!(result.is_err());
    }

    #[test]
    fn test_error_unterminated_string() {
        let input = "\"unterminated";
        let mut lexer = Lexer::new(input);
        let result = lexer.next_token();
        assert!(result.is_err());
    }

    #[test]
    fn test_error_invalid_number() {
        let input = "42.42.42";
        let mut lexer = Lexer::new(input);
        let result = lexer.next_token();
        assert!(result.is_err());
    }
} 