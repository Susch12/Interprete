// src/error.rs
// Sistema de manejo de errores con colores y contexto

use std::fmt;

#[derive(Debug, Clone)]
pub enum DiagnosticKind {
    LexicalError,
    SyntaxError,
    SemanticError,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub line: usize,
    pub column: usize,
    pub message: String,
}

impl Diagnostic {
    pub fn lexical_error(line: usize, column: usize, message: String) -> Self {
        Diagnostic {
            kind: DiagnosticKind::LexicalError,
            line,
            column,
            message,
        }
    }
    
    pub fn syntax_error(line: usize, column: usize, message: String) -> Self {
        Diagnostic {
            kind: DiagnosticKind::SyntaxError,
            line,
            column,
            message,
        }
    }
    
    pub fn semantic_error(line: usize, column: usize, message: String) -> Self {
        Diagnostic {
            kind: DiagnosticKind::SemanticError,
            line,
            column,
            message,
        }
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let kind_str = match self.kind {
            DiagnosticKind::LexicalError => "Error Léxico",
            DiagnosticKind::SyntaxError => "Error Sintáctico",
            DiagnosticKind::SemanticError => "Error Semántico",
        };
        
        write!(
            f,
            "{} en línea {}, columna {}: {}",
            kind_str, self.line, self.column, self.message
        )
    }
}
