use super::book::Book;
use rand::{prelude::thread_rng, seq::SliceRandom};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BookRecords {
    records: Vec<Book>,
}

impl BookRecords {
    pub fn new() -> Self {
        BookRecords {
            records: Vec::<Book>::new(),
        }
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

    pub fn sort_by_author(&mut self) {
        self.records.sort_by_key(|b| b.author.clone());
    }

    pub fn sort_by_title(&mut self) {
        self.records.sort_by_key(|b| b.title.clone());
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
        let books: BookRecords = vec![
            Book::new("A. Writer".into(), "Title1".into(), 3),
            Book::new("B. B. Author".into(), "Title2".into(), 2),
            Book::new("C. A. Hack".into(), "Title3".into(), 0),
        ]
        .into();
        let result = books.weighted_index_vector();
        assert_eq!(result, vec![0, 0, 0, 1, 1])
    }

    #[test]
    fn test_random_book_selector() {
        let books: BookRecords = vec![
            Book::new("A. Writer".into(), "Title1".into(), 0),
            Book::new("B. B. Author".into(), "Title2".into(), 5),
            Book::new("C. A. Hack".into(), "Title3".into(), 0),
        ]
        .into();
        let result = books.select_random_book();
        // should always return second book
        assert_eq!(result.unwrap(), books.get(1).unwrap());

        let books: BookRecords = vec![Book::new("A. Writer".into(), "Title1".into(), 0)].into();
        assert!(books.select_random_book().is_none());
    }

    #[test]
    fn test_book_records_sorting() {
        let mut books: BookRecords = vec![
            Book::new("B. B. Author".into(), "A Title2".into(), 5),
            Book::new("A. Something Writer".into(), "The Title1".into(), 1),
            Book::new("C. A. Hack".into(), "Bad Title3".into(), 4),
        ]
        .into();

        // use weights as easy way to check sorting order
        let weights_collect = |records: &BookRecords| -> Vec<u8> {
            records.records.iter().map(|b| b.weight).collect()
        };

        books.sort_by_author();
        assert_eq!(weights_collect(&books), vec![1, 5, 4]);

        books.sort_by_title();
        assert_eq!(weights_collect(&books), vec![5, 4, 1]);
    }
}
