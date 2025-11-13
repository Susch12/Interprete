// Parser - Análisis Sintáctico
// Convierte tokens en AST

use crate::lexer::{Token, TokenInfo};
use crate::ast::*;
use colored::*;

// ============================================================================
// ERROR DE PARSEO
// ============================================================================

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub location: Location,
}

impl ParseError {
    pub fn new(message: String, location: Location) -> Self {
        Self { message, location }
    }
}

// ============================================================================
// PARSER
// ============================================================================

pub struct Parser {
    tokens: Vec<TokenInfo>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        Self { tokens, current: 0 }
    }

    // ========== UTILIDADES ==========

    fn peek(&self) -> &Token {
        if self.current < self.tokens.len() {
            &self.tokens[self.current].token
        } else {
            &Token::Error
        }
    }

    fn peek_info(&self) -> Option<&TokenInfo> {
        if self.current < self.tokens.len() {
            Some(&self.tokens[self.current])
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<&TokenInfo> {
        if self.current < self.tokens.len() {
            self.current += 1;
            Some(&self.tokens[self.current - 1])
        } else {
            None
        }
    }

    fn expect(&mut self, expected: Token) -> Result<&TokenInfo, ParseError> {
        let current_info = self.peek_info().cloned();
        
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(&expected) {
            self.advance()
                .ok_or_else(|| ParseError::new(
                    "Token inesperado al final del archivo".into(),
                    Location::unknown(),
                ))
        } else {
            let loc = current_info
                .map(|t| Location::from_token(&t))
                .unwrap_or_else(Location::unknown);
            
            Err(ParseError::new(
                format!("Se esperaba {:?}, se encontró {:?}", expected, self.peek()),
                loc,
            ))
        }
    }

    fn current_location(&self) -> Location {
        self.peek_info()
            .map(Location::from_token)
            .unwrap_or_else(Location::unknown)
    }

    // ========== PARSER PRINCIPAL ==========

    pub fn parse(&mut self) -> Result<Program, Vec<ParseError>> {
        let mut errors = Vec::new();

        match self.parse_programa() {
            Ok(program) => {
                if errors.is_empty() {
                    Ok(program)
                } else {
                    Err(errors)
                }
            }
            Err(e) => {
                errors.push(e);
                Err(errors)
            }
        }
    }

    // ========== PROGRAMA ==========
    // programa ::= "programa" IDENTIFICADOR ";" definiciones

    fn parse_programa(&mut self) -> Result<Program, ParseError> {
        let loc = self.current_location();

        // Esperar "programa"
        self.expect(Token::Programa)?;

        // Esperar identificador (nombre del programa)
        let nombre = match self.peek() {
            Token::Identificador(name) => {
                let n = name.clone();
                self.advance();
                n
            }
            _ => {
                return Err(ParseError::new(
                    "Se esperaba nombre del programa".into(),
                    self.current_location(),
                ));
            }
        };

        // Esperar punto y coma
        self.expect(Token::PuntoYComa)?;

        // Parsear definiciones
        let definiciones = self.parse_definiciones()?;

        // Por ahora, simplificado - esperamos fin.
        // En fases posteriores agregaremos módulos y sentencias
        self.expect(Token::Inicio)?;
        self.expect(Token::Fin)?;
        self.expect(Token::Punto)?;

        Ok(Program {
            nombre,
            definiciones,
            location: loc,
        })
    }

    // ========== DEFINICIONES ==========
    // definiciones ::= define_maquinas? define_concentradores? define_coaxial?

    fn parse_definiciones(&mut self) -> Result<Definitions, ParseError> {
        let loc = self.current_location();
        let mut maquinas = Vec::new();
        let mut concentradores = Vec::new();
        let mut coaxiales = Vec::new();

        // Intentar parsear define maquinas
        if self.peek() == &Token::Define {
            self.advance();
            
            match self.peek() {
                Token::Maquinas => {
                    self.advance();
                    maquinas = self.parse_lista_maquinas()?;
                    self.expect(Token::PuntoYComa)?;
                }
                _ => {
                    // No es define maquinas, retroceder
                    self.current -= 1;
                }
            }
        }

        // Intentar parsear define concentradores
        if self.peek() == &Token::Define {
            self.advance();
            
            match self.peek() {
                Token::Concentradores => {
                    self.advance();
                    concentradores = self.parse_lista_concentradores()?;
                    self.expect(Token::PuntoYComa)?;
                }
                _ => {
                    // No es define concentradores, retroceder
                    self.current -= 1;
                }
            }
        }

        // ⚡ Intentar parsear define coaxial o segmento
        if self.peek() == &Token::Define {
            self.advance();
            
            match self.peek() {
                Token::Coaxial | Token::Segmento => {
                    self.advance();
                    coaxiales = self.parse_lista_coaxial()?;
                    self.expect(Token::PuntoYComa)?;
                }
                _ => {
                    // No es define coaxial/segmento, retroceder
                    self.current -= 1;
                }
            }
        }

        Ok(Definitions {
            maquinas,
            concentradores,
            coaxiales,
            location: loc,
        })
    }

    // ========== LISTA DE MÁQUINAS ==========
    // lista_ids ::= IDENTIFICADOR ("," IDENTIFICADOR)*

    fn parse_lista_maquinas(&mut self) -> Result<Vec<MaquinaDecl>, ParseError> {
        let mut maquinas = Vec::new();

        loop {
            let loc = self.current_location();

            match self.peek() {
                Token::Identificador(nombre) => {
                    let n = nombre.clone();
                    self.advance();

                    maquinas.push(MaquinaDecl {
                        nombre: n,
                        location: loc,
                    });
                }
                _ => {
                    return Err(ParseError::new(
                        "Se esperaba identificador de máquina".into(),
                        self.current_location(),
                    ));
                }
            }

            // Si no hay coma, terminar
            if self.peek() != &Token::Coma {
                break;
            }
            self.advance(); // Consumir coma
        }

        Ok(maquinas)
    }

    // ========== LISTA DE CONCENTRADORES ==========
    // def_concentrador ::= IDENTIFICADOR "=" NUMERO ("." "1")?

    fn parse_lista_concentradores(&mut self) -> Result<Vec<ConcentradorDecl>, ParseError> {
        let mut concentradores = Vec::new();

        loop {
            let loc = self.current_location();

            // Nombre del concentrador
            let nombre = match self.peek() {
                Token::Identificador(n) => {
                    let name = n.clone();
                    self.advance();
                    name
                }
                _ => {
                    return Err(ParseError::new(
                        "Se esperaba nombre de concentrador".into(),
                        self.current_location(),
                    ));
                }
            };

            // Igual
            self.expect(Token::Igual)?;

            // Número de puertos
            let puertos = match self.peek() {
                Token::Numero(p) => {
                    let ports = *p;
                    self.advance();
                    ports
                }
                _ => {
                    return Err(ParseError::new(
                        "Se esperaba número de puertos".into(),
                        self.current_location(),
                    ));
                }
            };

            // ⚡ Verificar si tiene .1 (salida coaxial)
            let tiene_coaxial = if self.peek() == &Token::Punto {
                self.advance();
                match self.peek() {
                    Token::Numero(1) => {
                        self.advance();
                        true
                    }
                    _ => {
                        return Err(ParseError::new(
                            "Después del punto debe ir 1 para indicar salida coaxial".into(),
                            self.current_location(),
                        ));
                    }
                }
            } else {
                false
            };

            concentradores.push(ConcentradorDecl {
                nombre,
                puertos,
                tiene_coaxial,
                location: loc,
            });

            // Si no hay coma, terminar
            if self.peek() != &Token::Coma {
                break;
            }
            self.advance(); // Consumir coma
        }

        Ok(concentradores)
    }

    // ========== LISTA DE COAXIALES ==========
    // def_coaxial ::= IDENTIFICADOR "=" NUMERO

    fn parse_lista_coaxial(&mut self) -> Result<Vec<CoaxialDecl>, ParseError> {
        let mut coaxiales = Vec::new();

        loop {
            let loc = self.current_location();

            // Nombre del coaxial
            let nombre = match self.peek() {
                Token::Identificador(n) => {
                    let name = n.clone();
                    self.advance();
                    name
                }
                _ => {
                    return Err(ParseError::new(
                        "Se esperaba nombre de coaxial".into(),
                        self.current_location(),
                    ));
                }
            };

            // Igual
            self.expect(Token::Igual)?;

            // Longitud
            let longitud = match self.peek() {
                Token::Numero(l) => {
                    let len = *l;
                    self.advance();
                    len
                }
                _ => {
                    return Err(ParseError::new(
                        "Se esperaba longitud del coaxial".into(),
                        self.current_location(),
                    ));
                }
            };

            coaxiales.push(CoaxialDecl {
                nombre,
                longitud,
                location: loc,
            });

            // Si no hay coma, terminar
            if self.peek() != &Token::Coma {
                break;
            }
            self.advance(); // Consumir coma
        }

        Ok(coaxiales)
    }
}

// ============================================================================
// HELPER: Reportar errores de parseo
// ============================================================================

pub fn report_parse_errors(errors: &[ParseError], source: &str) {
    println!("\n{} Se encontraron {} error(es) de parseo:\n", 
             "✗".red().bold(), errors.len());

    for (i, error) in errors.iter().enumerate() {
        println!("{}", format!("Error #{}", i + 1).red().bold());
        println!("  {} en línea {}, columna {}", 
                 "Posición:".yellow(),
                 error.location.line,
                 error.location.column);
        println!("  {} {}", "Mensaje:".yellow(), error.message);

        // Mostrar contexto
        if error.location.line > 0 {
            if let Some(line) = source.lines().nth(error.location.line - 1) {
                println!("\n  {}:", "Contexto".cyan());
                
                // Línea con error
                println!("  {:3} | {}", 
                         error.location.line.to_string().red().bold(), 
                         line);
                
                // Indicador visual
                let spaces = " ".repeat(error.location.column + 5);
                let carets = "^".repeat(error.location.length.max(1));
                println!("       {}{} {}", 
                         spaces, 
                         carets.red().bold(),
                         "aquí".red());
            }
        }
        
        println!();
    }
}
