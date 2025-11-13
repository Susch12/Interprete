// AST - Abstract Syntax Tree
// Representa la estructura sintáctica del programa

use crate::lexer::TokenInfo;

// ============================================================================
// UBICACIÓN EN EL CÓDIGO FUENTE
// ============================================================================

#[derive(Debug, Clone)]
pub struct Location {
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

impl Location {
    pub fn from_token(token: &TokenInfo) -> Self {
        Self {
            line: token.line,
            column: token.column,
            length: token.length,
        }
    }

    pub fn unknown() -> Self {
        Self {
            line: 0,
            column: 0,
            length: 0,
        }
    }
}

// ============================================================================
// PROGRAMA COMPLETO
// ============================================================================

#[derive(Debug, Clone)]
pub struct Program {
    pub nombre: String,
    pub definiciones: Definitions,
    pub location: Location,
}

// ============================================================================
// DEFINICIONES
// ============================================================================

#[derive(Debug, Clone)]
pub struct Definitions {
    pub maquinas: Vec<MaquinaDecl>,
    pub concentradores: Vec<ConcentradorDecl>,
    pub coaxiales: Vec<CoaxialDecl>,
    pub location: Location,
}

impl Definitions {
    pub fn empty() -> Self {
        Self {
            maquinas: Vec::new(),
            concentradores: Vec::new(),
            coaxiales: Vec::new(),
            location: Location::unknown(),
        }
    }
}

// ============================================================================
// DECLARACIÓN DE MÁQUINA
// ============================================================================

#[derive(Debug, Clone)]
pub struct MaquinaDecl {
    pub nombre: String,
    pub location: Location,
}

// ============================================================================
// DECLARACIÓN DE CONCENTRADOR
// ============================================================================

#[derive(Debug, Clone)]
pub struct ConcentradorDecl {
    pub nombre: String,
    pub puertos: i32,
    pub tiene_coaxial: bool, // true si se declara con .1
    pub location: Location,
}

// ============================================================================
// DECLARACIÓN DE COAXIAL
// ============================================================================

#[derive(Debug, Clone)]
pub struct CoaxialDecl {
    pub nombre: String,
    pub longitud: i32,
    pub location: Location,
}

// ============================================================================
// HELPER: Para imprimir el AST de manera legible
// ============================================================================

impl Program {
    pub fn pretty_print(&self) {
        println!("\n{}", "═".repeat(80));
        println!("AST del Programa: {}", self.nombre);
        println!("{}", "═".repeat(80));
        
        self.definiciones.pretty_print();
        
        println!("{}\n", "═".repeat(80));
    }
}

impl Definitions {
    pub fn pretty_print(&self) {
        if !self.maquinas.is_empty() {
            println!("\n📦 Máquinas declaradas: {}", self.maquinas.len());
            for (i, maq) in self.maquinas.iter().enumerate() {
                println!("   {}. {} (línea {})", i + 1, maq.nombre, maq.location.line);
            }
        }

        if !self.concentradores.is_empty() {
            println!("\n🔌 Concentradores declarados: {}", self.concentradores.len());
            for (i, conc) in self.concentradores.iter().enumerate() {
                let coax_info = if conc.tiene_coaxial { " + coaxial" } else { "" };
                println!("   {}. {} = {} puertos{} (línea {})", 
                         i + 1, conc.nombre, conc.puertos, coax_info, conc.location.line);
            }
        }

        if !self.coaxiales.is_empty() {
            println!("\n📡 Cables coaxiales declarados: {}", self.coaxiales.len());
            for (i, coax) in self.coaxiales.iter().enumerate() {
                println!("   {}. {} = {}m (línea {})", 
                         i + 1, coax.nombre, coax.longitud, coax.location.line);
            }
        }
    }
}
