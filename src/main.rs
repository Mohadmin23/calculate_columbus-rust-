mod db;
mod spending;
mod tax;

use crate::tax::total_tax;
use std::io;

fn main() {
    // Ask for income period
    println!("Is your income per year, per month, or per week? (Enter: year/month/week)");
    let mut period = String::new();
    io::stdin()
        .read_line(&mut period)
        .expect("Failed to read input");
    let period = period.trim().to_lowercase();

    println!("Enter your income amount: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let mut income: f64 = input.trim().parse().expect("Please enter a valid number");

    // Convert to annual income
    match period.as_str() {
        "year" => {}
        "month" => income *= 12.0,
        "week" => income *= 52.0,
        _ => println!("Invalid input, assuming annual income."),
    }

    let (federal, state, city, total_tax, net_income) = total_tax(income);

    println!("\n--- Tax Summary ---");
    println!("Annual income before tax: ${:.2}", income);
    println!(
        "Federal tax: ${:.2} ({:.2}%)",
        federal,
        federal / income * 100.0
    );
    println!(
        "Ohio state tax: ${:.2} ({:.2}%)",
        state,
        state / income * 100.0
    );
    println!(
        "Columbus city tax: ${:.2} ({:.2}%)",
        city,
        city / income * 100.0
    );
    println!(
        "Total tax: ${:.2} ({:.2}%)",
        total_tax,
        total_tax / income * 100.0
    );
    println!(
        "Take-home income: ${:.2} ({:.2}%)",
        net_income,
        net_income / income * 100.0
    );

    // Call spending calculator (it will save to DB)
    spending::calculate_spending(net_income);
}
