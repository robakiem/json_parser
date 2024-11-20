use std::env;
use json_parser::JsonDocument;

fn print_help() {
    println!("JSON Parser using Pest!");
    println!("Usage: cargo run <file_path>");
    println!("Options:");
    println!("  --help    Show this help message");
    println!("  --credits Show project credits");
}

fn print_credits() {
    println!("JSON simple parser");
    println!("Created by: Bahriantsev Ivan");
    println!("Version: 0.1.0");
    println!("License: MIT");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // If no arguments are passed, print the help message.
    if args.len() == 1 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "--help" => {
            print_help();
        }
        "--credits" => {
            print_credits();
        }
        _ => {
            // Handle file path
            let file_path = &args[1];

            // Read the file content
            let input = match std::fs::read_to_string(file_path) {
                Ok(content) => content.trim().to_string(),
                Err(e) => {
                    eprintln!("Error reading file {}: {}", file_path, e);
                    return;
                }
            };

            // Parse the JSON and print results
            match JsonDocument::parse(&input) {
                Ok(doc) => {
                    println!("✅ JSON is valid!");
                    println!("File: {}", file_path);
                    println!("Root Type: {:?}", doc.root_type);
                    println!("{}", doc.content);
                }
                Err(e) => {
                    println!("❌ JSON is invalid!");
                    println!("File: {}", file_path);
                    println!("Error: {}", e);
                }
            }
        }
    }
}