use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read, Write};

// define a Member struct to represent each participant
#[derive(Serialize, Deserialize)]
struct Member {
    name: String,
    monthly_income: i64, // monthly income in cents
}

impl Member {
    fn display(&self) {
        println!(
            "{} with income ${}",
            self.name,
            cents_to_dollars(self.monthly_income)
        );
    }
}

#[derive(Serialize, Deserialize)]
struct Expense {
    description: String,
    amount: i64,
}

// define a Fund struct to represent the shared pool
#[derive(Serialize, Deserialize)]
struct Fund {
    balance: i64, // total money in the pot, in cents
    members: Vec<Member>,
    expenses: Vec<Expense>,
}

impl Fund {
    fn display_total(&self) {
        println!(
            "Total funds in the pot: ${}",
            cents_to_dollars(self.balance)
        );
    }

    fn monthly_cycle(&mut self) {
        println!("--- Starting Monthly Cycle ---");

        // 1. add incomes to the fund balance
        for member in &self.members {
            self.balance += member.monthly_income;
            println!(
                "{} contributed ${} to the fund.",
                member.name,
                cents_to_dollars(member.monthly_income)
            );
        }

        // 2. deduct expenses from the fund
        for expense in &self.expenses {
            if self.balance >= expense.amount {
                self.balance -= expense.amount;
                println!(
                    "Paid ${} for {}.",
                    cents_to_dollars(expense.amount),
                    expense.description
                );
            } else {
                println!(
                    "Insufficient funds to pay ${} for {}.",
                    cents_to_dollars(expense.amount),
                    expense.description
                );
            }
        }

        // 3. redistribute leftover balance to members
        if !self.members.is_empty() {
            let leftover_per_member = self.balance / self.members.len() as i64;
            self.balance %= self.members.len() as i64; // retain the rounding leftovers in the fund
            for member in &mut self.members {
                println!(
                    "{} received a distribution of ${}.",
                    member.name,
                    cents_to_dollars(leftover_per_member)
                );
            }
        }

        // final balance
        self.display_total();
        println!("--- End of Monthly Cycle ---\n");
    }

    fn save_to_file(&self, file_path: &str) -> Result<(), std::io::Error> {
        let mut file = File::create(file_path).expect("Failed to create file.");
        let json = serde_json::to_string_pretty(&self).expect("Failed to serialize Fund to JSON.");
        file.write_all(json.as_bytes())
            .expect("Failed to write to file.");
        println!("Fund state saved to {}", file_path);
        Ok(())
    }

    fn load_from_file(file_path: &str) -> Result<Fund, std::io::Error> {
        let mut file = File::open(file_path).expect("Failed to open file.");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Failed to read file contents.");
        let fund: Fund = serde_json::from_str(&content).expect("Failed to deserialize JSON.");
        Ok(fund)
    }
}

fn cents_to_dollars(number: i64) -> String {
    format!("{}.{:02}", number / 100, number % 100)
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

        let monthly_income = get_valid_number(&format!("Enter {}'s income in dollars: ", name));
        members.push(Member {
            name,
            monthly_income,
        });
        println!("Member added successfully!");
    }
}

fn main() {
    let mut fund = Fund {
        balance: 0,
        members: vec![
            Member {
                name: "Alice".to_string(),
                monthly_income: 100000,
            },
            Member {
                name: "Bob".to_string(),
                monthly_income: 75000,
            },
            Member {
                name: "Charlie".to_string(),
                monthly_income: 50000,
            },
        ],
        expenses: vec![
            Expense {
                description: "Rent".to_string(),
                amount: 150000,
            },
            Expense {
                description: "Electricity".to_string(),
                amount: 30000,
            },
            Expense {
                description: "Internet".to_string(),
                amount: 10000,
            },
        ],
    };

    let file_path = "fund.json";

    println!("Initial Fund State:");
    println!("Balance: ${}.", cents_to_dollars(fund.balance));
    println!("Expenses:");
    for expense in &fund.expenses {
        println!(
            "{}: ${}",
            expense.description,
            cents_to_dollars(expense.amount)
        );
    }

    // simulate 3 monthly cycles
    for month in 1..=3 {
        println!("Month {}", month);
        fund.monthly_cycle();
    }

    fund.save_to_file(file_path);
    println!("Fund saved to {}", file_path);

    let loaded_fund = Fund::load_from_file(file_path).unwrap();
    println!(
        "Loaded fund balance: ${}",
        cents_to_dollars(loaded_fund.balance)
    );
}
