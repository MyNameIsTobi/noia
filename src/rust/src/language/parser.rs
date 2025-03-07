use super::ast::{Expression, Statement, BinaryOperator};
use super::error::ParserError;
use super::types::{Token, TokenWithPosition};
use super::lexer::Lexer;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Option<TokenWithPosition>,
    peek_token: Option<TokenWithPosition>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token().ok();
        let peek_token = lexer.next_token().ok();
        
        Parser {
            lexer,
            current_token,
            peek_token,
        }
    }

    fn advance_token(&mut self) {
        self.current_token = self.peek_token.take();
        self.peek_token = self.lexer.next_token().ok();
    }

    fn expect_token(&mut self, expected: Token) -> Result<(), ParserError> {
        if let Some(token) = &self.current_token {
            if token.token == expected {
                self.advance_token();
                return Ok(());
            }
        }
        Err(ParserError::UnexpectedToken)
    }

    // Hauptparsing-Methode
    pub fn parse_program(&mut self) -> Result<Vec<Statement>, ParserError> {
        let mut statements = Vec::new();
        
        while let Some(token) = &self.current_token {
            if matches!(token.token, Token::EOF) {
                break;
            }
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }
        
        Ok(statements)
    }

    // Statement-Parsing
    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match &self.current_token {
            Some(TokenWithPosition { token: Token::Var, .. }) => {
                self.parse_var_declaration()
            }
            Some(TokenWithPosition { token: Token::Fn, .. }) => {
                self.parse_function_declaration()
            }
            Some(TokenWithPosition { token: Token::If, .. }) => {
                self.parse_if_statement()
            }
            Some(TokenWithPosition { token: Token::While, .. }) => {
                self.parse_while_statement()
            }
            Some(TokenWithPosition { token: Token::For, .. }) => {
                self.parse_for_statement()
            }
            Some(TokenWithPosition { token: Token::Switch, .. }) => {
                self.parse_switch_statement()
            }
            Some(TokenWithPosition { token: Token::Return, .. }) => {
                self.parse_return_statement()
            }
            Some(TokenWithPosition { token: Token::LeftBrace, .. }) => {
                self.parse_block_statement()
            }
            Some(TokenWithPosition { token: Token::Identifier(name), .. }) => {
                let name = name.clone();
                self.advance_token();
                
                if let Some(TokenWithPosition { token: Token::Assign, .. }) = &self.current_token {
                    self.advance_token();
                    let value = self.parse_expression(0)?;
                    self.expect_token(Token::Semicolon)?;
                    Ok(Statement::Expression(Expression::Binary {
                        left: Box::new(Expression::Identifier(name)),
                        operator: BinaryOperator::Assign,
                        right: Box::new(value),
                    }))
                } else if let Some(TokenWithPosition { token: Token::LeftParen, .. }) = &self.current_token {
                    let expr = self.parse_function_call(Expression::Identifier(name))?;
                    self.expect_token(Token::Semicolon)?;
                    Ok(Statement::Expression(expr))
                } else {
                    let expr = Expression::Identifier(name);
                    self.expect_token(Token::Semicolon)?;
                    Ok(Statement::Expression(expr))
                }
            }
            Some(_) => {
                let expr = self.parse_expression(0)?;
                self.expect_token(Token::Semicolon)?;
                Ok(Statement::Expression(expr))
            }
            None => Err(ParserError::UnexpectedEOF),
        }
    }

    // Expression-Parsing mit Precedence Climbing
    fn parse_expression(&mut self, precedence: u8) -> Result<Expression, ParserError> {
        let mut left = self.parse_primary()?;

        while let Some(token) = &self.current_token {
            if let Some(op) = self.get_binary_operator(&token.token) {
                if op.precedence() <= precedence {
                    break;
                }
                
                self.advance_token();
                let right = self.parse_expression(op.precedence())?;
                left = Expression::Binary {
                    left: Box::new(left),
                    operator: op,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    // Primäre Ausdrücke
    fn parse_primary(&mut self) -> Result<Expression, ParserError> {
        let token = self.current_token.clone();
        match token {
            Some(token) => {
                match token.token {
                    Token::Number(n) => {
                        self.advance_token();
                        Ok(Expression::number(n))
                    },
                    Token::String(s) => {
                        self.advance_token();
                        Ok(Expression::string(s))
                    },
                    Token::Identifier(name) => {
                        self.advance_token();
                        if let Some(next) = &self.current_token {
                            if next.token == Token::LeftParen {
                                return self.parse_function_call(Expression::Identifier(name));
                            }
                        }
                        Ok(Expression::identifier(name))
                    },
                    Token::LeftParen => {
                        self.advance_token();
                        let expr = self.parse_expression(0)?;
                        self.expect_token(Token::RightParen)?;
                        Ok(expr)
                    },
                    Token::True => {
                        self.advance_token();
                        Ok(Expression::boolean(true))
                    },
                    Token::False => {
                        self.advance_token();
                        Ok(Expression::boolean(false))
                    },
                    _ => Err(ParserError::UnexpectedToken),
                }
            },
            None => Err(ParserError::UnexpectedEOF),
        }
    }

    // Hilfsmethoden
    fn get_binary_operator(&self, token: &Token) -> Option<BinaryOperator> {
        match token {
            Token::Plus => Some(BinaryOperator::Add),
            Token::Minus => Some(BinaryOperator::Subtract),
            Token::Multiply => Some(BinaryOperator::Multiply),
            Token::Divide => Some(BinaryOperator::Divide),
            Token::Equals => Some(BinaryOperator::Equal),
            Token::NotEquals => Some(BinaryOperator::NotEqual),
            Token::Greater => Some(BinaryOperator::Greater),
            Token::Less => Some(BinaryOperator::Less),
            Token::GreaterEquals => Some(BinaryOperator::GreaterEqual),
            Token::LessEquals => Some(BinaryOperator::LessEqual),
            Token::And => Some(BinaryOperator::And),
            Token::Or => Some(BinaryOperator::Or),
            _ => None,
        }
    }

    // Implementierung der Statement-Parser
    fn parse_var_declaration(&mut self) -> Result<Statement, ParserError> {
        self.advance_token(); // consume 'var'
        
        if let Some(TokenWithPosition { token: Token::Identifier(name), .. }) = &self.current_token {
            let name = name.clone();
            self.advance_token();
            
            let initializer = if let Some(token) = &self.current_token {
                if token.token == Token::Assign {
                    self.advance_token();
                    Some(self.parse_expression(0)?)
                } else {
                    None
                }
            } else {
                None
            };
            
            if let Some(token) = &self.current_token {
                if token.token == Token::Semicolon {
                    self.advance_token();
                    Ok(Statement::VarDeclaration { name, initializer })
                } else {
                    Err(ParserError::UnexpectedToken)
                }
            } else {
                Err(ParserError::UnexpectedEOF)
            }
        } else {
            Err(ParserError::ExpectedIdentifier)
        }
    }

    fn parse_function_declaration(&mut self) -> Result<Statement, ParserError> {
        self.advance_token(); // consume 'fn'
        
        let name = if let Some(TokenWithPosition { token: Token::Identifier(name), .. }) = &self.current_token {
            name.clone()
        } else {
            return Err(ParserError::ExpectedIdentifier);
        };
        self.advance_token();
        
        self.expect_token(Token::LeftParen)?;
        let mut params = Vec::new();
        
        if let Some(token) = &self.current_token {
            if token.token != Token::RightParen {
                loop {
                    if let Some(TokenWithPosition { token: Token::Identifier(param), .. }) = &self.current_token {
                        params.push(param.clone());
                        self.advance_token();
                        
                        if let Some(token) = &self.current_token {
                            match token.token {
                                Token::RightParen => break,
                                Token::Comma => {
                                    self.advance_token();
                                    continue;
                                }
                                _ => return Err(ParserError::UnexpectedToken),
                            }
                        } else {
                            return Err(ParserError::UnexpectedEOF);
                        }
                    } else {
                        return Err(ParserError::ExpectedIdentifier);
                    }
                }
            }
        }
        
        self.expect_token(Token::RightParen)?;
        let body = self.parse_block_statement()?;
        
        if let Statement::Block(statements) = body {
            Ok(Statement::FunctionDeclaration {
                name,
                params,
                body: statements,
            })
        } else {
            Err(ParserError::ExpectedBlock)
        }
    }

    fn parse_block_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect_token(Token::LeftBrace)?;
        let mut statements = Vec::new();
        
        while let Some(TokenWithPosition { token, .. }) = &self.current_token {
            if *token == Token::RightBrace {
                break;
            }
            statements.push(self.parse_statement()?);
        }
        
        self.expect_token(Token::RightBrace)?;
        Ok(Statement::Block(statements))
    }

    fn parse_if_statement(&mut self) -> Result<Statement, ParserError> {
        self.advance_token(); // consume 'if'
        
        self.expect_token(Token::LeftParen)?;
        let condition = self.parse_expression(0)?;
        self.expect_token(Token::RightParen)?;
        
        // Für den Then-Branch erwarten wir einen Block oder ein einzelnes Statement
        let then_branch = if let Some(TokenWithPosition { token: Token::LeftBrace, .. }) = &self.current_token {
            Box::new(self.parse_block_statement()?)
        } else {
            let stmt = self.parse_statement()?;
            Box::new(Statement::Block(vec![stmt]))
        };
        
        // Für den Else-Branch das gleiche
        let else_branch = if let Some(TokenWithPosition { token: Token::Else, .. }) = &self.current_token {
            self.advance_token(); // consume 'else'
            if let Some(TokenWithPosition { token: Token::LeftBrace, .. }) = &self.current_token {
                Some(Box::new(self.parse_block_statement()?))
            } else {
                let stmt = self.parse_statement()?;
                Some(Box::new(Statement::Block(vec![stmt])))
            }
        } else {
            None
        };
        
        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn parse_while_statement(&mut self) -> Result<Statement, ParserError> {
        self.advance_token(); // consume 'while'
        
        self.expect_token(Token::LeftParen)?;
        let condition = self.parse_expression(0)?;
        self.expect_token(Token::RightParen)?;
        
        let body = Box::new(self.parse_statement()?);
        
        Ok(Statement::While {
            condition,
            body,
        })
    }

    fn parse_for_statement(&mut self) -> Result<Statement, ParserError> {
        self.advance_token(); // consume 'for'
        
        self.expect_token(Token::LeftParen)?;
        
        // Initializer
        let initializer = if let Some(token) = &self.current_token {
            if token.token == Token::Semicolon {
                None
            } else {
                Some(Box::new(self.parse_statement()?))
            }
        } else {
            return Err(ParserError::UnexpectedEOF);
        };
        
        // Condition
        let condition = if let Some(token) = &self.current_token {
            if token.token == Token::Semicolon {
                None
            } else {
                Some(self.parse_expression(0)?)
            }
        } else {
            return Err(ParserError::UnexpectedEOF);
        };
        self.expect_token(Token::Semicolon)?;
        
        // Increment
        let increment = if let Some(token) = &self.current_token {
            if token.token == Token::RightParen {
                None
            } else {
                Some(self.parse_expression(0)?)
            }
        } else {
            return Err(ParserError::UnexpectedEOF);
        };
        self.expect_token(Token::RightParen)?;
        
        let body = Box::new(self.parse_statement()?);
        
        Ok(Statement::For {
            initializer,
            condition,
            increment,
            body,
        })
    }

    fn parse_switch_statement(&mut self) -> Result<Statement, ParserError> {
        self.advance_token(); // consume 'switch'
        
        self.expect_token(Token::LeftParen)?;
        let condition = self.parse_expression(0)?;
        self.expect_token(Token::RightParen)?;
        
        self.expect_token(Token::LeftBrace)?;
        
        let mut cases = Vec::new();
        let mut default = None;
        
        while let Some(token) = &self.current_token {
            match token.token {
                Token::RightBrace => break,
                Token::Case => {
                    self.advance_token();
                    let case_value = self.parse_expression(0)?;
                    self.expect_token(Token::Colon)?;
                    
                    let mut statements = Vec::new();
                    while let Some(token) = &self.current_token {
                        match token.token {
                            Token::Case | Token::Default | Token::RightBrace => break,
                            _ => statements.push(self.parse_statement()?),
                        }
                    }
                    cases.push((case_value, statements));
                },
                Token::Default => {
                    self.advance_token();
                    self.expect_token(Token::Colon)?;
                    
                    let mut statements = Vec::new();
                    while let Some(token) = &self.current_token {
                        match token.token {
                            Token::Case | Token::Default | Token::RightBrace => break,
                            _ => statements.push(self.parse_statement()?),
                        }
                    }
                    default = Some(statements);
                },
                _ => return Err(ParserError::UnexpectedToken),
            }
        }
        
        self.expect_token(Token::RightBrace)?;
        
        Ok(Statement::Switch {
            condition,
            cases,
            default,
        })
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        self.advance_token(); // consume 'return'
        
        let value = if let Some(token) = &self.current_token {
            if token.token == Token::Semicolon {
                None
            } else {
                Some(self.parse_expression(0)?)
            }
        } else {
            return Err(ParserError::UnexpectedEOF);
        };
        
        self.expect_token(Token::Semicolon)?;
        
        Ok(Statement::Return(value))
    }

    fn parse_function_call(&mut self, callee: Expression) -> Result<Expression, ParserError> {
        self.advance_token(); // consume '('
        let mut arguments = Vec::new();
        
        if let Some(token) = &self.current_token {
            if token.token != Token::RightParen {
                loop {
                    // Check for unary minus
                    let mut is_negative = false;
                    if let Some(TokenWithPosition { token: Token::Minus, .. }) = &self.current_token {
                        is_negative = true;
                        self.advance_token();
                    }
                    
                    let mut arg = self.parse_expression(0)?;
                    
                    // Apply the negative sign if needed
                    if is_negative {
                        if let Expression::Number(n) = arg {
                            arg = Expression::Number(-n);
                        }
                    }
                    
                    arguments.push(arg);
                    
                    if let Some(token) = &self.current_token {
                        match token.token {
                            Token::RightParen => break,
                            Token::Comma => {
                                self.advance_token();
                                continue;
                            }
                            _ => return Err(ParserError::UnexpectedToken),
                        }
                    } else {
                        return Err(ParserError::UnexpectedEOF);
                    }
                }
            }
        }
        
        self.expect_token(Token::RightParen)?;
        Ok(Expression::Call { callee: Box::new(callee), arguments })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_variable_declaration() {
        let input = "var x = 42;";
        let mut parser = Parser::new(input);
        let result = parser.parse_program().unwrap();
        
        assert_eq!(result.len(), 1);
        if let Statement::VarDeclaration { name, initializer } = &result[0] {
            assert_eq!(name, "x");
            if let Some(Expression::Number(value)) = initializer {
                assert_eq!(*value, 42.0);
            } else {
                panic!("Expected number initializer");
            }
        } else {
            panic!("Expected variable declaration");
        }
    }

    #[test]
    fn test_parse_function_declaration() {
        let input = "fn add(x, y) { return x + y; }";
        let mut parser = Parser::new(input);
        let result = parser.parse_program().unwrap();
        
        assert_eq!(result.len(), 1);
        if let Statement::FunctionDeclaration { name, params, body } = &result[0] {
            assert_eq!(name, "add");
            assert_eq!(params, &vec!["x".to_string(), "y".to_string()]);
            assert_eq!(body.len(), 1);
        } else {
            panic!("Expected function declaration");
        }
    }

    #[test]
    fn test_parse_if_statement() {
        let input = "if (x > 5) { return true; } else { return false; }";
        let mut parser = Parser::new(input);
        let result = parser.parse_program().unwrap();
        
        assert_eq!(result.len(), 1);
        if let Statement::If { condition, then_branch: _, else_branch } = &result[0] {
            if let Expression::Binary { operator, .. } = condition {
                assert_eq!(*operator, BinaryOperator::Greater);
            } else {
                panic!("Expected binary expression");
            }
            assert!(else_branch.is_some());
        } else {
            panic!("Expected if statement");
        }
    }

    #[test]
    fn test_parse_binary_expression() {
        let input = "2 + 3 * 4;";
        let mut parser = Parser::new(input);
        let result = parser.parse_program().unwrap();
        
        assert_eq!(result.len(), 1);
        if let Statement::Expression(Expression::Binary { left, operator, right }) = &result[0] {
            assert_eq!(*operator, BinaryOperator::Add);
            if let Expression::Number(left_val) = &**left {
                assert_eq!(*left_val, 2.0);
            } else {
                panic!("Expected number on left side");
            }
            if let Expression::Binary { operator, .. } = &**right {
                assert_eq!(*operator, BinaryOperator::Multiply);
            } else {
                panic!("Expected multiplication on right side");
            }
        } else {
            panic!("Expected binary expression");
        }
    }

    #[test]
    fn test_parse_function_call() {
        let input = "print(\"Hello\", 42);";
        let mut parser = Parser::new(input);
        let result = parser.parse_program().unwrap();
        
        assert_eq!(result.len(), 1);
        if let Statement::Expression(Expression::Call { callee, arguments }) = &result[0] {
            if let Expression::Identifier(name) = &**callee {
                assert_eq!(name, "print");
            } else {
                panic!("Expected function identifier");
            }
            assert_eq!(arguments.len(), 2);
        } else {
            panic!("Expected function call");
        }
    }

    #[test]
    fn test_parse_block() {
        let input = "{ var x = 1; x = x + 1; }";
        let mut parser = Parser::new(input);
        let result = parser.parse_program().unwrap();
        
        assert_eq!(result.len(), 1);
        if let Statement::Block(statements) = &result[0] {
            assert_eq!(statements.len(), 2);
        } else {
            panic!("Expected block statement");
        }
    }

    #[test]
    fn test_parse_error_unexpected_token() {
        let input = "var 42;";
        let mut parser = Parser::new(input);
        let result = parser.parse_program();
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_unexpected_eof() {
        let input = "var x =";
        let mut parser = Parser::new(input);
        let result = parser.parse_program();
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_expected_block() {
        let input = "fn test() return x;";
        let mut parser = Parser::new(input);
        let result = parser.parse_program();
        assert!(result.is_err());
    }
} 