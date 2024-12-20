# RockPile

RockPile is a cooperative microeconomy system designed to pool all income into a shared fund, which is then used to pay for utilities, insurance, and other shared expenses. At the end of the month, any remaining funds are redistributed equally among the members of the cooperative. The system emphasizes transparency, community, and shared benefit.

This project provides a simple implementation of the RockPile fund management system in Rust, using JSON for data persistence. It includes the ability to manage members, expenses, income contributions, and monthly cycles, making it a practical tool for small cooperatives or other collective groups.

## Features

- **Member Management**: Add new members to the cooperative and track their monthly income.
- **Expense Management**: Add shared expenses and track their payments from the pooled fund.
- **Income Contribution**: Automatically add each member's income to the fund each month.
- **Expense Deduction**: Deduct expenses from the fund, with checks for sufficient balance.
- **Fund Redistribution**: Redistribute any remaining funds equally among the members at the end of the month.
- **Data Persistence**: Save and load the fund state from a JSON file for easy recovery.
- **Monthly Cycle Automation**: Run the monthly cycle to add income, pay expenses, and redistribute funds.

## Requirements

- Rust 1.60.0 or later
- `serde` and `serde_json` crates for data serialization
- Basic knowledge of Rust for further customization

## Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/kieracarman/rockpile.git
    cd rockpile
    ```

2. Build and run the project:

    ```bash
    cargo run
    ```

3. If you don't have Rust installed, follow the installation instructions from [the official Rust website](https://www.rust-lang.org/tools/install).

## Project Structure

- `src/`: Contains all source code files for the application.
    - `main.rs`: The main entry point of the application, where the menu and user input are managed.
    - `fund.rs`: Contains the `Fund` struct and associated methods for managing members, expenses, income, and monthly cycles.
    - `member.rs`: Contains the `Member` struct and methods for displaying and managing members.
    - `expense.rs`: Contains the `Expense` struct and methods for displaying and managing expenses.
    - `utils.rs`: Contains utility functions like `get_valid_number`, `cents_to_dollars`, and `get_menu_choice`.
    - `lib.rs`: Serves as the module entry point for organizing the project structure.

- `Cargo.toml`: The manifest file for managing dependencies and project configuration.

## Usage

Once the project is running, you'll be presented with a menu where you can:

1. **View Fund State**: Display the current fund balance, members, and expenses.
2. **Add a Member**: Add a new member to the cooperative by entering their name and monthly income.
3. **Add an Expense**: Add a shared expense with a description and amount.
4. **Run Monthly Cycle**: Automatically add income, deduct expenses, and redistribute remaining funds to members.
5. **Save Fund to File**: Save the current state of the fund to a `fund.json` file.
6. **Load Fund from File**: Load a saved fund state from the `fund.json` file.
7. **Exit**: Exit the program.

### Example Flow

1. Start by adding members to the cooperative:
    ```
    Enter a member's name: Alice
    Enter Alice's income in dollars: 1000
    Member added successfully!
    ```

2. Add some expenses:
    ```
    Enter expense description: Rent
    Enter expense amount in dollars: 500
    Expense added successfully!
    ```

3. Run the monthly cycle:
    ```
    --- Starting Monthly Cycle ---
    Alice contributed $1000.00.
    Paid $500.00 for Rent.
    Remaining funds: $500.00
    Alice received a distribution of $500.00.
    --- End of Monthly Cycle ---
    ```

4. Save or load the fund state using `fund.json` for persistence between runs.

## Functions and Methods

### Fund

The `Fund` struct manages the fund, members, and expenses.

- **add_member()**: Adds a new member to the cooperative.
- **add_expense()**: Adds a shared expense.
- **add_income()**: Adds income contributions from all members.
- **deduct_expenses()**: Deducts expenses from the fund.
- **redistribute()**: Redistributes remaining funds equally among members.
- **display_total()**: Displays the total balance in the fund.
- **display_state()**: Displays the current state of the fund, including members and expenses.
- **monthly_cycle()**: Runs a full monthly cycle: add income, deduct expenses, redistribute funds.
- **save_to_file(file_path)**: Saves the current fund state to a file.
- **load_from_file(file_path)**: Loads the fund state from a file.

### Utility Functions

- **cents_to_dollars(i64)**: Converts a value in cents to a string representing dollars and cents.
- **get_valid_number(prompt)**: Prompts the user for a valid positive number.
- **get_menu_choice(prompt)**: Prompts the user for a valid menu choice.

## How It Works

1. The cooperative members input their monthly income, which is stored in the `Fund`.
2. Shared expenses are added to the `Fund` and paid from the pool.
3. At the end of each month, the fund's balance is redistributed equally among all members.
4. The state is saved to a file, allowing for persistence between sessions.

## Contact

For questions or suggestions, please feel free to contact me via GitHub or open an issue in the repository.

