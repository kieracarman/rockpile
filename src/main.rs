use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

// define a Member struct to represent each participant
#[derive(Serialize, Deserialize)]
struct Member {
    name: String,
    income: i64, // monthly income in cents
}

impl Member {
    fn display(&self) {
        println!(
            "{} with income ${}.{:02}",
            self.name,
            self.income / 100,
            self.income % 100
        );
    }
}

// define a Fund struct to represent the shared pool
#[derive(Default)]
struct Fund {
    total: i64, // total money in the pot, in cents
}

impl Fund {
    fn add_income(&mut self, amount: i64) {
        self.total += amount;
    }

    fn deduct_expenses(&mut self, amount: i64) {
        self.total -= amount;
    }

    fn distribute(&mut self, members: &[Member]) -> i64 {
        if members.is_empty() {
            println!("No members to distribute funds.");
            return 0;
        }
        let total_members = members.len() as i64;
        let share_per_member = self.total / total_members;
        self.total = 0; // reset fund after distribution
        share_per_member
    }

    fn display_total(&self) {
        println!(
            "Total funds in the pot: ${}.{:02}",
            self.total / 100,
            self.total % 100
        );
    }
}

fn save_to_file<T: Serialize>(data: &T, file_name: &str) {
    let json_data = serde_json::to_string(data).expect("Failed to serialize data");
    fs::write(file_name, json_data).expect("Failed to save data to file");
    println!("Data saved successfully!");
}

fn load_from_file<T>(file_name: &str) -> T
where
    T: for<'de> Deserialize<'de> + Default,
{
    let json_data = fs::read_to_string(file_name).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&json_data).unwrap_or_default()
}

fn get_valid_number(prompt: &str) -> i64 {
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

fn add_member(members: &mut Vec<Member>) {
    loop {
        println!("Enter a member's name (or type 'done' to finish):");
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read input");
        let name = name.trim().to_string();

        if name.to_lowercase() == "done" {
            break;
        }

        if name.is_empty() {
            println!("Name cannot be empty.");
            continue;
        }

        if members.iter().any(|m| m.name == name) {
            println!("This name already exists. Please enter a unique name.");
            continue;
        }

        let income = get_valid_number(&format!("Enter {}'s income in dollars: ", name));
        members.push(Member { name, income });
        println!("Member added successfully!");
    }
}

fn main() {
    let mut members: Vec<Member> = load_from_file("members.json");
    let mut fund = Fund::default();

    // check if there are already saved members
    if !members.is_empty() {
        println!("Loaded members:");
        for member in &members {
            member.display();
        }
    } else {
        println!("No saved members found.");
    }

    // add new members interactively
    add_member(&mut members);

    // save members to file
    save_to_file(&members, "members.json");

    // add incomes to the fund
    for member in &members {
        fund.add_income(member.income);
        member.display();
    }

    // collect expense information
    let expenses = get_valid_number("Enter total expenses in dollars: ");
    fund.deduct_expenses(expenses);
    println!(
        "Expenses deducted: ${}.{:02}",
        expenses / 100,
        expenses % 100
    );

    // distribute remaining funds
    let share = fund.distribute(&members);
    if share > 0 {
        println!("Each member gets: ${}.{:02}", share / 100, share % 100);
    }

    // final fund status
    fund.display_total();
}
