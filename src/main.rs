use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read, Write};

// Member struct representing each participant
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

// Expense struct for shared costs
#[derive(Serialize, Deserialize)]
struct Expense {
    description: String,
    amount: i64,
}

impl Expense {
    fn display(&self) {
        println!("{}: ${}", self.description, cents_to_dollars(self.amount));
    }
}

// Fund struct for managing the shared pool
#[derive(Serialize, Deserialize)]
struct Fund {
    balance: i64, // total money in the pot, in cents
    members: Vec<Member>,
    expenses: Vec<Expense>,
}

impl Fund {
    fn add_member(&mut self) {
        println!("Enter a member's name:");
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read input");
        let name = name.trim().to_string();

        if name.is_empty() {
            println!("Name cannot be empty.");
            return;
        }

        if self.members.iter().any(|m| m.name == name) {
            println!("This name already exists. Please enter a unique name.");
            return;
        }

        let monthly_income = get_valid_number(&format!("Enter {}'s income in dollars: ", name));
        self.members.push(Member {
            name,
            monthly_income,
        });
        println!("Member added successfully!");
    }

    fn add_expense(&mut self) {
        println!("Enter expense description:");
        let mut description = String::new();
        io::stdin()
            .read_line(&mut description)
            .expect("Failed to read input");
        let description = description.trim().to_string();

        if description.is_empty() {
            println!("Description cannot be empty.");
            return;
        }

        let amount = get_valid_number("Enter expense amount in dollars: ");
        self.expenses.push(Expense {
            description,
            amount,
        });
        println!("Expense added successfully!");
    }

    fn add_income(&mut self) {
        for member in &self.members {
            self.balance += member.monthly_income;
            println!(
                "{} contributed ${}.",
                member.name,
                cents_to_dollars(member.monthly_income)
            );
        }
    }

    fn deduct_expenses(&mut self) {
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
    }

    fn redistribute(&mut self) {
        if self.members.is_empty() {
            println!("No members to redistribute funds.");
            return;
        }

        let leftover_per_member = self.balance / self.members.len() as i64;
        self.balance %= self.members.len() as i64;

        for member in &self.members {
            println!(
                "{} received a distribution of ${}.",
                member.name,
                cents_to_dollars(leftover_per_member)
            );
        }
    }

    fn display_total(&self) {
        println!(
            "Total funds in the pot: ${}",
            cents_to_dollars(self.balance)
        );
    }

    fn display_state(&self) {
        println!("\n--- Fund State ---");
        self.display_total();
        println!("\nMembers:");
        for member in &self.members {
            member.display();
        }
        println!("\nExpenses:");
        for expense in &self.expenses {
            expense.display();
        }
    }

    fn monthly_cycle(&mut self) {
        println!("\n--- Starting Monthly Cycle ---");
        self.add_income();
        self.deduct_expenses();
        self.redistribute();
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

fn get_menu_choice(prompt: &str) -> i32 {
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

fn main() {
    let mut fund = Fund {
        balance: 0,
        members: Vec::new(),
        expenses: Vec::new(),
    };

    let file_path = "fund.json";

    loop {
        println!("\n--- RockPile Fund Management ---");
        println!("1. View Fund State");
        println!("2. Add a Member");
        println!("3. Add an Expense");
        println!("4. Run Monthly Cycle");
        println!("5. Save Fund to File");
        println!("6. Load Fund from File");
        println!("7. Exit");

        let choice = get_menu_choice("Enter your choice: ");

        match choice {
            1 => fund.display_state(),
            2 => fund.add_member(),
            3 => fund.add_expense(),
            4 => fund.monthly_cycle(),
            5 => {
                if fund.save_to_file(file_path).is_ok() {
                    println!("Fund saved to file successfully!");
                } else {
                    println!("Failed to save the fund to file.");
                }
            }
            6 => {
                if let Ok(loaded_fund) = Fund::load_from_file(file_path) {
                    fund = loaded_fund;
                    println!("Fund loaded successfully!");
                } else {
                    println!("Failed to load the fund from file.");
                }
            }
            7 => {
                println!("Exiting the program. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
