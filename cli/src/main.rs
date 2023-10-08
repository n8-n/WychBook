use std::{error::Error, process};

use clap::Parser;
use cli::{Cli, Commands};
use wych_book::{io, book::Header};

mod cli;


// TODO:
//  - Command to reset weights
//  - Create file if it doesn't exist.
//  - Option to point to file path
//  - Config file: default books list
//  - Add integration tests

fn main() {
    let cli = Cli::parse();

    if let Err(error) = run(cli) {
        eprintln!("Application error: {}\n", error);
        process::exit(1);
    }    
    
}

fn run(cli_args: Cli) -> Result<(), Box<dyn Error>> {
    println!();

    let filename = io::CSV_PATH;
    // let _ = OpenOptions::new()
    //     .create_new(true)
    //     .open(filename);

    let mut books = io::read_csv_file(filename)?;

    match cli_args.command {
        Commands::List => (),
        Commands::Add { author, title } => books.add_book(&author, &title),
        Commands::Sort { input } => {
            let header = Header::from(&input)?;
            books.sort_by(header);
        },
        Commands::Delete { input } => books.remove_book(&input),
        Commands::Wych => {
            if let Some(book) = books.select_random_book() {
                println!("You should read: {} by {}\n", book.title, book.author);
            } else {
                eprintln!("Could not select a book\n");
            }
        },
        Commands::Weight { input, weight } => books.change_weight(&input, weight),
    };

    println!("{books}\n");

    io::write_csv_file(filename, &books)
}


//
//
//
#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        println!("test");
    }
}