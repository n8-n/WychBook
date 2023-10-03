extern crate csv;

use std::{error::Error, fs::File};
use crate::book_record::BookRecord;


pub fn read_csv_file(filename: &str) -> Result<Vec<BookRecord>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut reader = csv::Reader::from_reader(file);
    let mut book_records = Vec::<BookRecord>::new();

    for result in reader.records() {
        let record = result?;
        
        let author = record[0].to_string();
        let title = record[1].to_string();
        let weight: u8 = record[2].parse()?;

        book_records.push(BookRecord::new(author, title, weight))
    }
    
    Ok(book_records)
}

//
//
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading_csv_file() {
        let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/test/books.csv");
        let result = read_csv_file(&filename).unwrap();
        assert_eq!(4, result.len());

        let record = BookRecord::new("Franz Kakfa".into(), 
            "The Metamorphosis, and other stories".into(), 1);
        assert_eq!(record, result[0]);
        let record = BookRecord::new("Yōko Ogawa".into(), "The Memory Police".into(), 1);
        assert_eq!(record, result[1]);
    }

    #[test]
    fn test_csv_file_errors() {
        let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/test/bad_file.csv");
        assert!(read_csv_file(&filename).is_err());
    }
}