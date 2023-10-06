use rand::{prelude::thread_rng, seq::SliceRandom};

#[derive(Debug, Eq, PartialEq)]
pub struct Book {
    pub author: String,
    pub title: String,
    pub weight: u8,
}

#[derive(Debug, Eq, PartialEq)]
pub struct BookRecords {
    records: Vec<Book>,
}

/// Selects a random book from the provided list of books, based on the associated weight values.
pub fn select_random_book(books: &BookRecords) -> Option<&Book> {
    let mut rng = thread_rng();
    let weights: Vec<usize> = weighted_index_vector(books);
    let result = weights.choose(&mut rng);

    if result.is_none() {
        return None;
    };

    let result = result.unwrap();
    books.get(*result)
}

/// Creates a vector of indexes. Each of these values refer to an index in the BookRecord vector.
/// The index vector will contain one entry for each of the BookRecords weight value. Higher weight values will thus make the BookRecord's index more common.
fn weighted_index_vector(books: &BookRecords) -> Vec<usize> {
    books
        .records
        .iter()
        .enumerate()
        .map(|(index, book)| vec![index; book.weight.into()])
        .flatten()
        .collect()
}

impl Book {
    pub fn new(author: String, title: String, weight: u8) -> Self {
        Book {
            author,
            title,
            weight,
        }
    }

    pub fn as_string_array(&self) -> [String; 3] {
        [
            self.author.clone(),
            self.title.clone(),
            self.weight.to_string(),
        ]
    }

    pub fn headers() -> [&'static str; 3] {
        ["author", "title", "weight"]
    }
}

impl BookRecords {
    pub fn new() -> Self {
        BookRecords { records: Vec::<Book>::new() }
    }

    pub fn get(&self, index: usize) -> Option<&Book> {
        self.records.get(index)
    }

    pub fn records(&self) -> &Vec<Book> {
        &self.records
    }

    pub fn push(&mut self, book: Book) {
        self.records.push(book);
    }
}

impl From<Vec<Book>> for BookRecords {
    fn from(records: Vec<Book>) -> Self {
        BookRecords { records }
    }
}

//
//
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weighted_index_vector() {
        let books = vec![
            Book::new("A. Writer".into(), "Title1".into(), 3),
            Book::new("B. B. Author".into(), "Title2".into(), 2),
            Book::new("C. A. Hack".into(), "Title3".into(), 0),
        ];
        let result = weighted_index_vector(&books.into());
        assert_eq!(result, vec![0, 0, 0, 1, 1])
    }

    #[test]
    fn test_random_book_selector() {
        let books = vec![
            Book::new("A. Writer".into(), "Title1".into(), 0),
            Book::new("B. B. Author".into(), "Title2".into(), 5),
            Book::new("C. A. Hack".into(), "Title3".into(), 0),
        ]
        .into();
        let result = select_random_book(&books);
        // should always return second book
        assert_eq!(result.unwrap(), books.get(1).unwrap());

        let books = vec![Book::new("A. Writer".into(), "Title1".into(), 0)].into();
        assert!(select_random_book(&books).is_none());
    }

    #[test]
    fn test_as_string_array() {
        let b = Book::new("A. Writer".into(), "Title1".into(), 5);
        assert_eq!(
            b.as_string_array(),
            ["A. Writer".to_string(), "Title1".into(), "5".into()]
        );
    }
}
