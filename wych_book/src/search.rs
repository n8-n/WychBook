/// Allows you to get an item from a collection using the index of the collection, or some other specified string value.
pub trait IndexSearch {
    type Item;

    /// The collection to be searched over.
    fn get_collection(&self) -> &Vec<Self::Item>;

    /// Checks for equality between the user input string and an item from the collection.
    fn is_equal(&self, item: &Self::Item, input: &str) -> bool;

    /// If input parses into an int, check if it refers to a valid index and return item with its index.
    /// Else if it is a string, search for a matching value.
    /// Returns: Some tuple of index and value if its found found, or None if not.
    fn get_from_input(&self, input: &str) -> Option<(usize, &Self::Item)> {
        let parse: Result<usize, _> = input.parse();
        let collection = self.get_collection();

        if let Ok(index) = parse {
            let get_result = collection.get(index);
            if let Some(result) = get_result {
                Some((index, result))
            } else {
                None
            }
        } else {
            collection.iter().enumerate().find(|(_, item)| self.is_equal(item, input))
        }
    }
}

//
//
//
#[cfg(test)]
mod tests {
    use super::*;

    struct Tester {
        collection: Vec<String>
    }

    impl Tester {
        pub fn values() -> Self {
            Tester { collection: vec!["hello".to_string(), "world".into(), "saluton".into()] }
        }
    }

    impl IndexSearch for Tester {
        type Item = String;

        fn get_collection(&self) -> &Vec<Self::Item> {
            &self.collection
        }

        fn is_equal(&self, item: &Self::Item, input: &str) -> bool {
            item.as_str() == input
        }
    }


    #[test]
    fn test_get_with_index() {
        let tester = Tester::values();

        let result = tester.get_from_input("1");
        let expected = (1 as usize, &"world".to_string());
        assert_eq!(result.unwrap(), expected);

        let result = tester.get_from_input("100");
        assert!(result.is_none());
    }

    #[test]
    fn test_get_with_value() {
        let tester = Tester::values();

        let result = tester.get_from_input("saluton");
        let expected = (2 as usize, &"saluton".to_string());
        assert_eq!(result.unwrap(), expected);

        let result = tester.get_from_input("hola");
        assert!(result.is_none());
    }
}
