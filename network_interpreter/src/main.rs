use colored::*;
use std::env;
use std::fs;
use std::process;

mod lexer;

use lexer::Lexer;

fn main() {
    println!("{}", "=== FASE 1.2: ANALIZADOR LÉXICO AVANZADO ===".cyan().bold());
    
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("{}", "Error: No se especificó archivo de entrada".red().bold());
        eprintln!("Uso: {} <archivo.net>", args[0]);
        eprintln!("\n{}", "Archivos de prueba disponibles:".yellow());
        eprintln!("  {} - Ejemplo simple", "test_simple.net".green());
        eprintln!("  {} - Ejemplo complejo del PDF", "test_complejo.net".green());
        eprintln!("  {} - Test de palabra 'segmento'", "test_segmento.net".green());
        eprintln!("  {} - Test con errores léxicos", "test_errores.net".green());
        process::exit(1);
    }

    let filename = &args[1];

    // Leer archivo fuente
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{} {}: {}", "Error al leer archivo".red().bold(), filename, e);
            process::exit(1);
        }
    };

    println!("{} {}\n", "Archivo:".green(), filename);
    
    // Mostrar estadísticas del código fuente
    let lines = source.lines().count();
    let chars = source.chars().count();
    let non_empty_lines = source.lines().filter(|l| !l.trim().is_empty()).count();
    
    println!("{}", "Estadísticas del código:".cyan());
    println!("  Líneas totales: {}", lines);
    println!("  Líneas no vacías: {}", non_empty_lines);
    println!("  Caracteres: {}", chars);
    println!();

    println!("{}", "Código fuente:".yellow());
    println!("{}", "─".repeat(80));
    
    // Mostrar con números de línea
    for (i, line) in source.lines().enumerate() {
        println!("{:3} | {}", (i + 1).to_string().blue(), line);
    }
    
    println!("{}", "─".repeat(80));
    println!();

    // ANÁLISIS LÉXICO
    println!("{}", "📝 Analizando léxicamente...".yellow().bold());
    
    let mut lexer = Lexer::new(source.clone());
    
    match lexer.tokenize() {
        Ok(tokens) => {
            println!("{} {} tokens generados\n", "✓".green().bold(), tokens.len());
            
            // Estadísticas de tokens
            print_token_statistics(tokens);
            
            // Mostrar tabla de tokens
            println!("\n{}", "Tabla de Tokens:".cyan().bold());
            println!("{}", "─".repeat(90));
            println!("{:<6} {:<8} {:<25} {:<35}", 
                     "Línea", "Columna", "Token", "Lexema");
            println!("{}", "─".repeat(90));
            
            for token_info in tokens {
                let token_str = format!("{:?}", token_info.token);
                let token_display = if token_str.len() > 23 {
                    format!("{}...", &token_str[..20])
                } else {
                    token_str
                };
                
                let lexeme_display = if token_info.lexeme.len() > 33 {
                    format!("{}...", &token_info.lexeme[..30])
                } else {
                    token_info.lexeme.clone()
                };
                
                println!("{:<6} {:<8} {:<25} {:<35}", 
                         token_info.line,
                         token_info.column,
                         token_display,
                         lexeme_display);
            }
            println!("{}", "─".repeat(90));
            
            println!("\n{}", "✅ Análisis léxico completado exitosamente".green().bold());
        }
        Err(errors) => {
            println!("{} Se encontraron {} error(es) léxico(s):\n", 
                     "✗".red().bold(), errors.len());
            
            // Reporte mejorado de errores
            for (i, error) in errors.iter().enumerate() {
                println!("{}", format!("Error #{}", i + 1).red().bold());
                println!("  {} en línea {}, columna {}", 
                         "Posición:".yellow(),
                         error.line,
                         error.column);
                println!("  {} {}", "Mensaje:".yellow(), error.message);
                
                // Mostrar contexto con la línea del error
                if let Some(line) = source.lines().nth(error.line - 1) {
                    println!("\n  {}:", "Contexto".cyan());
                    
                    // Línea anterior (si existe)
                    if error.line > 1 {
                        if let Some(prev_line) = source.lines().nth(error.line - 2) {
                            println!("  {:3} | {}", 
                                     (error.line - 1).to_string().blue(), 
                                     prev_line.dimmed());
                        }
                    }
                    
                    // Línea con error
                    println!("  {:3} | {}", 
                             error.line.to_string().red().bold(), 
                             line);
                    
                    // Indicador visual del error
                    let spaces = " ".repeat(error.column + 5);
                    let carets = "^".repeat(error.length.max(1));
                    println!("       {}{} {}", 
                             spaces, 
                             carets.red().bold(),
                             "aquí".red());
                    
                    // Línea siguiente (si existe)
                    if let Some(next_line) = source.lines().nth(error.line) {
                        println!("  {:3} | {}", 
                                 (error.line + 1).to_string().blue(), 
                                 next_line.dimmed());
                    }
                }
                
                println!();
            }
            
            println!("{}", "❌ Análisis léxico falló".red().bold());
            process::exit(1);
        }
    }
}

fn print_token_statistics(tokens: &[lexer::TokenInfo]) {
    use std::collections::HashMap;
    
    let mut token_counts: HashMap<String, usize> = HashMap::new();
    
    for token_info in tokens {
        let token_type = format!("{:?}", token_info.token)
            .split('(')
            .next()
            .unwrap_or("Unknown")
            .to_string();
        *token_counts.entry(token_type).or_insert(0) += 1;
    }
    
    println!("{}", "Estadísticas de Tokens:".cyan());
    
    let mut sorted_counts: Vec<_> = token_counts.iter().collect();
    sorted_counts.sort_by_key(|(_, count)| std::cmp::Reverse(**count));
    
    // Mostrar top 10
    for (i, (token_type, count)) in sorted_counts.iter().take(10).enumerate() {
        if i == 0 {
            println!("  {} {} ({})", "→".green(), token_type, count);
        } else {
            println!("  {} {} ({})", " ", token_type, count);
        }
    }
    
    if sorted_counts.len() > 10 {
        println!("  ... y {} tipos más", sorted_counts.len() - 10);
    }
}
