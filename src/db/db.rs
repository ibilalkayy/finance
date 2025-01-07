use std::fs;
use std::env;
use dotenv::dotenv;
use std::path::Path;
use postgres::{Client, NoTls, Error};
use crate::commands::commands::{TransactionDetails};

fn connection() -> Result<Client, Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    let client = Client::connect(&database_url, NoTls)?;
    Ok(client)
}

pub fn create_table() -> Result<(), Error> {
    let mut client = connection()?;
    let sql_file_path = Path::new("sql/create_table.sql");

    let sql_query = fs::read_to_string(sql_file_path)
        .expect("Failed to read SQL file");

    client.batch_execute(&sql_query)?;
    Ok(())
}

pub fn insert_data(data: &TransactionDetails) -> Result<(), Error> {
    let mut client = connection()?;
    let _ = client.execute(
        "INSERT INTO transaction(description, amount, category, date) VALUES($1, $2, $3, $4)",
        &[&data.description, &data.amount, &data.category, &data.date]
    );
    Ok(())
}

pub fn view_data() -> Result<(), Error> {
    let mut client = connection()?;
    for row in client.query("SELECT description, amount, category, date FROM transaction", &[])? {
        let description: String = row.get(0);
        let amount: f64 = row.get(1);
        let category: String = row.get(2);
        let date: String = row.get(3);

        println!("Description: {}, Amount: {}, Category: {}, Date: {}", description, amount, category, date);
    }
    Ok(())
}

pub fn filter_data(data: &str) -> Result<(), Error> {
    let mut client = connection()?;
    for row in client.query("SELECT description, amount, category, date FROM transaction where category = $1", &[&data])? {
        let description: String = row.get(0);
        let amount: f64 = row.get(1);
        let category: String = row.get(2);
        let date: String = row.get(3);

        println!("Description: {}, Amount: {}, Category: {}, Date: {}", description, amount, category, date);
    }
    Ok(())
}