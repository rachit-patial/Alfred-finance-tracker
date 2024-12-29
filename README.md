# Alfred: Personal Expense Tracker

Alfred is a command-line utility designed to help you manage your personal finances efficiently. With Alfred, you can track your income, expenses, and view your current balance, all while persisting your data in JSON format for easy access and portability.

## Features

- **Add Income**: Record your income with an optional description.
- **Add Expenses**: Track expenses along with a description to categorize spending.
- **View Balance**: Check your current balance at any time.
- **Persistent Storage**: All data is saved in a JSON file using the `serde` crate, ensuring your financial records are always up-to-date.
- **Simple CLI Interface**: Easy-to-use command-line interface for quick access to all features.

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/alfred-expense-tracker.git
   cd alfred-expense-tracker
   ```

2. Build the project using Cargo:
   ```bash
   cargo build --release
   ```

3. Run the binary:
   ```bash
   ./target/release/alfred
   ```

## Usage

Alfred provides a simple CLI to interact with your financial data. Below are the available commands and options:

### Adding Income
```bash
alfred --income <amount> --description "<description>"
```
- `<amount>`: The amount to be credited.
- `--description`: (Optional) A brief description of the income source.

**Example:**
```bash
alfred --income 5000 --description "Salary for December"
```

### Adding Expenses
```bash
alfred --expense <amount> --description "<description>"
```
- `<amount>`: The amount to be debited.
- `--description`: (Optional) A brief description of the expense.

**Example:**
```bash
alfred --expense 1200 --description "Grocery shopping"
```

### Viewing Balance
```bash
alfred --balance
```
Displays the current balance based on recorded income and expenses.

### Help
```bash
alfred --help
```
Displays a list of all available commands and options.

## How It Works

1. **Data Storage**: Alfred uses a JSON file (default: `finance_data.json`) to store all income, expenses, and balance data. This ensures that your financial records persist across sessions.
2. **Rust Libraries**: The project leverages the following crates:
   - `serde` and `serde_json` for serializing and deserializing JSON data.
   - `clap` for command-line argument parsing.

## Example Workflow

1. Add an income entry:
   ```bash
   alfred --income 2000 --description "Freelance project payment"
   ```

2. Add an expense entry:
   ```bash
   alfred --expense 500 --description "Monthly subscription"
   ```

3. View your balance:
   ```bash
   alfred --balance
   ```

## Acknowledgments

- Built with ❤️ using Rust.
- Special thanks to the `serde` and `clap` communities for their amazing libraries.

---

Stay financially smart with Alfred!

