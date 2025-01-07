use clap::Parser;

mod commands;
mod db;

use crate::commands::commands::{
    Finance,
    Tracker,
    Category,
    TransactionDetails,
};
use crate::db::db::{create_table, insert_data, view_data, filter_data};

fn main() {
    let finance = Finance::parse();
    match finance.tracker {
        Tracker::Add(details) => {
            let info = TransactionDetails {
                description: details.description,
                amount: details.amount,
                category: details.category,
                date: details.date,
            };
            let _ = create_table();
            let _ = insert_data(&info);
            println!("Transaction added successfully");
        },

        Tracker::View => {
            let _ = view_data();
        },

        Tracker::Filter(details) => {
            let info = Category {
                category: details.category,
            };
            let _ = filter_data(&info.category);
        },
    }
}