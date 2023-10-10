use std::{error::Error, process};
use clap::Parser;
use cli::{Cli, Commands};
use wych_book::book::{Book, Header};

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

    let filename = wych_book::io::CSV_PATH;
    // let _ = OpenOptions::new()
    //     .create_new(true)
    //     .open(filename);

    let mut books = wych_book::io::read_csv_file(filename)?;

    match cli_args.command {
        Commands::Add { author, title } => books.add_book(&author, &title),
        Commands::Delete {
            input,
            auto_confirm,
        } => {
            let book = books.get_book(&input);
            if should_delete(book, auto_confirm)? {
                books.remove_book(&input);
            }
        }
        Commands::List => (),
        Commands::Reset { confirm: _ } => {
            todo!()
        }
        Commands::Sort { input } => {
            let header = Header::from(&input)?;
            books.sort_by(header);
        }
        Commands::Weight { input, weight } => books.change_weight(&input, weight),
        Commands::Wych => {
            if let Some(book) = books.select_random_book() {
                println!("You should read: {} by {}\n", book.title, book.author);
            } else {
                eprintln!("Could not select a book\n");
            }
        }
    };

    println!("{books}\n");

    wych_book::io::write_csv_file(filename, &books)
}

fn should_delete(book: Option<&Book>, auto_confirm: bool) -> Result<bool, Box<dyn Error>>  {
    if auto_confirm {
        return Ok(true);
    }
    
    if book.is_none() {
        eprintln!("No book to delete");
        return Ok(false);
    }
    let book = book.unwrap();

    println!("[Y/n] Delete book: {} by {}?", book.title, book.author);
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)?;

    match input.trim() {
        "Y" | "y" => Ok(true),
        "N" | "n" => Ok(false),
        _ => Err("Invalid user input. Valid choices are [Y/n]".into())
    }
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
