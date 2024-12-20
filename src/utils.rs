use std::io::{self, Write};

pub fn cents_to_dollars(number: i64) -> String {
    format!("{}.{:02}", number / 100, number % 100)
}

pub fn get_valid_number(prompt: &str) -> i64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        match input.trim().parse::<f64>() {
            Ok(num) if num >= 0.0 => return (num * 100.0).round() as i64,
            _ => println!("Please enter a valid positive number."),
        }
    }
}

pub fn get_menu_choice(prompt: &str) -> i32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        match input.trim().parse::<i32>() {
            Ok(num) if num > 0 => return num,
            _ => println!("Please enter a valid positive number corresponding to the menu option."),
        }
    }
}
