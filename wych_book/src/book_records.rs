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

    pub fn add_book(&mut self, author: &str, title: &str) {
        self.push(Book { 
            author: author.to_string(), 
            title: title.to_string(), 
            weight: 1 
        });
    }

    /// Can remove book based on index, or title.
    pub fn remove_book(&mut self, input: &str) {
        let parse: Result<usize, _> = input.parse();
        let records = &mut self.records;
        let remove = |r: &mut Vec<Book>, i: usize| if i < r.len() { r.remove(i); };

        if let Ok(index) = parse {
            remove(records, index);
            return;
        }

        // get index title
        let result = records.iter().enumerate().find(|b| b.1.title == input);
        if let Some(result) = result {
            remove(records, result.0);
        }
    }
}

impl From<Vec<Book>> for BookRecords {
    fn from(records: Vec<Book>) -> Self {
        BookRecords { records }
    }
}


// TODO: impl display


//
//
//
#[cfg(test)]
mod tests {
    use super::*;

    fn books_to_test(weights: Vec<u8>) -> BookRecords {
        vec![
            Book::new("B. B. Author".into(), "A Title2".into(), weights[0]),
            Book::new("A. Something Writer".into(), "The Title1".into(), weights[1]),
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

        books.sort_by_author();
        assert_eq!(collect_weights(&books), vec![1, 5, 4]);

        books.sort_by_title();
        assert_eq!(collect_weights(&books), vec![5, 4, 1]);
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
}
