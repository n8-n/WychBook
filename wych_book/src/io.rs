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

pub fn write_csv_file(filename: &str, records: &Vec<BookRecord>) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path(filename)?;

    writer.write_record(&BookRecord::headers())?;

    for record in records {
        writer.write_record(&record.as_string_array())?;
    }

    writer.flush()?;
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

        let temp_dir = TempDir::new("wych_book_tests").unwrap();
        let file_path = temp_dir.path().join("new_books.csv");
        let filename = file_path.to_str().unwrap();

        let mut writer = csv::Writer::from_path(filename).unwrap();
        writer.write_record(vec!["bad", "data"]).unwrap();

        let result = read_csv_file(filename).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_write_csv_file() {
        let temp_dir = TempDir::new("wych_book_tests").unwrap();
        let file_path = temp_dir.path().join("new_books.csv");
        let filename = file_path.to_str().unwrap();

        let records = vec![BookRecord::new("Franz Kakfa".into(), "The Castle".into(), 1)];
        let result = write_csv_file(filename, &records);
        assert!(result.is_ok());

        let read_result = read_csv_file(filename).unwrap();
        assert_eq!(read_result.len(), 1);
        assert_eq!(read_result[0], records[0]);
    }
}