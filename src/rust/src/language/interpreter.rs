use std::collections::HashMap;
use super::ast::{Expression, Statement, BinaryOperator, UnaryOperator};
use super::error::RuntimeError;
use std::sync::Arc;
use napi::threadsafe_function::{ThreadsafeFunction, ErrorStrategy};

pub type NativeFn = fn(Vec<Value>) -> Result<Value, RuntimeError>;

#[derive(Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    NativeFunction(String),  // Nur der Name der Funktion
    Function {
        params: Vec<String>,
        body: Vec<Statement>,
    },
}

impl Value {
    fn as_number(&self) -> Result<f64, RuntimeError> {
        match self {
            Value::Number(n) => Ok(*n),
            _ => Err(RuntimeError::TypeError("Expected a number".to_string())),
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
            Value::NativeFunction(name) => write!(f, "<native fn {}>", name),
            Value::Function { .. } => write!(f, "<fn>"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => (a - b).abs() < f64::EPSILON,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::NativeFunction(a), Value::NativeFunction(b)) => a == b,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn with_enclosing(enclosing: Environment) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), RuntimeError> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            Ok(())
        } else if let Some(enclosing) = &mut self.enclosing {
            enclosing.assign(name, value)
        } else {
            Err(RuntimeError::UndefinedVariable(name.to_string()))
        }
    }

    pub fn get(&self, name: &str) -> Result<Value, RuntimeError> {
        match self.values.get(name) {
            Some(value) => Ok(value.clone()),
            None => {
                match &self.enclosing {
                    Some(enclosing) => enclosing.get(name),
                    None => Err(RuntimeError::UndefinedVariable(name.to_string()))
                }
            }
        }
    }
}

