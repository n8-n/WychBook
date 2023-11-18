use clap::Parser;
use cli::{BookCommand, Cli, Commands, ConfigCommand};
use std::{error::Error, process};
use wych_book::{
    books::book::{Book, Header},
    io::config,
};

mod cli;

fn main() {
    let cli = Cli::parse();

    if let Err(error) = run(cli) {
        eprintln!("Application error: {}\n", error);
        process::exit(1);
    }
}

fn run(cli_args: Cli) -> Result<(), Box<dyn Error>> {
    println!();

    let mut config = config::get_config()?;

    if config.get_default().is_empty() {
        let default = prompt_for_default_list()?;
        config.add_new_empty_list(&default)?;
    }

    // list_name used for printing; filename for actual list manipulation
    let (list_name, filename) = if let Some(list) = cli_args.list {
        (list.clone(), config::csv_file(&list))
    } else {
        (config.get_default().to_string(), config.default_csv())
    };

    let mut books = wych_book::io::csv::read_csv_file(&filename)?;
    let mut print_list = !cli_args.quiet; // if quiet, don't print list

    match cli_args.command {
        Commands::Book { command } => match command {
            BookCommand::Add { author, book } => books.add_book(&author, &book),
            BookCommand::Delete {
                input,
                auto_confirm,
            } => {
                let book = books.get_book(&input);
                if should_delete(book, auto_confirm)? {
                    books.remove_book(&input);
                }
            }
            BookCommand::Weight { input, weight } => books.change_weight(&input, weight),
        },
        Commands::Config { command } => {
            print_list = false;
            match command {
                ConfigCommand::Copy {
                    from,
                    to,
                    overwrite,
                } => config.copy_csv_list(&from, &to, overwrite)?,
                ConfigCommand::Delete { list } => config.delete_list(&list)?,
                ConfigCommand::Default { list } => config.set_default(&list)?,
                ConfigCommand::List => (),
                ConfigCommand::New { list } => config.add_new_empty_list(&list)?,
            }
            config.print_lists();
        }
        Commands::List => print_list = true,
        Commands::Reset { auto_confirm } => {
            if should_reset_weights(auto_confirm)? {
                books.reset_weights();
            }
        }
        Commands::Sort { input } => {
            let header = Header::from(&input)?;
            books.sort_by(header);
        }
        Commands::Wych => {
            if let Some(book) = books.select_random_book() {
                println!("You should read: {} by {}\n", book.title, book.author);
            } else {
                eprintln!("Could not select a book\n");
            }
        }
    };

    if print_list {
        let list_print = format!("| List Name: {list_name} |");
        let print_cap = str::repeat("-", list_print.len());
        println!("{print_cap}\n{list_print}\n{books}\n");
    }

    wych_book::io::csv::write_csv_file(&filename, &books)?;
    config::save_config(&mut config)
}

fn should_delete(book: Option<&Book>, auto_confirm: bool) -> Result<bool, Box<dyn Error>> {
    if auto_confirm {
        return Ok(true);
    }

    if book.is_none() {
        eprintln!("No book to delete");
        return Ok(false);
    }
    let book = book.unwrap();

    println!("[Y/n] Delete book: {} by {}?", book.title, book.author);
    prompt_for_choice()
}

fn should_reset_weights(auto_confirm: bool) -> Result<bool, Box<dyn Error>> {
    if auto_confirm {
        return Ok(true);
    }

    println!("[Y/n] Reset weight to 1 for all books?");
    prompt_for_choice()
}

fn prompt_for_choice() -> Result<bool, Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    match input.trim() {
        "Y" | "y" => Ok(true),
        "N" | "n" => Ok(false),
        _ => Err("Invalid user input. Valid choices are [Y/n]".into()),
    }
}

fn prompt_for_default_list() -> Result<String, Box<dyn Error>> {
    println!("Enter name for default book list: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
