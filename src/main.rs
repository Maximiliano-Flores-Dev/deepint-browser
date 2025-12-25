mod network;
mod crawler;
mod protocols;
mod assets;
mod ai_engine;

use std::io::{self, Write};
use network::DeepRequest;
use protocols::HexIdentity;

fn main() {
    // 1. Limpiar pantalla y mostrar el banner de DeepInt
    print!("{}[2J{}[1;1H", 27 as char, 27 as char); 
    assets::print_banner();
    
    println!("\x1b[1;32m[+] Estado:\x1b[0m Sistema Operativo y Protegido.");
    println!("\x1b[1;32m[+] IA:\x1b[0m Ollama (llama2-uncensored) vinculada.");

    loop {
        print!("\n\x1b[1;35mDeepInt > \x1b[0m");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim();

        match command {
            "help" => {
                println!("\n\x1b[1;33mComandos de Inteligencia:\x1b[0m");
                println!("  scan      - Inicia el Crawler de Python (Onion)");
                println!("  list      - Muestra los sitios guardados en master_db.json");
                println!("  ai-scan   - Analiza la base de datos con IA sin censura");
                println!("  hex-test  - Prueba la resolución de protocolos Hex://");
                println!("  exit      - Apagar motor y salir");
            },

            "scan" => {
                println!("\n\x1b[1;36m[*] Lanzando Onion Crawler recursivo...\x1b[0m");
                let status = std::process::Command::new("python")
                    .arg("src/crawler/scripts/crawler_engine.py")
                    .status();
                
                match status {
                    Ok(_) => println!("\x1b[1;32m[OK] Ingesta finalizada.\x1b[0m"),
                    Err(e) => println!("\x1b[1;31m[!] Error al ejecutar Python: {}\x1b[0m", e),
                }
            },

            "list" => {
                let sites = crawler::load_ingested_data();
                if sites.is_empty() {
                    println!("\x1b[1;31m[!] La base de datos está vacía. Ejecuta 'scan' primero.\x1b[0m");
                } else {
                    println!("\n\x1b[1;34m--- BASE DE DATOS LOCAL ---\x1b[0m");
                    for (i, site) in sites.iter().enumerate() {
                        println!("{}. [{}] - {}", i + 1, site.status, site.url);
                        println!("   Título: {}", site.title.as_deref().unwrap_or("N/A"));
                    }
                }
            },

            "ai-scan" => {
                print!("\n\x1b[1;35m[?] Objetivo de inteligencia (ej: ethical hacking): \x1b[0m");
                io::stdout().flush().unwrap();
                let mut target = String::new();
                io::stdin().read_line(&mut target).unwrap();
                let target = target.trim();

                let sites = crawler::load_ingested_data();
                println!("\n\x1b[1;36m[*] Iniciando análisis semántico local...\x1b[0m");

                for site in sites {
                    let content = format!("{} {}", site.url, site.title.as_deref().unwrap_or(""));
                    let report = ai_engine::analyze_with_ai(&content, target);
                    println!("\n\x1b[1;33m>>> URL:\x1b[0m {}", site.url);
                    println!("\x1b[1;32m>>> IA:\x1b[0m {}", report);
                }
            },

            "hex-test" => {
                let test_url = "hex://maxflores.id";
                println!("\n\x1b[1;34m[*] Resolviendo identidad en Red Hex...\x1b[0m");
                if let Some(id) = HexIdentity::resolve(test_url) {
                    println!("✅ Identidad: {}", id.alias);
                    println!("🔑 Key: {}", id.public_key);
                }
            },

            "exit" => {
                println!("\x1b[1;31m[!] DeepInt se está cerrando. Borrando trazas de memoria...\x1b[0m");
                break;
            },

            _ => {
                if command.starts_with("http") {
                    let req = DeepRequest::new(command);
                    req.dispatch();
                } else {
                    println!("\x1b[1;31m[!] Comando desconocido. Escribe 'help'.\x1b[0m");
                }
            }
        }
    }
}