pub struct Interpreter {
    environment: Environment,
    console_callback: Option<ThreadsafeFunction<String>>,
    native_functions: HashMap<String, Arc<dyn Fn(&Interpreter, Vec<Value>) -> Result<Value, RuntimeError> + Send + Sync + 'static>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Interpreter {
            environment: Environment::new(),
            console_callback: None,
            native_functions: HashMap::new(),
        };
        interpreter.define_native_functions();
        interpreter
    }

    pub fn with_callback(callback: ThreadsafeFunction<String>) -> Self {
        let mut interpreter = Interpreter {
            environment: Environment::new(),
            console_callback: Some(callback),
            native_functions: HashMap::new(),
        };
        interpreter.define_native_functions();
        interpreter
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<Value, RuntimeError> {
        let mut result = Value::Null;
        
        for statement in statements {
            result = self.execute_statement(statement)?;
        }
        
        Ok(result)
    }

    fn execute_statement(&mut self, statement: Statement) -> Result<Value, RuntimeError> {
        match statement {
            Statement::Expression(expr) => self.evaluate_expression(expr),
            Statement::VarDeclaration { name, initializer } => {
                let value = match initializer {
                    Some(expr) => self.evaluate_expression(expr)?,
                    None => Value::Null,
                };
                self.environment.define(name.clone(), value.clone());
                Ok(value)
            },
            Statement::Block(statements) => {
                let previous_env = self.environment.clone();
                self.environment = Environment::with_enclosing(previous_env);
                
                let mut result = Value::Null;
                for stmt in statements {
                    result = self.execute_statement(stmt)?;
                }
                
                if let Some(enclosing) = self.environment.enclosing.take() {
                    self.environment = *enclosing;
                }
                
                Ok(result)
            },
            Statement::FunctionDeclaration { name, params, body } => {
                let function = Value::Function {
                    params,
                    body,
                };
                self.environment.define(name, function);
                Ok(Value::Null)
            },
            Statement::Return(expr) => {
                let value = match expr {
                    Some(expr) => self.evaluate_expression(expr)?,
                    None => Value::Null,
                };
                Ok(value)
            },
            Statement::If { condition, then_branch, else_branch } => {
                let condition_value = self.evaluate_expression(condition)?;
                if self.is_truthy(&condition_value) {
                    self.execute_statement(*then_branch)
                } else if let Some(else_stmt) = else_branch {
                    self.execute_statement(*else_stmt)
                } else {
                    Ok(Value::Null)
                }
            },
            Statement::While { condition, body } => {
                let mut result = Value::Null;
                loop {
                    let condition_value = self.evaluate_expression(condition.clone())?;
                    if !self.is_truthy(&condition_value) {
                        break;
                    }
                    result = self.execute_statement(*body.clone())?;
                }
                Ok(result)
            },
            Statement::For { initializer, condition, increment, body } => {
                // Initialisierung
                if let Some(init_stmt) = initializer {
                    self.execute_statement(*init_stmt)?;
                }

                let mut result = Value::Null;
                
                loop {
                    // Überprüfe die Bedingung
                    if let Some(cond) = condition.as_ref() {
                        let condition_value = self.evaluate_expression(cond.clone())?;
                        if !self.is_truthy(&condition_value) {
                            break;
                        }
                    }
                    
                    // Führe den Schleifenkörper aus
                    result = self.execute_statement(*body.clone())?;
                    
                    // Führe das Inkrement aus
                    if let Some(incr) = increment.as_ref() {
                        self.evaluate_expression(incr.clone())?;
                    }
                }
                
                Ok(result)
            },
            Statement::Switch { condition, cases, default } => {
                let value = self.evaluate_expression(condition)?;
                
                // Suche nach einem passenden case
                for (case_value_expr, case_body) in cases {
                    let case_value = self.evaluate_expression(case_value_expr)?;
                    if value == case_value {
                        let mut result = Value::Null;
                        for stmt in case_body {
                            result = self.execute_statement(stmt)?;
                        }
                        return Ok(result);
                    }
                }
                
                // Wenn kein case passt, führe den default-Block aus
                if let Some(default_stmts) = default {
                    let mut result = Value::Null;
                    for stmt in default_stmts {
                        result = self.execute_statement(stmt)?;
                    }
                    Ok(result)
                } else {
                    Ok(Value::Null)
                }
            },
        }
    }

    fn evaluate_expression(&mut self, expression: Expression) -> Result<Value, RuntimeError> {
        match expression {
            Expression::Number(n) => Ok(Value::Number(n)),
            Expression::String(s) => Ok(Value::String(s)),
            Expression::Boolean(b) => Ok(Value::Boolean(b)),
            Expression::Identifier(name) => self.environment.get(&name),
            Expression::Binary { left, operator, right } => {
                // Spezielle Behandlung für Zuweisungen
                if let BinaryOperator::Assign = operator {
                    if let Expression::Identifier(name) = *left {
                        let right_val = self.evaluate_expression(*right)?;
                        self.environment.assign(&name, right_val.clone())?;
                        return Ok(right_val);
                    } else {
                        return Err(RuntimeError::TypeError(
                            "Left side of assignment must be a variable".to_string()
                        ));
                    }
                }
                
                let left_val = self.evaluate_expression(*left)?;
                let right_val = self.evaluate_expression(*right)?;
                self.evaluate_binary_op(operator, left_val, right_val)
            },
            Expression::Unary { operator, operand } => {
                let operand_val = self.evaluate_expression(*operand)?;
                self.evaluate_unary_op(operator, operand_val)
            },
            Expression::Call { callee, arguments } => {
                let callee_val = self.evaluate_expression(*callee)?;
                let mut evaluated_args = Vec::new();
                
                for arg in arguments {
                    evaluated_args.push(self.evaluate_expression(arg)?);
                }
                
                match callee_val {
                    Value::NativeFunction(name) => {
                        if let Some(func) = self.native_functions.get(&name) {
                            func(self, evaluated_args)
                        } else {
                            Err(RuntimeError::UndefinedVariable(name))
                        }
                    },
                    Value::Function { params, body } => {
                        if evaluated_args.len() != params.len() {
                            return Err(RuntimeError::InvalidArgumentCount {
                                expected: params.len(),
                                got: evaluated_args.len(),
                            });
                        }
                        
                        let previous_env = self.environment.clone();
                        self.environment = Environment::with_enclosing(previous_env);
                        
                        for (param, arg) in params.iter().zip(evaluated_args) {
                            self.environment.define(param.clone(), arg);
                        }
                        
                        let result = self.interpret(body)?;
                        
                        if let Some(enclosing) = self.environment.enclosing.take() {
                            self.environment = *enclosing;
                        }
                        
                        Ok(result)
                    },
                    _ => Err(RuntimeError::TypeError(format!(
                        "Cannot call non-function value: {:?}",
                        callee_val
                    ))),
                }
            },
        }
    }

    fn evaluate_binary_op(&self, operator: BinaryOperator, left: Value, right: Value) -> Result<Value, RuntimeError> {
        match (operator, &left, &right) {
            // Arithmetische Operationen
            (BinaryOperator::Add, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (BinaryOperator::Add, Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (BinaryOperator::Subtract, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            (BinaryOperator::Multiply, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            (BinaryOperator::Divide, Value::Number(a), Value::Number(b)) => {
                if *b == 0.0 {
                    Err(RuntimeError::DivisionByZero)
                } else {
                    Ok(Value::Number(a / b))
                }
            },
            
            // Vergleichsoperationen
            (BinaryOperator::Equal, _, _) => Ok(Value::Boolean(left == right)),
            (BinaryOperator::NotEqual, _, _) => Ok(Value::Boolean(left != right)),
            (BinaryOperator::Greater, Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a > b)),
            (BinaryOperator::Less, Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a < b)),
            (BinaryOperator::GreaterEqual, Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a >= b)),
            (BinaryOperator::LessEqual, Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a <= b)),
            
            // Logische Operationen
            (BinaryOperator::And, Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(*a && *b)),
            (BinaryOperator::Or, Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(*a || *b)),
            
            // Ungültige Operationen
            (op, _, _) => Err(RuntimeError::TypeError(format!(
                "Invalid operation: cannot apply {:?} to {:?} and {:?}",
                op, left, right
            ))),
        }
    }

    fn evaluate_unary_op(&self, operator: UnaryOperator, operand: Value) -> Result<Value, RuntimeError> {
        match (operator, operand) {
            (UnaryOperator::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
            (UnaryOperator::Not, Value::Boolean(b)) => Ok(Value::Boolean(!b)),
            _ => Err(RuntimeError::InvalidOperation),
        }
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Boolean(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            _ => true,
        }
    }

    fn define_native_functions(&mut self) {
        // Definiere die out-Funktion
        let out_func = Arc::new(|interpreter: &Interpreter, args: Vec<Value>| {
            if args.is_empty() {
                return Err(RuntimeError::InvalidArgumentCount {
                    expected: 1,
                    got: 0,
                });
            }

            let output = match &args[0] {
                Value::Number(n) => n.to_string(),
                Value::String(s) => s.clone(),
                Value::Boolean(b) => b.to_string(),
                Value::Null => "null".to_string(),
                Value::NativeFunction(name) => format!("[Native Function: {}]", name),
                Value::Function { .. } => "[Function]".to_string(),
            };

            if let Some(callback) = &interpreter.console_callback {
                callback.call(Ok(output.clone()), napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking);
            } else {
                println!("{}", output);
            }

            Ok(Value::Null)
        });

        self.native_functions.insert("out".to_string(), out_func);
        self.environment.define("out".to_string(), Value::NativeFunction("out".to_string()));

        // Definiere die round-Funktion
        let round_func = Arc::new(|_: &Interpreter, args: Vec<Value>| {
            if args.is_empty() {
                return Err(RuntimeError::InvalidArgumentCount {
                    expected: 1,
                    got: 0,
                });
            }

            match &args[0] {
                Value::Number(n) => Ok(Value::Number(n.round())),
                _ => Err(RuntimeError::TypeError("Expected number argument".to_string())),
            }
        });

        self.native_functions.insert("round".to_string(), round_func);
        self.environment.define("round".to_string(), Value::NativeFunction("round".to_string()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::parser::Parser;
    use super::super::ast::{Expression, Statement};

    fn evaluate_str(input: &str) -> Result<Value, RuntimeError> {
        let mut parser = Parser::new(input);
        let statements = parser.parse_program().unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.define_native_functions();
        interpreter.interpret(statements)
    }

    #[test]
    fn test_arithmetic() {
        assert!(matches!(evaluate_str("var x = 5 + 3;").unwrap(), Value::Number(8.0)));
        assert!(matches!(evaluate_str("var x = 10 - 4;").unwrap(), Value::Number(6.0)));
        assert!(matches!(evaluate_str("var x = 3 * 4;").unwrap(), Value::Number(12.0)));
        assert!(matches!(evaluate_str("var x = 15 / 3;").unwrap(), Value::Number(5.0)));
    }

    #[test]
    fn test_variables() {
        let mut interpreter = Interpreter::new();
        let statements = vec![
            Statement::VarDeclaration {
                name: "x".to_string(),
                initializer: Some(Expression::Number(42.0)),
            },
            Statement::Expression(Expression::Identifier("x".to_string())),
        ];
        
        assert!(matches!(interpreter.interpret(statements).unwrap(), Value::Number(42.0)));
    }

    #[test]
    fn test_if_statement() {
        let input = r#"
            var result;
            if (true) {
                result = 1;
            } else {
                result = 2;
            }
            result;
        "#;
        assert!(matches!(evaluate_str(input).unwrap(), Value::Number(1.0)));
    }

    #[test]
    fn test_boolean_operations() {
        let input = r#"
            var x = true;
            var y = false;
            var z = x && y;
            z;
        "#;
        assert!(matches!(evaluate_str(input).unwrap(), Value::Boolean(false)));
    }

    #[test]
    fn test_comparison() {
        assert!(matches!(evaluate_str("var x = 5 > 3;").unwrap(), Value::Boolean(true)));
        assert!(matches!(evaluate_str("var x = 5 < 3;").unwrap(), Value::Boolean(false)));
        assert!(matches!(evaluate_str("var x = 5 == 5;").unwrap(), Value::Boolean(true)));
        assert!(matches!(evaluate_str("var x = 5 != 3;").unwrap(), Value::Boolean(true)));
    }

    #[test]
    fn test_string_operations() {
        assert!(matches!(
            evaluate_str(r#"var x = "Hello" + " World";"#).unwrap(),
            Value::String(s) if s == "Hello World"
        ));
    }

    #[test]
    fn test_division_by_zero() {
        let result = evaluate_str("var x = 5 / 0;");
        assert!(matches!(result, Err(RuntimeError::DivisionByZero)));
    }

    #[test]
    fn test_undefined_variable() {
        let result = evaluate_str("var x = y;");
        assert!(matches!(result, Err(RuntimeError::UndefinedVariable(name)) if name == "y"));
    }

    #[test]
    fn test_block_scope() {
        let input = r#"
            var x = 1;
            {
                var x = 2;
                x = 3;
            }
            x;
        "#;
        assert!(matches!(evaluate_str(input).unwrap(), Value::Number(1.0)));
    }

    #[test]
    fn test_round_function() {
        let mut interpreter = Interpreter::new();
        interpreter.define_native_functions();

        // Normale Rundung
        let input = "{ var x = round(3.14159, 2); x; }";
        let mut parser = Parser::new(input);
        let statements = match parser.parse_program() {
            Ok(stmts) => stmts,
            Err(e) => {
                println!("Parser error for input '{}': {:?}", input, e);
                panic!("Parser error");
            }
        };
        println!("Parsed statements: {:?}", statements);
        
        assert!(matches!(
            interpreter.interpret(statements).unwrap(),
            Value::Number(n) if (n - 3.14).abs() < f64::EPSILON
        ));

        // Rundung auf ganze Zahl
        let input = "{ var x = round(3.7, 0); x; }";
        let mut parser = Parser::new(input);
        let statements = match parser.parse_program() {
            Ok(stmts) => stmts,
            Err(e) => {
                println!("Parser error for input '{}': {:?}", input, e);
                panic!("Parser error");
            }
        };
        println!("Parsed statements: {:?}", statements);
        
        assert!(matches!(
            interpreter.interpret(statements).unwrap(),
            Value::Number(n) if (n - 4.0).abs() < f64::EPSILON
        ));

        // Negative Stellen
        let input = "{ var x = round(1234.5678, -2); x; }";
        let mut parser = Parser::new(input);
        let statements = match parser.parse_program() {
            Ok(stmts) => stmts,
            Err(e) => {
                println!("Parser error for input '{}': {:?}", input, e);
                panic!("Parser error");
            }
        };
        println!("Parsed statements: {:?}", statements);
        
        assert!(matches!(
            interpreter.interpret(statements).unwrap(),
            Value::Number(n) if (n - 1200.0).abs() < f64::EPSILON
        ));

        // Große Anzahl von Dezimalstellen
        let input = "{ var x = round(3.14159, 4); x; }";
        let mut parser = Parser::new(input);
        let statements = match parser.parse_program() {
            Ok(stmts) => stmts,
            Err(e) => {
                println!("Parser error for input '{}': {:?}", input, e);
                panic!("Parser error");
            }
        };
        println!("Parsed statements: {:?}", statements);
        
        assert!(matches!(
            interpreter.interpret(statements).unwrap(),
            Value::Number(n) if (n - 3.1416).abs() < f64::EPSILON
        ));
    }

    #[test]
    fn test_round_function_errors() {
        // Falsche Anzahl von Argumenten
        assert!(matches!(
            evaluate_str("round(3.14);"),
            Err(RuntimeError::InvalidArgumentCount { expected: 2, got: 1 })
        ));

        // Falscher Typ für erstes Argument
        assert!(matches!(
            evaluate_str("round(\"3.14\", 2);"),
            Err(RuntimeError::TypeError(_))
        ));

        // Falscher Typ für zweites Argument
        assert!(matches!(
            evaluate_str("round(3.14, true);"),
            Err(RuntimeError::TypeError(_))
        ));

        // Nicht-ganzzahliges zweites Argument
        assert!(matches!(
            evaluate_str("round(3.14, 2.5);"),
            Err(RuntimeError::TypeError(_))
        ));
    }

    #[test]
    fn test_function_call() {
        // Test für normale Funktionsaufrufe
        let input = r#"
            fn test(x, y) {
                return x + y;
            }
            test(2, 3);
        "#;
        assert!(matches!(evaluate_str(input).unwrap(), Value::Number(n) if (n - 5.0).abs() < f64::EPSILON));
    }

    #[test]
    fn test_function_call_errors() {
        // Test für Aufruf einer nicht existierenden Funktion
        assert!(matches!(
            evaluate_str("nonexistent(1, 2);"),
            Err(RuntimeError::UndefinedVariable(_))
        ));

        // Test für Aufruf mit falscher Argumentanzahl
        let input = r#"
            fn test(x) {
                return x;
            }
            test(1, 2);
        "#;
        assert!(matches!(
            evaluate_str(input),
            Err(RuntimeError::InvalidArgumentCount { expected: 1, got: 2 })
        ));
    }

    #[test]
    fn test_out_function() {
        let mut interpreter = Interpreter::new();
        
        // Test mit verschiedenen Werttypen
        let test_cases = vec![
            ("out(42);", Value::Number(42.0)),
            ("out(\"Hello\");", Value::String("Hello".to_string())),
            ("out(true);", Value::Boolean(true)),
            ("var x = 123; out(x);", Value::Number(123.0)),
        ];

        for (input, expected_value) in test_cases {
            let result = evaluate_str(input);
            assert!(result.is_ok(), "Failed to evaluate: {}", input);
        }
    }

    #[test]
    fn test_out_function_errors() {
        let mut interpreter = Interpreter::new();
        
        // Test ohne Argumente
        let result = evaluate_str("out();");
        assert!(matches!(
            result,
            Err(RuntimeError::InvalidArgumentCount { expected: 1, got: 0 })
        ));
    }
} 