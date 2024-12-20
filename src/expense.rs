use crate::cents_to_dollars;
use serde::{Deserialize, Serialize};

// Expense struct for shared costs
#[derive(Serialize, Deserialize)]
pub struct Expense {
    pub description: String,
    pub amount: i64,
}

impl Expense {
    pub fn display(&self) {
        println!("{}: ${}", self.description, cents_to_dollars(self.amount));
    }
}
