use clap::{Arg, ArgAction, Command};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize)]
enum Transaction {
    Income {
        amount: i32,
        description: Option<String>,
    },
    Expense {
        amount: i32,
        description: Option<String>,
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Account {
    balance: i32,
    transactions: Vec<Transaction>,
}

impl Account {
    fn add_income(&mut self, amount: i32, description: Option<String>) {
        self.balance += amount;
        self.transactions.push(Transaction::Income { amount, description: description.clone()});
        println!("{} has been credited to your account.", amount);
        if let Some(desc) = description.as_ref() {
            println!("Description: {}", desc);
        }
    }

    fn add_expense(&mut self, amount: i32, description: Option<String>) {
        self.balance -= amount;
        self.transactions.push(Transaction::Expense { amount, description: description.clone() });
        println!("{} has been debited from your account.", amount);
        if let Some(desc) = description.as_ref() {
            println!("Description: {}", desc);
        }
    }

    fn show_balance(&self) {
        println!("You current balance is {}", self.balance);
    }

    fn save_to_file(&self, file_path: &str) {
        let json = serde_json::to_string_pretty(self).expect("Failed to serialize account data");
        let mut file = File::create(file_path).expect("Failed to create file");
        file.write_all(json.as_bytes()).expect("Failed to write data to file");
    }

    fn load_from_file(file_path: &str) -> Self {
        if let Ok(data) = fs::read_to_string(file_path) {
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    fn total_income(&self) -> i32 {
        self.transactions.iter().fold(0, |sum, transaction| {
            if let Transaction::Income { amount, .. } = transaction {
                sum + amount
            } else {
                sum
            }
        })
    }

    fn total_expenses(&self) -> i32 {
        self.transactions.iter().fold(0, |sum, transaction| {
            if let Transaction::Expense { amount, .. } = transaction {
                sum + amount
            } else {
                sum
            }
    })
}
}

fn call_alfred() {
    let matches = Command::new("Alfred at your service")
        .version("0.1.0")
        .about("Tracing you finances")
        .arg(Arg::new("income")
                .short('i')
                .long("income")
                .value_name("Income Amount")
                .action(ArgAction::Set)
                .help("Add you income by specifying -i or --income"))
        .arg(Arg::new("description")
                .short('d')
                .long("description")
                .value_name("Description")
                .action(ArgAction::Set)
                .help("Add a description by specifying -d or --description"))
        .arg(Arg::new("expense")
                .short('e')
                .long("expense")
                .value_name("Expense Amount")
                .action(ArgAction::Set)
                .help("Add your expense by specifying -e or --expense"))
        .arg(Arg::new("balance")
                .short('b')
                .long("balance")
                .action(ArgAction::SetTrue)
                .help("View you balance by specifying -b or --balance"))
        .arg(Arg::new("Total_Expense")
                .short('E')
                .long("totalexpense")
                .action(ArgAction::SetTrue)
                .help("View your total expenditure by specifying -E or --totalexpenditure"))
        .arg(Arg::new("Total_Income")
                .short('I')
                .long("totalincome")
                .action(ArgAction::SetTrue)
                .help("View your total income by specifying -E or --totalincome"))
        .get_matches();

    let file_path = "account_data.json";
    let mut account = Account::load_from_file(file_path);

    if let Some(income) = matches.get_one::<String>("income") {
        let description = matches.get_one::<String>("description").map(|s| s.clone());
        match income.parse::<i32>() {
            Ok(amount) => account.add_income(amount, description),
            Err(_) => println!("Invalid input for income {}", income),
        }
    }

    if let Some(expense) = matches.get_one::<String>("expense") {
        let description = matches.get_one::<String>("description").map(|s| s.clone());
        match expense.parse::<i32>() {
            Ok(amount) => account.add_expense(amount, description),
            Err(_) => println!("Invalid input for income {}", expense),
        }
    }

    if matches.get_flag("balance") {
        account.show_balance();
    }

    if matches.get_flag("Total_Expense") {
        println!("Total expnses: {}", account.total_expenses());
    }

    if matches.get_flag("Total_Income") {
        println!("Total Income: {}", account.total_income());
    }

    account.save_to_file(file_path);
}


fn main() {
    call_alfred();
}
