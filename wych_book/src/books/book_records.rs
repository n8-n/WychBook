use std::fmt::Display;

use super::book::{Book, Header};
use crate::search::IndexSearch;

use rand::{prelude::thread_rng, seq::SliceRandom};

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct BookRecords {
    records: Vec<Book>,
}

const MAX_WEIGHT: u8 = 10;

impl BookRecords {
    pub fn get(&self, index: usize) -> Option<&Book> {
        self.records.get(index)
    }

    pub fn records(&self) -> &Vec<Book> {
        &self.records
    }

    pub fn push(&mut self, book: Book) {
        self.records.push(book);
    }

    pub fn sort_by(&mut self, header: Header) {
        let r = &mut self.records;
        match header {
            Header::Author => r.sort_by_key(|b| b.author.clone()),
            Header::Title => r.sort_by_key(|b| b.title.clone()),
            Header::Weight => r.sort_by_key(|b| b.weight),
            _ => (),
        };
    }

    /// Selects a random book from the provided list of books, based on the associated weight values.
    pub fn select_random_book(&self) -> Option<&Book> {
        let mut rng = thread_rng();
        let weights: Vec<usize> = self.weighted_index_vector();
        let result = weights.choose(&mut rng);

        result?;

        let result = result.unwrap();
        self.get(*result)
    }

    /// Creates a vector of indexes. Each of these values refer to an index in the BookRecord vector.
    /// The index vector will contain one entry for each of the BookRecords weight value. Higher weight values will thus make the BookRecord's index more common.
    fn weighted_index_vector(&self) -> Vec<usize> {
        self.records
            .iter()
            .enumerate()
            .flat_map(|(index, book)| vec![index; book.weight.into()])
            .collect()
    }

    pub fn add_book(&mut self, author: &str, title: &str) {
        self.push(Book {
            author: author.to_string(),
            title: title.to_string(),
            weight: 1,
        });
    }

    pub fn get_book(&self, input: &str) -> Option<&Book> {
        let result = self.get_from_input(input);

        if let Some((_, book)) = result {
            return Some(book);
        }

        None
    }

    /// Can remove book based on index, or title.
    pub fn remove_book(&mut self, input: &str) {
        let result = self.get_from_input(input);

        if let Some((i, _)) = result {
            self.records.remove(i);
        }
    }

    /// Reset weight of all books to 1.
    pub fn reset_weights(&mut self) {
        self.records.iter_mut().for_each(|b| b.change_weight(1));
    }

    pub fn change_weight(&mut self, input: &str, new_weight: u8) {
        let new_weight = if new_weight > MAX_WEIGHT {
            MAX_WEIGHT
        } else {
            new_weight
        };

        let result = self.get_from_input(input);

        if let Some((i, _)) = result {
            self.records
                .get_mut(i)
                .expect("Should be valid index")
                .change_weight(new_weight);
        }
    }
}

impl IndexSearch for BookRecords {
    type Item = Book;

    fn get_collection(&self) -> &Vec<Self::Item> {
        self.records()
    }

    fn is_equal(&self, item: &Self::Item, input: &str) -> bool {
        item.title == input
    }
}

impl From<Vec<Book>> for BookRecords {
    fn from(records: Vec<Book>) -> Self {
        BookRecords { records }
    }
}

impl Display for BookRecords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line = str::repeat("-", 80);
        let [i_len, a_len, t_len, w_len] = Header::lens();

        let header = format!(
            "|{:^i_len$}|{:^a_len$}|{:^t_len$}|{:^w_len$}|",
            "", "author", "title", "weight"
        );

        let books = self
            .records()
            .iter()
            .enumerate()
            .map(|(i, b)| b.print_string(i))
            .reduce(|acc, b| format!("{}\n{}\n{}", acc, line, b));
        let books = books.unwrap_or("".into());

        let double_line = str::repeat("=", 80);
        let full_header = format!("{line}\n{header}\n{double_line}");
        let final_string = format!("{full_header}\n{books}\n{line}");
        write!(f, "{final_string}")
    }
}

