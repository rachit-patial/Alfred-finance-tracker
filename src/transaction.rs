use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};

#[derive(Debug, Serialize, Deserialize)]
pub enum Transaction {
    Income {
        amount: i32,
        description: Option<String>,
        timestamp: DateTime<Local>,
    },
    Expense {
        amount: i32,
        description: Option<String>,
        timestamp: DateTime<Local>,
    }
}