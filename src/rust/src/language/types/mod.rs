// Token-Definitionen für unsere Sprache
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    Var,
    Fn,
    If,
    Else,
    While,
    For,
    Switch,
    Case,
    Default,
    Out,
    Return,
    True,   // Neues Token
    False,  // Neues Token
    
    // Literale
    Identifier(String),
    Number(f64),
    String(String),
    
    // Operatoren
    Plus,
    Minus,
    Multiply,
    Divide,
    Assign,
    Equals,
    NotEquals,
    Greater,
    Less,
    GreaterEquals,
    LessEquals,
    And,
    Or,
    Not,
    
    // Delimiters
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    Comma,        // ,
    Semicolon,    // ;
    Colon,        // :
    
    // Spezielle Token
    EOF,
}

// Position im Quellcode für bessere Fehlermeldungen
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

// Token mit Position
#[derive(Debug, Clone)]
pub struct TokenWithPosition {
    pub token: Token,
    pub position: Position,
} 