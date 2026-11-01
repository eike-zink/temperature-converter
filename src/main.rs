use std::io;

const ABSOLUTE_ZERO_F: f64 = -459.67;
const ABSOLUTE_ZERO_C: f64 = -273.15;

fn main() {
    // Loop until the user quits
    loop {
        // Show main menu and get the user's choice
        show_menu();
        match get_menu_choice() {
            // None means the user entered no number
            // (a letter or EOF), so quit
            None => {
                println!("Thanks for using Temperature Converter!");
                break;
            }
            Some(1) => run_conversion("F", "C", fahrenheit_to_celsius, is_valid_fahrenheit),
            Some(2) => run_conversion("C", "F", celsius_to_fahrenheit, is_valid_celsius),
            Some(_) => println!("Invalid choice! Please try again."),
        }
    }
}

fn run_conversion(
    from_unit: &str,
    to_unit: &str,
    convert: fn(f64) -> f64,
    is_valid: fn(f64) -> bool,
) {
    println!("Type the °{from_unit} temperature you want to convert:");
    match get_temp() {
        Some(temp) if is_valid(temp) => {
            print_converted_temp(temp, from_unit, convert(temp), to_unit);
        }
        Some(_) => {
            println!();
            println!("Not a valid temperature! Please try again.");
        }
        None => {
            println!();
            println!("Not a valid number! Please try again.");
        }
    }
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

/// Reads a line and returns its content, or None on a read error.
fn read_line() -> Option<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    Some(input)
}

/// Returns the entered number, or None if the input is not a number.
fn get_menu_choice() -> Option<i32> {
    read_line().and_then(|line| line.trim().parse().ok())
}

/// Returns the entered temperature, or None if the input is not a number.
fn get_temp() -> Option<f64> {
    read_line().and_then(|line| line.trim().parse().ok())
}

fn is_valid_fahrenheit(fahrenheit: f64) -> bool {
    fahrenheit >= ABSOLUTE_ZERO_F
}

fn is_valid_celsius(celsius: f64) -> bool {
    celsius >= ABSOLUTE_ZERO_C
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn print_converted_temp(
    original_temp: f64,
    original_unit: &str,
    converted_temp: f64,
    converted_unit: &str,
) {
    println!();
    println!(
        "{:.2}°{} = {:.2}°{}",
        original_temp, original_unit, converted_temp, converted_unit
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOLERANCE: f64 = 1e-9;

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < TOLERANCE,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn converts_freezing_point() {
        assert_close(fahrenheit_to_celsius(32.0), 0.0);
        assert_close(celsius_to_fahrenheit(0.0), 32.0);
    }

    #[test]
    fn converts_boiling_point() {
        assert_close(fahrenheit_to_celsius(212.0), 100.0);
        assert_close(celsius_to_fahrenheit(100.0), 212.0);
    }

    #[test]
    fn converts_body_temperature() {
        assert_close(fahrenheit_to_celsius(98.6), 37.0);
        assert_close(celsius_to_fahrenheit(37.0), 98.6);
    }

    #[test]
    fn absolute_zero_is_the_lower_limit_in_both_scales() {
        assert!(is_valid_fahrenheit(ABSOLUTE_ZERO_F));
        assert!(is_valid_celsius(ABSOLUTE_ZERO_C));
        assert!(!is_valid_fahrenheit(ABSOLUTE_ZERO_F - 1.0));
        assert!(!is_valid_celsius(ABSOLUTE_ZERO_C - 1.0));
    }
}