//
//
//
#[cfg(test)]
mod tests {
    use super::*;

    fn books_to_test(weights: Vec<u8>) -> BookRecords {
        vec![
            Book::new("B. B. Author".into(), "A Title2".into(), weights[0]),
            Book::new(
                "A. Something Writer".into(),
                "The Title1".into(),
                weights[1],
            ),
            Book::new("C. A. Hack".into(), "Bad Title3".into(), weights[2]),
        ]
        .into()
    }

    // Can us weights as an easy way to check sorting order
    fn collect_weights(books: &BookRecords) -> Vec<u8> {
        books.records.iter().map(|b| b.weight).collect()
    }

    #[test]
    fn test_weighted_index_vector() {
        let books: BookRecords = books_to_test(vec![3, 2, 0]);
        let result = books.weighted_index_vector();
        assert_eq!(result, vec![0, 0, 0, 1, 1])
    }

    #[test]
    fn test_random_book_selector() {
        let books: BookRecords = books_to_test(vec![0, 5, 0]);

        let result = books.select_random_book();
        // should always return second book
        assert_eq!(result.unwrap(), books.get(1).unwrap());

        let books: BookRecords = vec![Book::new("A. Writer".into(), "Title1".into(), 0)].into();
        assert!(books.select_random_book().is_none());
    }

    #[test]
    fn test_book_records_sorting() {
        let mut books: BookRecords = books_to_test(vec![5, 1, 4]);

        books.sort_by(Header::Author);
        assert_eq!(collect_weights(&books), vec![1, 5, 4]);

        books.sort_by(Header::Title);
        assert_eq!(collect_weights(&books), vec![5, 4, 1]);

        books.sort_by(Header::Weight);
        assert_eq!(collect_weights(&books), vec![1, 4, 5]);
    }

    #[test]
    fn test_remove_book() {
        let mut books: BookRecords = books_to_test(vec![3, 2, 0]);
        books.remove_book("1");
        books.remove_book("18"); // does nothing
        assert_eq!(collect_weights(&books), vec![3, 0]);

        let mut books: BookRecords = books_to_test(vec![3, 2, 0]);
        let book_title = &books.get(2).unwrap().title.clone();
        books.remove_book(book_title);
        books.remove_book("Non-existent title"); // does nothing
        assert_eq!(collect_weights(&books), vec![3, 2]);
    }

    #[test]
    fn test_change_weight() {
        let mut books: BookRecords = books_to_test(vec![3, 2, 0]);
        books.change_weight("0", 8);
        assert_eq!(collect_weights(&books), vec![8, 2, 0]);

        let book_title = &books.get(2).unwrap().title.clone();
        books.change_weight(book_title, 3);
        assert_eq!(collect_weights(&books), vec![8, 2, 3]);

        books.change_weight("1", 80); // 80 changed to MAX_WEIGHT
        assert_eq!(collect_weights(&books), vec![8, MAX_WEIGHT, 3]);

        // does nothing
        books.change_weight("10", 8); // index doesn't exist
        books.change_weight("Non-existent title", 8); // book doesn't exist
        assert_eq!(collect_weights(&books), vec![8, MAX_WEIGHT, 3]);
    }

    #[test]
    fn test_reset_weights() {
        let mut books: BookRecords = books_to_test(vec![3, 2, 0]);
        books.reset_weights();
        assert_eq!(collect_weights(&books), vec![1; 3]);
    }

    #[test]
    fn test_get_books() {
        let mut books = BookRecords::default();
        books.add_book("Franz Kafka", "The Castle");
        books.add_book("Yoko Ogawa", "The Memory Police");
        assert_eq!(2, books.records().len());

        assert_eq!("Franz Kafka", books.get_book("The Castle").unwrap().author);
        assert_eq!("The Memory Police", books.get_book("1").unwrap().title);
        assert_eq!(None, books.get_book("100"));
        assert_eq!(None, books.get_book("Ulysses"));
    }
}
