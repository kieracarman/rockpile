use crate::cents_to_dollars;
use serde::{Deserialize, Serialize};

// Member struct representing each participant
#[derive(Serialize, Deserialize)]
pub struct Member {
    pub name: String,
    pub monthly_income: i64, // monthly income in cents
}

impl Member {
    pub fn display(&self) {
        println!(
            "{} with income ${}",
            self.name,
            cents_to_dollars(self.monthly_income)
        );
    }
}
