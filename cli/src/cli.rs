use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about = "A tool for choosing a random book from a weighted list")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add book to list
    #[command()]
    Add {
        /// Author of book
        #[arg(short, long, value_name = "AUTHOR")]
        author: String,

        /// Title of book
        #[arg(short, long, value_name = "TITLE")]
        title: String,
    },

    /// Delete book from list
    #[command()]
    Delete {
        /// Title or index of book
        #[arg(short = 'b', long = "book", value_name = "TITLE | INDEX")]
        input: String,

        /// Auto-confirm deletion check
        #[arg(short = 'y', long = "yes")]
        auto_confirm: bool,
    },

    /// List all books in CSV file
    #[command()]
    List,

    /// Reset weight of all books to 1
    #[command()]
    Reset {
        /// Auto-confirm reset check
        #[arg(short = 'y', long = "yes")]
        auto_confirm: bool,
    },

    /// Sort book list
    #[command()]
    Sort {
        /// Column to order by
        #[arg(short = 'o', long = "order", value_name = "ORDER BY")]
        input: String,
    },

    /// Modify a books weight
    #[command()]
    Weight {
        /// Title or index of book
        #[arg(short = 'b', long = "book", value_name = "TITLE | INDEX")]
        input: String,
        /// Weight to assign to book
        #[arg(short, long, value_name = "WEIGHT")]
        weight: u8,
    },

    /// Select a random book based on weight values
    #[command()]
    Wych,
}
