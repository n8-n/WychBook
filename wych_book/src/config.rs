use std::{error::Error, fs::File, io::{Read, Write}};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WychConfig {
    default_list: String,
    all_lists: Vec<String>,
}

// TODO: get filename for books  CSVs

const CONFIG_DIR: &str = "/.config/wych_book/";
const CONFIG_FILE: &str = "config.json";

pub fn config_file() -> Option<String> {
    if let Some(mut config) = home_directory() {
        config.push_str(CONFIG_DIR);
        config.push_str(CONFIG_FILE);
        return Some(config);
    }
    None
}

pub fn read_config(filename: &str) -> Result<WychConfig, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut json = String::new();
    file.read_to_string(&mut json)?;

    let deserialized: WychConfig = serde_json::from_str(&json)?;
    Ok(deserialized)
}


pub fn write_config(filename: &str, config: &WychConfig) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filename).unwrap();
    let serialized = serde_json::to_string(config).unwrap();
    println!("{:?}", serialized);
    write!(file, "{serialized}")?;
    Ok(())
}

fn home_directory() -> Option<String> {
    if let Some(home) = home::home_dir() {
        let home = home.to_str().expect("Should be valid");
        return Some(home.to_string());
    }
    None
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
        assert_eq!(result.all_lists, vec!["books".to_string(), "books2".into(), "books3".into()]);
    }

    #[test]
    fn test_read_config_errors() {
        let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/test/unknown_config.json");
        assert!(read_config(&filename).is_err());

        let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/test/books.csv");
        assert!(read_config(&filename).is_err());
    }

    #[test]
    fn test_write_config() {
        let temp_dir = TempDir::new("wych_book_tests").unwrap();
        let file_path = temp_dir.path().join("new_config.json");
        let filename = file_path.to_str().unwrap();

        let config = WychConfig { default_list: "BOOKS".into(), all_lists: vec!["BOOKS".to_string()] };
        println!("{:?}", config);

        let result = write_config(filename, &config);
        assert!(result.is_ok());

        let read_result = read_config(filename).unwrap();
        assert_eq!(read_result.all_lists, config.all_lists);
        assert_eq!(read_result.default_list, config.default_list);
    }

}