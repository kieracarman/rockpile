use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

// define a Member struct to represent each participant
#[derive(Serialize, Deserialize)]
struct Member {
    name: String,
    income: i64, // monthly income in cents
}

// define a Fund struct to represent the shared pool
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
        let total_members = members.len() as i64;
        let share_per_member = self.total / total_members;
        self.total = 0; // reset fund after distribution
        share_per_member
    }
}

fn save_members(members: &Vec<Member>) {
    let json_data = serde_json::to_string(&members).expect("Failed to serialize members");
    fs::write("members.json", json_data).expect("Failed to save members to file");
    println!("Members saved successfully!");
}

fn load_members() -> Vec<Member> {
    let json_data = fs::read_to_string("members.json").unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&json_data).expect("Failed to deserialize members")
}

fn main() {
    let mut members = load_members();
    let mut fund = Fund { total: 0 };

    // check if there are already saved members
    if !members.is_empty() {
        println!("Loaded members:");
        for member in &members {
            println!(
                "{} with income ${}.{:02}",
                member.name,
                member.income / 100,
                member.income % 100
            );
        }
    } else {
        println!("No saved members found.");
    }

    // add new members interactively
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

        println!(
            "Enter {}'s income in dollars (e.g., 1000 for $1000.00):",
            name
        );
        let mut income = String::new();
        io::stdin()
            .read_line(&mut income)
            .expect("Failed to read input");
        let income = income
            .trim()
            .parse::<i64>()
            .expect("Please enter a valid number")
            * 100;

        members.push(Member { name, income });
    }

    // save members to file
    save_members(&members);

    // add incomes to the fund
    for member in &members {
        fund.add_income(member.income);
        println!(
            "{} contributed ${}.{:02}",
            member.name,
            member.income / 100,
            member.income % 100
        );
    }

    // collect expense information
    println!("Enter total expenses in dollars:");
    let mut expenses = String::new();
    io::stdin()
        .read_line(&mut expenses)
        .expect("Failed to read input");
    let expenses = expenses
        .trim()
        .parse::<i64>()
        .expect("Please enter a valid number")
        * 100;

    fund.deduct_expenses(expenses);
    println!(
        "Expenses deducted: ${}.{:02}",
        expenses / 100,
        expenses % 100
    );

    // distribute remaining funds
    let share = fund.distribute(&members);
    println!("Each member gets: ${}.{:02}", share / 100, share % 100);

    // final fund status
    println!(
        "Total funds in the pot: ${}.{:02}",
        fund.total / 100,
        fund.total % 100
    );
}
