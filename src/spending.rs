use crate::db;
use std::io;

pub fn calculate_spending(net_income: f64) {
    println!(
        "Do you want to calculate spending per day, week, month, or year? (day/week/month/year)"
    );
    let mut period = String::new();
    io::stdin()
        .read_line(&mut period)
        .expect("Failed to read input");
    let period = period.trim().to_lowercase();

    let annual_spending: f64;

    match period.as_str() {
        "month" | "year" => {
            let rent = read_f64("Enter your monthly/yearly rent: ");
            let groceries = read_f64("Enter your monthly/yearly groceries: ");
            let transport = read_f64("Enter your monthly/yearly transportation: ");
            let bills = read_f64("Enter your monthly/yearly bills (utilities, internet, etc.): ");
            let total = rent + groceries + transport + bills;
            annual_spending = if period == "month" {
                total * 12.0
            } else {
                total
            };

            if let Err(e) =
                db::save_monthly_entry(&period, rent, groceries, transport, bills, annual_spending)
            {
                eprintln!("Failed to save monthly entry: {}", e);
            } else {
                println!("Monthly/yearly spending saved.");
            }
        }

        "day" | "week" => {
            let spending = read_f64("Enter your spending amount for the selected period: ");
            println!("Please type a note about what you did with the money: ");
            let mut note = String::new();
            io::stdin()
                .read_line(&mut note)
                .expect("Failed to read input");
            let note_trimmed = note.trim();

            annual_spending = if period == "day" {
                spending * 365.0
            } else {
                spending * 52.0
            };

            if let Err(e) =
                db::save_shortterm_entry(&period, spending, note_trimmed, annual_spending)
            {
                eprintln!("Failed to save shortterm entry: {}", e);
            } else {
                println!("Short-term spending saved (with note).");
            }
        }

        _ => {
            println!("Invalid period, assuming yearly spending.");
            annual_spending = 0.0;
        }
    }

    let remaining = net_income - annual_spending;
    println!("\n--- Spending Summary ---");
    println!("Annual net income: ${:.2}", net_income);
    println!("Annual spending: ${:.2}", annual_spending);
    println!("Remaining after spending: ${:.2}", remaining);
    if net_income > 0.0 {
        println!(
            "Percentage of income spent: {:.2}%",
            annual_spending / net_income * 100.0
        );
    }

    // --- Ask if user wants to see the database ---
    println!("\nDo you want to look inside the saved spending database? (yes/no)");
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read input");
    let answer = answer.trim().to_lowercase();
    if answer == "yes" {
        show_spending_db();
    }
}

fn read_f64(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read input");
    s.trim().parse().unwrap_or(0.0)
}

/// Query DB and print saved spending
fn show_spending_db() {
    let conn = match rusqlite::Connection::open("spending.db") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to open DB: {}", e);
            return;
        }
    };

    println!("\n--- Monthly/Yearly Spending ---");
    let mut stmt = conn.prepare("SELECT date, period, rent, groceries, transport, bills, total, annual_amount FROM monthly_spending").unwrap();
    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?, // date
                row.get::<_, String>(1)?, // period
                row.get::<_, f64>(2)?,    // rent
                row.get::<_, f64>(3)?,    // groceries
                row.get::<_, f64>(4)?,    // transport
                row.get::<_, f64>(5)?,    // bills
                row.get::<_, f64>(6)?,    // total
                row.get::<_, f64>(7)?,    // annual_amount
            ))
        })
        .unwrap();

    for r in rows {
        if let Ok((date, period, rent, groceries, transport, bills, total, annual)) = r {
            println!(
                "{} [{}] Rent: ${}, Groceries: ${}, Transport: ${}, Bills: ${}, Total: ${}, Annualized: ${}",
                date, period, rent, groceries, transport, bills, total, annual
            );
        }
    }

    println!("\n--- Day/Week Spending ---");
    let mut stmt2 = conn
        .prepare("SELECT date, period, amount, note, annual_amount FROM shortterm_spending")
        .unwrap();
    let rows2 = stmt2
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?, // date
                row.get::<_, String>(1)?, // period
                row.get::<_, f64>(2)?,    // amount
                row.get::<_, String>(3)?, // note
                row.get::<_, f64>(4)?,    // annual_amount
            ))
        })
        .unwrap();

    for r in rows2 {
        if let Ok((date, period, amount, note, annual)) = r {
            println!(
                "{} [{}] Amount: ${}, Note: {}, Annualized: ${}",
                date, period, amount, note, annual
            );
        }
    }
}
