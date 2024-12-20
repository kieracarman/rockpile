use crate::{cents_to_dollars, get_valid_number, Expense, Member};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read, Write};

// Fund struct for managing the shared pool
#[derive(Serialize, Deserialize)]
pub struct Fund {
    pub balance: i64, // total money in the pot, in cents
    pub members: Vec<Member>,
    pub expenses: Vec<Expense>,
}

impl Fund {
    pub fn add_member(&mut self) {
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

    pub fn add_expense(&mut self) {
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

    pub fn add_income(&mut self) {
        for member in &self.members {
            self.balance += member.monthly_income;
            println!(
                "{} contributed ${}.",
                member.name,
                cents_to_dollars(member.monthly_income)
            );
        }
    }

    pub fn deduct_expenses(&mut self) {
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

    pub fn redistribute(&mut self) {
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

    pub fn display_total(&self) {
        println!(
            "Total funds in the pot: ${}",
            cents_to_dollars(self.balance)
        );
    }

    pub fn display_state(&self) {
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

    pub fn process_monthly_cycle(&mut self) {
        println!("\n--- Starting Monthly Cycle ---");
        self.add_income();
        self.deduct_expenses();
        self.redistribute();
        self.display_total();
        println!("--- End of Monthly Cycle ---\n");
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<(), std::io::Error> {
        let mut file = File::create(file_path).expect("Failed to create file.");
        let json = serde_json::to_string_pretty(&self).expect("Failed to serialize Fund to JSON.");
        file.write_all(json.as_bytes())
            .expect("Failed to write to file.");
        println!("Fund state saved to {}", file_path);
        Ok(())
    }

    pub fn load_from_file(file_path: &str) -> Result<Fund, std::io::Error> {
        let mut file = File::open(file_path).expect("Failed to open file.");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Failed to read file contents.");
        let fund: Fund = serde_json::from_str(&content).expect("Failed to deserialize JSON.");
        Ok(fund)
    }
}
