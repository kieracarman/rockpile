use rockpile::{get_menu_choice, Fund};

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
            4 => fund.process_monthly_cycle(),
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
