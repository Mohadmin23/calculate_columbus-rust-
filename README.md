# Columbus Tax & Spending Calculator (Rust)

A **Rust-based interactive command-line tool** to calculate **federal, Ohio state, and Columbus city taxes**, track spending, and save entries in **SQLite**.  

This project helps users see their **take-home income**, plan spending, and log short-term or long-term expenses.

---

## Features

- **Income & Tax Calculator**
  - Supports **annual, monthly, or weekly income** input.
  - Calculates **Federal, Ohio state, and Columbus city taxes**.
  - Displays **total tax**, **take-home income**, and percentages.

- **Spending Calculator**
  - Separate handling for **day/week** and **month/year** spending:
    - **Day/Week**: Total spending with **user note** (what the money was spent on).  
    - **Month/Year**: Detailed spending categories (**rent, groceries, transportation, bills**).  
  - Converts all entries to **annualized spending**.  
  - Calculates **remaining net income** and percentage spent.

- **SQLite Database Integration**
  - Stores spending entries in two tables:
    - `monthly_spending` (month/year category entries)
    - `shortterm_spending` (day/week totals + notes)
  - Users can **view all saved entries** from the CLI.

- **Interactive CLI**
  - Asks the user about **income period**.
  - Asks about **spending period** and relevant inputs.
  - Offers option to **look inside the saved spending database**.

---

## Project Structure

calculate_columbus/
├── Cargo.toml
└── src/
├── main.rs # Program entry & menu
├── tax.rs # Tax calculation module
├── spending.rs # Spending calculator module
└── db.rs # SQLite database helpers

---

## Dependencies

- [`rusqlite`](https://crates.io/crates/rusqlite) – SQLite integration
- [`chrono`](https://crates.io/crates/chrono) – Date/time handling

Add to your `Cargo.toml`:

```toml
[dependencies]
rusqlite = "0.29"
chrono = "0.4"

? How to Run

Clone the repository: 
git clone https://github.com/yourusername/calculate_columbus.git
cd calculate_columbus

Build and run the project:
cargo run

Follow the prompts:

Enter income amount and period (year/month/week).

Choose spending period (day/week/month/year).

Enter spending details or total with note.

Optionally view saved spending database entries.

Is your income per year, per month, or per week? month
Enter your income amount: 2000

--- Tax Summary ---
Annual income before tax: $24000.00
Federal tax: $2500.00 (10.42%)
Ohio state tax: $660.00 (2.75%)
Columbus city tax: $600.00 (2.50%)
Total tax: $3760.00 (15.67%)
Take-home income: $20240.00 (84.33%)

Do you want to calculate spending per day, week, month, or year? month
Enter your monthly/yearly rent: 800
Enter your monthly/yearly groceries: 300
Enter your monthly/yearly transportation: 100
Enter your monthly/yearly bills (utilities, internet, etc.): 150

Monthly/yearly spending saved.

Annual net income: $20240.00
Annual spending: $15600.00
Remaining after spending: $4640.00
Percentage of income spent: 77.09%

Do you want to look inside the saved spending database? yes
--- Monthly/Yearly Spending ---
2025-11-28 [month] Rent: $800.0, Groceries: $300.0, Transport: $100.0, Bills: $150.0, Total: $1350.0, Annualized: $16200.0

--- Day/Week Spending ---

Monthly/Yearly Spending Table:
| Column        | Type | Notes                 |
| ------------- | ---- | --------------------- |
| id            | INT  | PK, auto increment    |
| date          | TEXT | Date of entry         |
| period        | TEXT | month/year            |
| rent          | REAL | Rent amount           |
| groceries     | REAL | Groceries amount      |
| transport     | REAL | Transportation amount |
| bills         | REAL | Bills amount          |
| total         | REAL | Total of categories   |
| annual_amount | REAL | Converted to annual   |

Day/Week Spending Table:

| Column        | Type | Notes               |
| ------------- | ---- | ------------------- |
| id            | INT  | PK, auto increment  |
| date          | TEXT | Date of entry       |
| period        | TEXT | day/week            |
| amount        | REAL | Total spending      |
| note          | TEXT | User note           |
| annual_amount | REAL | Converted to annual |


Future Improvements:
Add CLI menu with options:

Calculate taxes

Calculate spending

View saved spending

Exit

Support multiple users.

Generate monthly/annual reports with graphs.

Export database to CSV or JSON.

=============================================================
License

This project is open-source under the MIT License.
