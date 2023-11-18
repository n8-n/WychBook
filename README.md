# WychBook
A simple app to choose a random book from a user-provided list. User can assign weight values to each book to make them more likely to be selected.


# Installation
Clone the git repo:  
`git clone git@github.com:n8-n/WychBook.git`


Install using Cargo. To install the CLI tool, use the `--path` parameter:  
`cargo install --path cli/`

Note: if you run the CLI tool and get an error saying that `config.json` file could not be found, you can modify the `cli/build.rs` file to trigger cargo to create this file.


# CLI Usage
### Main Commands:
```
Usage: wych-cli [OPTIONS] <COMMAND>

Commands:
  book    Add, delete, or modify books in your list
  config  Edit your configuration
  list    List table of books
  reset   Reset weight of all books to 1
  sort    Sort book list
  wych    Select a random book based on weight values
  help    Print this message or the help of the given subcommand(s)

Options:
  -q, --quiet        Suppress printing of table, unless `list` is called
  -l, --list <LIST>  Use a list other than the default
  -h, --help         Print help
```  
  

### `book` Command  
```
Add, delete, or modify books in your list

Usage: wych-cli book <COMMAND>

Commands:
  add     Add book to list
  delete  Delete book from list
  weight  Modify the weight of a books
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```  


### `config` Command
```
Edit your configuration

Usage: wych-cli config <COMMAND>

Commands:
  copy     Copy a list to a new list
  default  Set a new default list
  delete   Delete a book list
  list     List names of all book lists
  new      Create a new list
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Example of List
The current book list is printed after every command. You can suppress this printing using the `-q` or `--quiet` command.

```
--------------------
| List Name: books |
--------------------------------------------------------------------------------
|    |       author        |                 title                  |  weight  |
================================================================================
| 0  |     John Fowles     |               The Magus                |    2     |
--------------------------------------------------------------------------------
| 1  |   Virginia Woolf    |             Mrs. Dalloway              |    2     |
--------------------------------------------------------------------------------
| 2  |     Bram Stoker     |                Dracula                 |    1     |
--------------------------------------------------------------------------------
```


## Example Usage
### Modifying Book Elements  
#### Add a new book to the list:  
`wych-cli book add -a "Bram Stoker" -b "Dracula"`  
`wych-cli --list "other_list" book add -a "Mary Shelley" -b "Frankenstein"`  
  
#### Modify the weight of a book:  
`wych-cli book weight -b "Dracula" -w 3`
  
#### Delete a book:  
`wych-cli book delete --book "Dracula"`

  
### Getting a Book Recommendation:  
`wych-cli wych`

  
### Managing Config
#### Create a new book list:  
`wych-cli config new -l other_list`

#### Set a new default list:  
`wych-cli config default -l other_list`

#### Delete a list:  
`wych-cli config delete --list old_list`
