use std::fmt::Display;
use itertools::Itertools;

pub(crate) trait StrTools<T: Into<String>> {
    fn divide_at(&self, index: usize) -> Triple<String>;
    fn car(&self) -> Option<String>;
    fn cdr(&self) -> Option<String>;
}

pub(crate) struct Triple<T: Display> {
    pub left: Option<T>,
    pub middle: Option<T>,
    pub right: Option<T>,
}

impl<T: Display> Triple<T> {
    pub fn new(left: Option<T>, middle: Option<T>, right: Option<T>) -> Triple<T> {
        Triple {
            left,
            middle,
            right,
        }
    }

    pub fn join(&self) -> String {
        let mut parts = Vec::new();
        if let Some(left) = &self.left {
            parts.push(left.to_string());
        }
        if let Some(middle) = &self.middle {
            parts.push(middle.to_string());
        }
        if let Some(right) = &self.right {
            parts.push(right.to_string());
        }
        parts.join("")
    }
}

impl StrTools<String> for String {
    // Returns a Triple of (left, middle, right), each of which is an Option.
    // left: the 0-(index - 1) characters of the string
    // middle: the index-th character of the string
    // right: the (index + 1)-end characters of the string
    // If any of these ranges are empty (or out of bounds), they will be None in
    // the returned vector
    fn divide_at(&self, index: usize) -> Triple<String> {
        let last_char = self.len();

        if index >= last_char {
            return Triple::new(Some(self.to_string()), None, None);
        }

        if index == last_char - 1 {
            let s = self.split_at(index);
            return Triple::new(Some(s.0.to_string()), s.1.car(), None);
        }

        let (left, right) = self.split_at(index);
        Triple::new(Some(left.to_string()), right.car(), right.cdr())
    }

    fn car(&self) -> Option<String> {
        match self.split_at_checked(1) {
            Some((left, _)) => Some(left.to_string()),
            None => None
        }
    }
    fn cdr(&self) -> Option<String> {
        match self.split_at_checked(1) {
            Some((_, right)) => Some(right.to_string()),
            None => None
        }
    }
}

impl StrTools<String> for &str {
    // Returns a Triple of (left, middle, right), each of which is an Option.
    // left: the 0-(index - 1) characters of the string
    // middle: the index-th character of the string
    // right: the (index + 1)-end characters of the string
    // If any of these ranges are empty (or out of bounds), they will be None in
    // the returned vector
    fn divide_at(&self, index: usize) -> Triple<String> {
        let last_char = self.len();

        if index >= last_char {
            return Triple::new(Some(self.to_string()), None, None);
        }

        if index == last_char - 1 {
            let s = self.split_at(index);
            return Triple::new(Some(s.0.to_string()), s.1.car(), None);
        }

        let (left, right) = self.split_at(index);
        Triple::new(Some(left.to_string()), right.car(), right.cdr())
    }

    fn car(&self) -> Option<String> {
        match self.split_at_checked(1) {
            Some((left, _)) => Some(left.to_string()),
            None => None
        }
    }
    fn cdr(&self) -> Option<String> {
        match self.split_at_checked(1) {
            Some((_, right)) => Some(right.to_string()),
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_divide_at() {
        let s = "hello";
        let t = s.divide_at(3);
        assert_eq!(t.left, Some("hel".to_owned()));
        assert_eq!(t.middle, Some("l".to_owned()));
        assert_eq!(t.right, Some("o".to_owned()));
    }

    #[test]
    fn test_dividing_out_of_bounds() {
        let s = "hello";
        let t = s.divide_at(5);
        assert_eq!(t.left, Some("hello".to_owned()));
        assert_eq!(t.middle, None);
        assert_eq!(t.right, None);
    }

    #[test]
    fn test_dividing_at_end() {
        let s = "hello";
        let t = s.divide_at(4);
        assert_eq!(t.left, Some("hell".to_owned()));
        assert_eq!(t.middle, Some("o".to_owned()));
        assert_eq!(t.right, None);
    }

    #[test]
    fn test_dividing_empty_str() {
        let s = "";
        let t = s.divide_at(0);
        assert_eq!(t.left, Some("".to_owned()));
        assert_eq!(t.middle, None);
        assert_eq!(t.right, None);
    }

    #[test]
    fn test_combining_triple() {
        let t = Triple::new(Some("hello".to_owned()), Some(" ".to_owned()), Some("world".to_owned()));
        assert_eq!(t.join(), "hello world".to_owned());
    }
}