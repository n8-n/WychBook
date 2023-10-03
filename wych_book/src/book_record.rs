use rand::{prelude::thread_rng, seq::SliceRandom};

#[derive(Debug, Eq, PartialEq)]
pub struct BookRecord {
    pub author: String,
    pub title: String,
    pub weight: u8,
}

/// Selects a random book from the provided list of books, based on the associated weight values.
pub fn random_book(books: &Vec<BookRecord>) -> Option<&BookRecord> {
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
fn weighted_index_vector(books: &Vec<BookRecord>) -> Vec<usize> {
    books
        .iter()
        .enumerate()
        .map(|(index, book)| vec![index; book.weight.into()])
        .flatten()
        .collect()
}

impl BookRecord {
    pub fn new(author: String, title: String, weight: u8) -> Self {
        BookRecord {
            author,
            title,
            weight,
        }
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
            BookRecord::new("A. Writer".into(), "Title1".into(), 3),
            BookRecord::new("B. B. Author".into(), "Title2".into(), 2),
            BookRecord::new("C. A. Hack".into(), "Title3".into(), 0),
        ];
        let result = weighted_index_vector(&books);
        assert_eq!(result, vec![0, 0, 0, 1, 1])
    }

    #[test]
    fn test_random_book_selector() {
        let books = vec![
            BookRecord::new("A. Writer".into(), "Title1".into(), 0),
            BookRecord::new("B. B. Author".into(), "Title2".into(), 5),
            BookRecord::new("C. A. Hack".into(), "Title3".into(), 0),
        ];
        let result = random_book(&books);
        // should always return second book
        assert_eq!(result.unwrap(), books.get(1).unwrap());

        let books = vec![BookRecord::new("A. Writer".into(), "Title1".into(), 0)];
        assert!(random_book(&books).is_none());
    }
}
