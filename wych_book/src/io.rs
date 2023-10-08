extern crate csv;

use crate::{
    book::{Book, Header},
    book_records::BookRecords,
};
use std::{error::Error, fs::File};

// TODO: make this configurable
pub const CSV_PATH: &str = "~/.config/wych_book/books.csv";

pub fn read_csv_file(filename: &str) -> Result<BookRecords, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut reader = csv::Reader::from_reader(file);
    let mut book_records = BookRecords::default();

    for result in reader.records() {
        let record = result?;

        let author = record[0].to_string();
        let title = record[1].to_string();
        let weight: u8 = record[2].parse()?;

        book_records.push(Book::new(author, title, weight))
    }

    Ok(book_records)
}

pub fn write_csv_file(filename: &str, books: &BookRecords) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path(filename)?;

    writer.write_record(Header::headers())?;

    for record in books.records() {
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
        assert_eq!(4, result.records().len());

        println!("{result}");

        let record = Book::new(
            "Franz Kakfa".into(),
            "The Metamorphosis, and other stories".into(),
            1,
        );
        assert_eq!(record, *result.get(0).unwrap());
        let record = Book::new("Yōko Ogawa".into(), "The Memory Police".into(), 1);
        assert_eq!(record, *result.get(1).unwrap());
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
        assert!(result.records().is_empty());
    }

    #[test]
    fn test_write_csv_file() {
        let temp_dir = TempDir::new("wych_book_tests").unwrap();
        let file_path = temp_dir.path().join("new_books.csv");
        let filename = file_path.to_str().unwrap();

        let records = vec![Book::new("Franz Kakfa".into(), "The Castle".into(), 1)].into();
        let result = write_csv_file(filename, &records);
        assert!(result.is_ok());

        let read_result = read_csv_file(filename).unwrap();
        assert_eq!(read_result.records().len(), 1);
        assert_eq!(read_result.get(0), records.get(0));
    }
}
