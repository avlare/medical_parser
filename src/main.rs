use anyhow::Result;
use medical_parser::*;
use std::env;

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        None => print_help(),
        Some("--help") | Some("-h") => print_help(),
        Some("--credits") | Some("-c") => print_credits(),
        Some(path) => run_parser(path)?,
    }
    Ok(())
}

fn run_parser(path: &str) -> Result<()> {
    let input = std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("Failed to read file '{path}': {e}"))?;

    let patients =
        parse_medical_document(&input).map_err(|e| anyhow::anyhow!("Parsing failed: {e}"))?;

    print_patients(&patients);

    println!("Total patients: {}", count_patients(&patients));
    println!("Total visits: {}", count_total_visits(&patients));

    Ok(())
}

fn print_help() {
    println!("Usage:");
    println!("  medical_parser <PATH>      Parse the provided MedXML file");
    println!("  medical_parser --help      Show this help message");
    println!("  medical_parser --credits   Display author credits");
    println!("\nExample:\n  cargo run -- example.xml");
}

fn print_credits() {
    println!("Author: Anna Rechkalova");
    println!("NaUKMA SE-4");
}
