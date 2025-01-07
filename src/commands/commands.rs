use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, about, version)]
pub struct Finance {
    #[clap(subcommand)]
    pub tracker: Tracker,
}

#[derive(Debug, Subcommand)]
pub enum Tracker {
    /// Add a transaction
    Add(TransactionDetails),

    /// View transactions
    View,

    /// Filter transactions by category
    Filter(Category),
}

#[derive(Debug, Parser)]
pub struct TransactionDetails {
    /// Write the description of transaction
    #[clap(short, long)]
    pub description: String,

    /// Write the amount of transaction
    #[clap(short, long)]
    pub amount: f64,

    /// Write the category of transaction
    #[clap(short, long)]
    pub category: String,

    /// Write the date of transaction
    #[clap(short, long)]
    pub date: String,
}

#[derive(Debug, Parser)]
pub struct Category {
    /// Write the category of transaction
    #[clap(short, long)]
    pub category: String,
}