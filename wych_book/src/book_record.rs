
#[derive(Debug, Eq, PartialEq)]
pub struct BookRecord {
    pub author: String,
    pub title: String,
    pub weight: u8
}

impl BookRecord {
    pub fn new(author: String, title: String, weight: u8) -> Self {
        BookRecord { author, title, weight }
    }
}