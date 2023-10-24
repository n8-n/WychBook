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

        // if these are greater than one, we'll need to multiline print
        let author_div = (self.author.len() as f32 / a_len as f32).ceil() as usize;
        let title_div = (self.title.len() as f32 / t_len as f32).ceil() as usize;
        let mut lines = if author_div >= title_div {
            author_div
        } else {
            title_div
        };

        let mut print_string = String::new();

        let mut author = self.author.clone();
        let mut title = self.title.clone();
        let mut weight = self.weight.to_string();
        let mut index = index.to_string();

        while lines > 0 {
            let formatted = format!(
                "|{}|{}|{}|{}|",
                centre(&index, i_len),
                centre(&author, a_len),
                centre(&title, t_len),
                centre(&weight, w_len)
            );

            print_string.push_str(&formatted);

            if lines > 1 {
                print_string.push('\n');
                weight = "".to_string();
                index = "".to_string();

                author = if author_div > 1 {
                    author[a_len..].to_string()
                } else {
                    "".to_string()
                };

                title = if title_div > 1 {
                    title[t_len..].to_string()
                } else {
                    "".to_string()
                }
            }
            lines -= 1;
        }

        print_string
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

fn centre(string: &str, space: usize) -> String {
    let print_str = if string.len() >= space {
        &string[..space]
    } else {
        string
    };
    format!("{:^space$}", print_str)
}

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

    #[test]
    fn test_print() {
        let b = Book::new("A. Writer".into(), "Title1".into(), 5);
        let result =
            "| 0  |      A. Writer      |                 Title1                 |    5     |";
        assert_eq!(result, b.print_string(0));

        let b = Book::new(
            "Anonymous Secretive Writer".into(),
            "Let this be the too long title of their debut: A Novel in multiple parts".into(),
            5,
        );
        let result =
            "| 0  |Anonymous Secretive W|Let this be the too long title of their |    5     |
|    |        riter        |    debut: A Novel in multiple parts    |          |";
        assert_eq!(result, b.print_string(0));
    }
}
