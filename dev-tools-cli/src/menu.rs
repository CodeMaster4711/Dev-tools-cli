use std::io::{self, Write};
use colored::*;

pub fn show_menu() {
    // Schöner Rahmen für das Menü
    println!("{}", "╔══════════════════════════════════════╗".cyan());
    println!("{}", "║          🚀 DEV TOOLS CLI 🚀          ║".cyan().bold());
    println!("{}", "╠══════════════════════════════════════╣".cyan());
    println!("║                                      ║");
    println!("║ {} {}                    ║", "1)".yellow().bold(), "🔧 Development Tools".white());
    println!("║ {} {}                    ║", "2)".yellow().bold(), "📦 Package Manager".white());
    println!("║ {} {}                        ║", "3)".yellow().bold(), "🌐 Network Tools".white());
    println!("║ {} {}                           ║", "4)".yellow().bold(), "📊 System Info".white());
    println!("║ {} {}                            ║", "5)".yellow().bold(), "🚪 Beenden".red());
    println!("║                                      ║");
    println!("{}", "╚══════════════════════════════════════╝".cyan());
    println!();

    print!("{}", "➤ Deine Auswahl: ".bold().green());
    io::stdout().flush().unwrap();

    let mut auswahl = String::new();
    io::stdin().read_line(&mut auswahl).unwrap();
    println!();

    match auswahl.trim() {
        "1" => {
            println!("{}", "✅ Development Tools ausgewählt!".green().bold());
            // Hier kannst du weitere Funktionen aufrufen
        },
        "2" => {
            println!("{}", "✅ Package Manager ausgewählt!".green().bold());
            // Hier kannst du weitere Funktionen aufrufen
        },
        "3" => {
            println!("{}", "✅ Network Tools ausgewählt!".green().bold());
            // Hier kannst du weitere Funktionen aufrufen
        },
        "4" => {
            println!("{}", "✅ System Info ausgewählt!".green().bold());
            // Hier kannst du weitere Funktionen aufrufen
        },
        "5" => {
            println!("{}", "👋 Auf Wiedersehen!".yellow().bold());
            std::process::exit(0);
        }
        _ => {
            println!("{}", "❌ Ungültige Auswahl! Bitte wähle eine Zahl zwischen 1-5.".red().bold());
        }
    }
}