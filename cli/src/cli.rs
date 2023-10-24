use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about = "A tool for choosing a random book from a weighted list")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Suppress printing of table, unless `list` is called
    #[arg(short, long)]
    pub quiet: bool,

    /// Use a list other than the default
    #[arg(short, long)]
    pub list: Option<String>,
}

#[derive(Subcommand, PartialEq, Eq)]
pub enum Commands {
    /// Add, delete, or modify books in your list
    Book {
        #[command(subcommand)]
        command: BookCommand,
    },

    /// Edit your configuration
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },

    /// List table of books
    List,

    /// Reset weight of all books to 1
    Reset {
        /// Auto-confirm reset check
        #[arg(short = 'y', long = "yes")]
        auto_confirm: bool,
    },

    /// Sort book list
    Sort {
        /// Column to order by
        #[arg(short = 'o', long = "order", value_name = "ORDER BY")]
        input: String,
    },

    /// Select a random book based on weight values
    Wych,
}

#[derive(Subcommand, PartialEq, Eq)]
pub enum BookCommand {
    /// Add book to list
    Add {
        /// Author of book
        #[arg(short, long, value_name = "AUTHOR")]
        author: String,

        /// Title of book
        #[arg(short, long, value_name = "TITLE")]
        title: String,
    },

    /// Delete book from list
    Delete {
        /// Title or index of book
        #[arg(short = 'b', long = "book", value_name = "TITLE | INDEX")]
        input: String,

        /// Auto-confirm deletion check
        #[arg(short = 'y', long = "yes")]
        auto_confirm: bool,
    },

    /// Modify the weight of a books
    Weight {
        /// Title or index of book
        #[arg(short = 'b', long = "book", value_name = "TITLE | INDEX")]
        input: String,
        /// Weight to assign to book
        #[arg(short, long, value_name = "WEIGHT")]
        weight: u8,
    },
}

#[derive(Subcommand, PartialEq, Eq)]
pub enum ConfigCommand {
    /// Copy a list to a new list
    Copy {
        /// List to copy
        #[arg(short, long, value_name = "LIST")]
        from: String,

        /// List to create
        #[arg(short, long, value_name = "NEW LIST")]
        to: String,

        /// Overwrite `to` list if it already exists
        #[arg(short, long)]
        overwrite: bool,
    },

    /// Set a new default list
    Default {
        /// List to set as default
        #[arg(short, long, value_name = "NAME | INDEX")]
        list: String,
    },

    /// Delete a book list
    Delete {
        /// List to delete
        #[arg(short, long, value_name = "NAME | INDEX")]
        list: String,
    },

    /// List names of all book lists
    List,

    /// Create a new list
    New {
        /// List to create
        #[arg(short, long, value_name = "LIST")]
        name: String,
    },
}
