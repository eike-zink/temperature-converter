use std::io;

fn main() {
    println!("Hello, world!");
}

fn show_menu() {
    println!();
    println!("=====================");
    println!("Temperature Converter");
    println!("=====================");
    println!("1. Convert °F to °C");
    println!("2. Convert °C to °F");
    println!("Choose an option (1 or 2; press a letter to quit):");
}

fn get_menu_choice() -> i32 {
    let mut choice = String::new();
    let _ = io::stdin().read_line(&mut choice);
    let choice = choice.trim().parse();
    choice.unwrap_or(0)
}
