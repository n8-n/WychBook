use std::fmt::Display;

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

    pub fn change_weight(&mut self, new_weight: u8) {
        self.weight = new_weight;
    }

    /// Create a string of attributes for displaying to console.
    pub fn print_string(&self) -> String {
        let author = Self::centre_string(&self.author, Header::Author.print_len());
        let title = Self::centre_string(&self.title, Header::Title.print_len());
        let weight = Self::centre_string(&self.weight.to_string(), Header::Weight.print_len());

        format!("|{author}|{title}|{weight}|")
    }

    fn centre_string(string: &str, space: usize) -> String {
        if string.len() >= space {
            return string[..space].to_string();
        }
        format!("{:^space$}", string)
    }
}

impl Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "author: {}, title: {}, weight: {}",
            self.author, self.title, self.weight
        )
    }
}

pub enum Header {
    Author,
    Title,
    Weight,
}

impl Header {
    pub fn value(&self) -> &str {
        match self {
            Header::Author => "author",
            Header::Title => "title",
            Header::Weight => "weight",
        }
    }

    pub fn headers() -> [&'static str; 3] {
        ["author", "title", "weight"]
    }

    pub fn lens() -> [usize; 3] {
        [
            Header::Author.print_len(),
            Header::Title.print_len(),
            Header::Weight.print_len(),
        ]
    }

    // Max string lengths when printing to console
    pub fn print_len(&self) -> usize {
        match self {
            Header::Author => 24,
            Header::Title => 42,
            Header::Weight => 10,
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
    fn test_as_string_array() {
        let b = Book::new("A. Writer".into(), "Title1".into(), 5);
        assert_eq!(
            b.as_string_array(),
            ["A. Writer".to_string(), "Title1".into(), "5".into()]
        );
    }
}
