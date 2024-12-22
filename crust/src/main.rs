use std::env;
use std::fs;

// enum State {
//     Help,
//     Version,
//     Run(String),
// }

fn print_help() {
    println!("crust, a cat clone written in Rust");
    println!();
    println!("Usage:");
    println!();
    println!("  crust [file-path]   print the file's contents");
    println!("  crust --help        print this help message");
    println!("  crust --version     print the version number");
}

fn print_version() {
    println!("v0.1.0");
}

fn run(file_path: &str) {
    let contents_result = fs::read_to_string(file_path);
    match contents_result {
        Ok(contents) => println!("{}", contents),
        Err(error) => println!("File not found or could not be read: {}", error),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => match args[0].as_str() {
            "--help" => print_help(),
            "--version" => print_version(),
            _ => run(&args[1]),
        },
        _ => print_help(),
    }
}
