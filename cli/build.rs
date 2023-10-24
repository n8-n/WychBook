use std::{fs, path::Path};

fn main() {
    // Create config and lists directory
    let home = home::home_dir().expect("You really should set your home directory");
    let lists_dir = home.as_path().join(".config/wych_book/lists/");
    fs::create_dir_all(lists_dir).unwrap();

    // Create config file if it doesn't exist
    let config_file = home.as_path().join(".config/wych_book/config.json");
    if !Path::new(&config_file).exists() {
        let config_contents = r#"{"default_list": "","all_lists": []}"#;
        fs::write(config_file, config_contents).expect("Cannot write default config file");
    }

    println!("cargo:rerun-if-changed=build.rs");
}
