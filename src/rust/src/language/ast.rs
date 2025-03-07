#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    // Literale
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(String),

    // Mathematische Operationen
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },

    // Funktionsaufruf
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    // Variablendeklaration
    VarDeclaration {
        name: String,
        initializer: Option<Expression>,
    },

    // Funktionsdeklaration
    FunctionDeclaration {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },

    // Kontrollstrukturen
    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    While {
        condition: Expression,
        body: Box<Statement>,
    },
    For {
        initializer: Option<Box<Statement>>,
        condition: Option<Expression>,
        increment: Option<Expression>,
        body: Box<Statement>,
    },

    // Switch-Case
    Switch {
        condition: Expression,
        cases: Vec<(Expression, Vec<Statement>)>,
        default: Option<Vec<Statement>>,
    },

    // Block von Statements
    Block(Vec<Statement>),

    // Expression-Statement
    Expression(Expression),

    // Return-Statement
    Return(Option<Expression>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    // Arithmetische Operatoren
    Add,
    Subtract,
    Multiply,
    Divide,

    // Vergleichsoperatoren
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,

    // Logische Operatoren
    And,
    Or,

    // Zuweisungsoperator
    Assign,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Minus,
    Not,
}

// Implementierung für BinaryOperator
impl BinaryOperator {
    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOperator::Assign => 1,
            BinaryOperator::Or => 2,
            BinaryOperator::And => 3,
            BinaryOperator::Equal | BinaryOperator::NotEqual => 4,
            BinaryOperator::Less | BinaryOperator::LessEqual |
            BinaryOperator::Greater | BinaryOperator::GreaterEqual => 5,
            BinaryOperator::Add | BinaryOperator::Subtract => 6,
            BinaryOperator::Multiply | BinaryOperator::Divide => 7,
        }
    }
}

// Hilfsfunktionen für AST-Erstellung
impl Expression {
    pub fn number(value: f64) -> Self {
        Expression::Number(value)
    }

    pub fn string(value: String) -> Self {
        Expression::String(value)
    }

    pub fn boolean(value: bool) -> Self {
        Expression::Boolean(value)
    }

    pub fn identifier(name: String) -> Self {
        Expression::Identifier(name)
    }

    pub fn binary(left: Expression, operator: BinaryOperator, right: Expression) -> Self {
        Expression::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn unary(operator: UnaryOperator, operand: Expression) -> Self {
        Expression::Unary {
            operator,
            operand: Box::new(operand),
        }
    }

    pub fn call(callee: Expression, arguments: Vec<Expression>) -> Self {
        Expression::Call {
            callee: Box::new(callee),
            arguments,
        }
    }
} 