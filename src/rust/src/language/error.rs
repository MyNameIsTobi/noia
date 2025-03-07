#[derive(Debug)]
pub enum LexerError {
    InvalidCharacter(char, usize, usize),  // Zeichen, Zeile, Spalte
    UnterminatedString(usize, usize),      // Zeile, Spalte
    InvalidNumber(String, usize, usize),    // Nummer als String, Zeile, Spalte
    UnterminatedBlockComment(usize, usize), // Zeile, Spalte wo der Kommentar begann
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken,
    UnexpectedEOF,
    ExpectedIdentifier,
    ExpectedBlock,
    InvalidExpression,
    InvalidStatement,
}

#[derive(Debug)]
pub enum RuntimeError {
    UndefinedVariable(String),
    DivisionByZero,
    InvalidOperation,
    TypeError(String),
    InvalidArgumentCount { expected: usize, got: usize },
    ReturnValue(Box<RuntimeError>),
    Custom(String),
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LexerError::InvalidCharacter(c, line, col) => 
                write!(f, "Invalid character '{}' at line {}, column {}", c, line, col),
            LexerError::UnterminatedString(line, col) => 
                write!(f, "Unterminated string at line {}, column {}", line, col),
            LexerError::InvalidNumber(num, line, col) => 
                write!(f, "Invalid number '{}' at line {}, column {}", num, line, col),
            LexerError::UnterminatedBlockComment(line, col) =>
                write!(f, "Unterminated block comment starting at line {}, column {}", line, col),
        }
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedToken => write!(f, "Unexpected token"),
            ParserError::UnexpectedEOF => write!(f, "Unexpected end of file"),
            ParserError::ExpectedIdentifier => write!(f, "Expected identifier"),
            ParserError::ExpectedBlock => write!(f, "Expected block"),
            ParserError::InvalidExpression => write!(f, "Invalid expression"),
            ParserError::InvalidStatement => write!(f, "Invalid statement"),
        }
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuntimeError::UndefinedVariable(name) => 
                write!(f, "Undefined variable '{}'", name),
            RuntimeError::DivisionByZero => 
                write!(f, "Division by zero"),
            RuntimeError::InvalidOperation => 
                write!(f, "Invalid operation"),
            RuntimeError::TypeError(msg) => 
                write!(f, "Type error: {}", msg),
            RuntimeError::InvalidArgumentCount { expected, got } => 
                write!(f, "Invalid argument count: expected {}, got {}", expected, got),
            RuntimeError::ReturnValue(err) => 
                write!(f, "Return value error: {}", err),
            RuntimeError::Custom(msg) => 
                write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for LexerError {}
impl std::error::Error for ParserError {}
impl std::error::Error for RuntimeError {} 