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
    pub fn print_string(&self, index: usize) -> String {
        let [i_len, a_len, t_len, w_len] = Header::lens();
        let author = Self::centre_string(&self.author, a_len);
        let title = Self::centre_string(&self.title, t_len);
        let weight = Self::centre_string(&self.weight.to_string(), w_len);
        let index = Self::centre_string(&index.to_string(), i_len);

        format!("|{index}|{author}|{title}|{weight}|")
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

// #[derive(Display)]
pub enum Header {
    Index,
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
            _ => "",
        }
    }

    pub fn from(s: &str) -> Result<Self, &'static str> {
        match s {
            "author" => Ok(Header::Author),
            "title" => Ok(Header::Title),
            "weight" => Ok(Header::Weight),
            _ => Err("Invalid sort column choice"),
        }
    }

    pub fn headers() -> [&'static str; 3] {
        ["author", "title", "weight"]
    }

    pub fn lens() -> [usize; 4] {
        [
            Header::Index.print_len(),
            Header::Author.print_len(),
            Header::Title.print_len(),
            Header::Weight.print_len(),
        ]
    }

    // Max string lengths when printing to console.
    // Sums to 75. (80 - 5 line characters between headers.)
    pub fn print_len(&self) -> usize {
        match self {
            Header::Index => 4,
            Header::Author => 21,
            Header::Title => 40,
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
