use super::csv::{self, read_csv_file, write_csv_file};
use crate::search::IndexSearch;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{Read, Write},
    path::Path,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct WychConfig {
    default_list: String,
    all_lists: Vec<String>,
}

const CONFIG_DIR: &str = "/.config/wych_book/";
const CONFIG_FILE: &str = "config.json";
const LISTS_DIR: &str = "lists/";

pub fn get_config() -> Result<WychConfig, Box<dyn Error>> {
    read_config(&config_file())
}

pub fn save_config(config: &mut WychConfig) -> Result<(), Box<dyn Error>> {
    config.validate_config();
    write_config(&config_file(), config)
}

pub fn config_file() -> String {
    let mut config = wych_directory();
    config.push_str(CONFIG_FILE);
    config
}

pub fn csv_file(name: &str) -> String {
    let mut file = wych_directory();
    file.push_str(LISTS_DIR);
    file.push_str(name);
    file.push_str(".csv");
    file
}

pub fn does_list_exist(name: &str) -> bool {
    let filename = csv_file(name);
    Path::new(&filename).exists()
}

impl WychConfig {
    pub fn default_csv(&self) -> String {
        csv_file(&self.default_list)
    }

    pub fn set_default(&mut self, new_default: &str) -> Result<(), Box<dyn Error>> {
        if !does_list_exist(new_default) {
            return Err("Provided list does not exist".into());
        }
        self.default_list = new_default.to_string();
        Ok(())
    }

    pub fn print_lists(&self) {
        println!("{self}");
    }

    pub fn add_new_empty_list(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        if does_list_exist(name) {
            println!("List already exists");
            return Ok(());
        }

        let filename = csv_file(name);
        let result = csv::create_blank_file(&filename);

        if result.is_ok() {
            self.all_lists.push(name.to_string());
        }
        result
    }

    pub fn copy_csv_list(
        &mut self,
        from: &str,
        to: &str,
        overwrite: bool,
    ) -> Result<(), Box<dyn Error>> {
        if !does_list_exist(from) {
            return Err("Cannot copy a non-existent list".into());
        }
        if does_list_exist(to) && !overwrite {
            println!("List {to} already exists, use -o to overwrite.");
            return Ok(());
        }

        let from_list = read_csv_file(&csv_file(from))?;
        write_csv_file(&csv_file(to), &from_list)?;
        self.all_lists.push(to.to_string());
        Ok(())
    }

    pub fn delete_list(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        if !does_list_exist(name) {
            return Err("Cannot delete a non-existent list".into());
        }
        if name == self.default_list {
            return Err("Cannot delete default list".into());
        }
        let filename = csv_file(name);
        std::fs::remove_file(filename)?;

        match self.get_from_input(name) {
            Some((index, _)) => {
                self.all_lists.remove(index);
                Ok(())
            }
            None => Err("Could not find list".into()),
        }
    }

    /// Check if all the lists in the config file actually exist and remove any that don't.
    fn validate_config(&mut self) {
        let existent_lists: Vec<String> = self
            .all_lists
            .iter()
            .filter(|l| does_list_exist(l))
            .map(|l| l.to_string())
            .collect();

        if self.all_lists.len() != existent_lists.len() {
            self.all_lists = existent_lists;
        }

        if !does_list_exist(&self.default_list) {
            self.default_list = if self.all_lists.is_empty() {
                String::new()
            } else {
                self.all_lists[0].clone()
            }
        }
    }
}

impl Display for WychConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lists = self
            .all_lists
            .iter()
            .enumerate()
            .fold(String::new(), |acc, (i, l)| format!("{acc}- {i}: {l}\n"));

        write!(
            f,
            "Default List: {}\nAll Lists:\n{}",
            self.default_list, lists
        )
    }
}

impl IndexSearch for WychConfig {
    type Item = String;

    fn get_collection(&self) -> &Vec<Self::Item> {
        &self.all_lists
    }

    fn is_equal(&self, item: &Self::Item, input: &str) -> bool {
        item == input
    }
}

fn wych_directory() -> String {
    let home = home::home_dir().expect("You really should set your home directory");
    let mut home = home.to_str().expect("Should be valid str").to_string();
    home.push_str(CONFIG_DIR);
    home
}

fn read_config(filename: &str) -> Result<WychConfig, Box<dyn Error>> {
    let file = File::open(filename);
    if let Err(e) = file {
        return Err(format!("Cannot open: {filename}, {e}").into());
    };

    let mut json = String::new();
    file.unwrap().read_to_string(&mut json)?;

    let deserialized: WychConfig = serde_json::from_str(&json)?;
    Ok(deserialized)
}

fn write_config(filename: &str, config: &WychConfig) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filename).unwrap();
    let serialized = serde_json::to_string(config).unwrap();
    write!(file, "{serialized}")?;
    Ok(())
}

//
//
//
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempdir::TempDir;

    fn set_up_home_dir() -> TempDir {
        let temp_dir = TempDir::new("wych_book_tests").unwrap();
        let lists_dir = temp_dir.path().join(".config/wych_book/lists/");
        fs::create_dir_all(lists_dir).unwrap();

        std::env::set_var("HOME", temp_dir.path().to_str().unwrap());

        temp_dir
    }

    #[test]
    fn test_read_config() {
        let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/test/config.json");
        let result = read_config(filename).unwrap();
        assert_eq!(result.default_list, "books".to_string());
        assert_eq!(
            result.all_lists,
            vec!["books".to_string(), "books2".into(), "books3".into()]
        );
    }

    #[test]
    fn test_read_config_errors() {
        let filename = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/test/unknown_config.json"
        );
        assert!(read_config(&filename).is_err());

        let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/test/books.csv");
        assert!(read_config(&filename).is_err());
    }

    #[test]
    fn test_write_config() {
        let temp_dir = TempDir::new("wych_book_tests").unwrap();
        let file_path = temp_dir.path().join("new_config.json");
        let filename = file_path.to_str().unwrap();

        let config = WychConfig {
            default_list: "BOOKS".into(),
            all_lists: vec!["BOOKS".to_string()],
        };
        println!("{:?}", config);

        let result = write_config(filename, &config);
        assert!(result.is_ok());

        let read_result = read_config(filename).unwrap();
        assert_eq!(read_result.all_lists, config.all_lists);
        assert_eq!(read_result.default_list, config.default_list);
    }

    #[test]
    fn test_create_new_list() {
        let list_name = String::from("books");
        let mut config = WychConfig {
            default_list: list_name.clone(),
            all_lists: vec![list_name.clone()],
        };

        let _temp_dir = set_up_home_dir();

        let new_list = "new_list";
        assert!(config.add_new_empty_list(&new_list).is_ok());
        assert!(does_list_exist(&new_list));
        assert!(config.all_lists.contains(&new_list.to_string()));

        // try create again
        assert!(config.add_new_empty_list(&new_list).is_ok());
    }

    #[test]
    fn test_validate_config() {
        let _temp_dir = set_up_home_dir();

        let list_name = String::from("books");
        let mut config = WychConfig {
            default_list: String::new(),
            all_lists: vec![String::from("does_not_exist")],
        };

        let _ = config.add_new_empty_list(&list_name);
        config.validate_config();
        assert!(config.all_lists.len() == 1);
        assert_eq!(config.default_list, list_name);
    }
}
