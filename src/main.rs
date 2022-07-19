mod roman_ext;
use std::{env, process};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_help() {
    println!("romanrs, version {}", VERSION);
    println!("======================");
    println!("help\t\tsee this message");
    println!("<uint>\t\tconvert unsigned integer in range [1, {}] to roman numerals", roman_ext::MAX);
}

fn convert(argument: usize) {
    match roman_ext::from(argument) {
        Some(x) => println!("{}", x),
        None => {
            eprintln!("error: argument outside of range [1, {}].", roman_ext::MAX);
            process::exit(1);
        }
    }
}

fn parse_command(command: &String) {
    match command.parse() {
        Ok(argument) => convert(argument),
        _ => {
            match command.to_lowercase().as_str() {
                "help" => print_help(),
                _ => {
                    eprintln!("error: could not parse argument.");
                    process::exit(1);
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            eprintln!("error: no arguments passed.");
            process::exit(1);
        }
        2 => parse_command(&args[1]),
        _ => {
            eprintln!("error: too many arguments passed.");
            process::exit(1);
        }
    }
}
