use chrono::Local;
use rusqlite::{Connection, Result, params};

fn open_db() -> Result<Connection> {
    let conn = Connection::open("spending.db")?;
    // Ensure tables exist
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS monthly_spending (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT NOT NULL,
            period TEXT NOT NULL,
            rent REAL DEFAULT 0,
            groceries REAL DEFAULT 0,
            transport REAL DEFAULT 0,
            bills REAL DEFAULT 0,
            total REAL NOT NULL,
            annual_amount REAL NOT NULL
        );
        CREATE TABLE IF NOT EXISTS shortterm_spending (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT NOT NULL,
            period TEXT NOT NULL,
            amount REAL NOT NULL,
            note TEXT,
            annual_amount REAL NOT NULL
        );
        "#,
    )?;
    Ok(conn)
}

pub fn save_monthly_entry(
    period: &str,
    rent: f64,
    groceries: f64,
    transport: f64,
    bills: f64,
    annual_amount: f64,
) -> Result<()> {
    let conn = open_db()?;
    let today = Local::now().format("%Y-%m-%d").to_string();
    let total = rent + groceries + transport + bills;

    conn.execute(
        "INSERT INTO monthly_spending (date, period, rent, groceries, transport, bills, total, annual_amount)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![today, period, rent, groceries, transport, bills, total, annual_amount],
    )?;
    Ok(())
}

pub fn save_shortterm_entry(
    period: &str,
    amount: f64,
    note: &str,
    annual_amount: f64,
) -> Result<()> {
    let conn = open_db()?;
    let today = Local::now().format("%Y-%m-%d").to_string();

    conn.execute(
        "INSERT INTO shortterm_spending (date, period, amount, note, annual_amount)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![today, period, amount, note, annual_amount],
    )?;
    Ok(())
}
