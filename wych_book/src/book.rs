#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Book {
    pub author: String,
    pub title: String,
    pub weight: u8,
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

//
//
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_string_array() {
        let b = Book::new("A. Writer".into(), "Title1".into(), 5);
        assert_eq!(
            b.as_string_array(),
            ["A. Writer".to_string(), "Title1".into(), "5".into()]
        );
    }
}
