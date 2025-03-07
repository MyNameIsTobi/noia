#[macro_use]
extern crate napi_derive;

use napi::{Error, Result as NapiResult, Status};
use serde::{Deserialize, Serialize};
use sysinfo::{ProcessExt, System, SystemExt, PidExt, CpuExt};
use napi::threadsafe_function::{ThreadsafeFunction, ErrorStrategy};
use std::sync::Arc;
use std::sync::Mutex;
use once_cell::sync::OnceCell;

// Language-Modul einbinden
pub mod language;

// Struct to represent process information
#[derive(Serialize, Deserialize)]
struct ProcessInfo {
    pid: u32,
    name: String,
    memory_usage_kb: u64,
    cpu_usage_percent: f32,
}

// Struct to represent system information
#[derive(Serialize, Deserialize)]
struct SystemInfo {
    processes: Vec<ProcessInfo>,
    total_memory: u64,
    used_memory: u64,
    total_cpu_usage: f32,
}

// Create a static System instance to reuse between calls
static mut SYSTEM: Option<System> = None;

// Initialize the system monitoring
fn get_system() -> &'static mut System {
    #[allow(static_mut_refs)]
    unsafe {
        if SYSTEM.is_none() {
            SYSTEM = Some(System::new_all());
        }
        SYSTEM.as_mut().unwrap()
    }
}

// Find a specific process by name
#[napi]
pub fn find_process(process_name: String) -> NapiResult<String> {
    let sys = get_system();
    sys.refresh_processes();

    for (pid, process) in sys.processes() {
        if process.name().to_lowercase() == process_name.to_lowercase() {
            let process_info = ProcessInfo {
                pid: pid.as_u32(),
                name: process.name().to_string(),
                memory_usage_kb: process.memory(),
                cpu_usage_percent: process.cpu_usage(),
            };

            return Ok(serde_json::to_string(&process_info).unwrap());
        }
    }

    Err(Error::new(
        Status::GenericFailure,
        format!("Process '{}' not found", process_name),
    ))
}

// Get information about a specific process by PID
#[napi]
pub fn get_process_info(pid: u32) -> NapiResult<String> {
    let sys = get_system();
    sys.refresh_processes();

    let pid = sysinfo::Pid::from_u32(pid);
    if let Some(process) = sys.process(pid) {
        let process_info = ProcessInfo {
            pid: pid.as_u32(),
            name: process.name().to_string(),
            memory_usage_kb: process.memory(),
            cpu_usage_percent: process.cpu_usage(),
        };

        return Ok(serde_json::to_string(&process_info).unwrap());
    }

    Err(Error::new(
        Status::GenericFailure,
        format!("Process with PID {} not found", pid),
    ))
}

// Get all running processes
#[napi]
pub fn get_all_processes() -> String {
    let sys = get_system();
    sys.refresh_processes();

    let mut processes = Vec::new();

    for (pid, process) in sys.processes() {
        let process_info = ProcessInfo {
            pid: pid.as_u32(),
            name: process.name().to_string(),
            memory_usage_kb: process.memory(),
            cpu_usage_percent: process.cpu_usage(),
        };
        processes.push(process_info);
    }

    serde_json::to_string(&processes).unwrap()
}

// Search for processes by name pattern (case insensitive)
#[napi]
pub fn search_processes(name_pattern: String) -> String {
    let sys = get_system();
    sys.refresh_processes();

    let mut matching_processes = Vec::new();
    let name_pattern_lower = name_pattern.to_lowercase();

    for (pid, process) in sys.processes() {
        if process.name().to_lowercase().contains(&name_pattern_lower) {
            let process_info = ProcessInfo {
                pid: pid.as_u32(),
                name: process.name().to_string(),
                memory_usage_kb: process.memory(),
                cpu_usage_percent: process.cpu_usage(),
            };
            matching_processes.push(process_info);
        }
    }

    serde_json::to_string(&matching_processes).unwrap()
}

// Get system information
#[napi]
pub fn get_system_info() -> String {
    let sys = get_system();
    sys.refresh_all();

    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let total_cpu_usage = sys.global_cpu_info().cpu_usage();

    let mut processes = Vec::new();
    for (pid, process) in sys.processes() {
        let process_info = ProcessInfo {
            pid: pid.as_u32(),
            name: process.name().to_string(),
            memory_usage_kb: process.memory(),
            cpu_usage_percent: process.cpu_usage(),
        };
        processes.push(process_info);
    }

    let system_info = SystemInfo {
        processes,
        total_memory,
        used_memory,
        total_cpu_usage,
    };

    serde_json::to_string(&system_info).unwrap()
}

// Keep the original sample and add functions for backward compatibility
#[napi]
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[napi]
pub fn execute_code(
    code: String,
    callback: ThreadsafeFunction<String, ErrorStrategy::Fatal>,
) -> Result<String, napi::Error> {
    // Erstelle einen Klon des Callbacks für die Verwendung im Interpreter
    let tsfn = callback.clone();
    
    let mut parser = language::Parser::new(&code);
    let statements = match parser.parse_program() {
        Ok(stmts) => stmts,
        Err(e) => return Err(napi::Error::from_reason(format!("Parse Error: {}", e)))
    };

    // Erstelle den Interpreter mit dem geklonten Callback
    let mut interpreter = language::Interpreter::with_callback(tsfn);
    
    match interpreter.interpret(statements) {
        Ok(_) => Ok("Code successfully executed.".to_string()),
        Err(e) => Err(napi::Error::from_reason(format!("Runtime Error: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    // Language-Tests
    #[test]
    fn test_language_module() {
        // Dieser Test stellt sicher, dass das Language-Modul korrekt eingebunden ist
        use super::language::lexer::Lexer;
        use super::language::types::Token;
        
        let mut lexer = Lexer::new("var x = 42;");
        let tokens: Vec<_> = std::iter::from_fn(|| lexer.next_token().ok())
            .take_while(|t| !matches!(t.token, Token::EOF))
            .collect();
        
        assert!(!tokens.is_empty());
    }
} 