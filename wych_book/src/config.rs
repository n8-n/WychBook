use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WychConfig {
    default_list: String,
    all_lists: Vec<String>,
}

const CONFIG_DIR: &str = "/.config/wych_book/";
const CONFIG_FILE: &str = "config.json";

pub fn get_config() -> Result<WychConfig, Box<dyn Error>> {
    read_config(&config_file())
}

pub fn save_config(config: &WychConfig) -> Result<(), Box<dyn Error>> {
    write_config(&config_file(), config)
}

pub fn config_file() -> String {
    let mut config = wych_directory();
    config.push_str(CONFIG_FILE);
    config
}

pub fn csv_file(filename: &str) -> String {
    let mut file = wych_directory();
        file.push_str(filename);
        file.push_str(".csv");
        file
}

impl WychConfig {
    pub fn default_csv(&self) -> String {
        csv_file(&self.default_list)
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
    println!("{:?}", serialized);
    write!(file, "{serialized}")?;
    Ok(())
}

//
//
//
#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

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
}
