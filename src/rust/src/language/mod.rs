pub mod lexer;
pub mod error;
pub mod types;
pub mod ast;
pub mod parser;
pub mod interpreter;

// Re-export wichtiger Komponenten
pub use lexer::*;
pub use error::*;
pub use types::*;
pub use ast::*;
pub use parser::*;
pub use interpreter::*; 