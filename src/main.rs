use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

// define a Member struct to represent each participant
#[derive(Serialize, Deserialize)]
struct Member {
    name: String,
    monthly_income: i64, // monthly income in cents
}

impl Member {
    fn display(&self) {
        println!(
            "{} with income ${}.{:02}",
            self.name,
            self.monthly_income / 100,
            self.monthly_income % 100
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
    fn add_income(&mut self, amount: i64) {
        self.balance += amount;
    }

    fn deduct_expenses(&mut self, amount: i64) {
        self.balance -= amount;
    }

    fn distribute(&mut self, members: &[Member]) -> i64 {
        if members.is_empty() {
            println!("No members to distribute funds.");
            return 0;
        }
        let total_members = members.len() as i64;
        let share_per_member = self.balance / total_members;
        self.balance = 0; // reset fund after distribution
        share_per_member
    }

    fn display_total(&self) {
        println!(
            "Total funds in the pot: ${}.{:02}",
            self.balance / 100,
            self.balance % 100
        );
    }

    fn monthly_cycle(&mut self) {
        println!("--- Starting Monthly Cycle ---");

        // 1. add incomes to the fund balance
        for member in &self.members {
            self.balance += member.monthly_income;
            println!(
                "{} contributed ${}.{:02} to the fund.",
                member.name,
                member.monthly_income / 100,
                member.monthly_income % 100
            );
        }

        // 2. deduct expenses from the fund
        for expense in &self.expenses {
            if self.balance >= expense.amount {
                self.balance -= expense.amount;
                println!(
                    "Paid ${}.{:02} for {}.",
                    expense.amount / 100,
                    expense.amount % 100,
                    expense.description
                );
            } else {
                println!(
                    "Insufficient funds to pay ${}.{:02} for {}.",
                    expense.amount / 100,
                    expense.amount % 100,
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
                    "{} received a distribution of ${}.{:02}.",
                    member.name,
                    leftover_per_member / 100,
                    leftover_per_member % 100
                );
            }
        }

        // final balance
        self.display_total();
        println!("--- End of Monthly Cycle ---\n");
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

    println!("Initial Fund State:");
    println!(
        "Balance: ${}.{:02}.",
        fund.balance / 100,
        fund.balance % 100
    );
    println!("Expenses:");
    for expense in &fund.expenses {
        println!(
            "{}: ${}.{:02}",
            expense.description,
            expense.amount / 100,
            expense.amount % 100
        );
    }

    // simulate 3 monthly cycles
    for month in 1..=3 {
        println!("Month {}", month);
        fund.monthly_cycle();
    }
}
