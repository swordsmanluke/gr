use crate::asni_mods::AnsiWrapper;

pub(crate) struct OneLineBuffer {
    text: String,
    pub cursor: usize
}

impl OneLineBuffer {
    pub fn new(text: impl Into<String>) -> OneLineBuffer {
        OneLineBuffer {
            text: text.into(),
            cursor: 0
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn display(&self) -> String {
        self.text.clone()
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor = 0;
    }

    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            self.text.remove(self.cursor - 1);
            self.cursor -= 1;
        }
    }

    pub fn delete(&mut self) {
        if self.cursor < self.text.len() {
            self.text.remove(self.cursor);
            while self.cursor > self.text.len() {
                self.cursor -= 1;
            }
        }
    }

    pub fn home(&mut self) {
        self.cursor = 0;
    }

    pub fn end(&mut self) {
        self.cursor = self.text.len();
    }

    pub fn left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn right(&mut self) {
        if self.cursor < self.text.len() {
            self.cursor += 1;
        }
    }

    pub fn insert(&mut self, c: char) {
        self.text.insert(self.cursor, c);
        self.cursor += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut buffer = OneLineBuffer::new();
        assert_eq!(buffer.cursor, 0);

        buffer.insert('a');
        assert_eq!(buffer.cursor, 1);

        buffer.insert('b');
        assert_eq!(buffer.cursor, 2);

        buffer.insert('c');
        assert_eq!(buffer.cursor, 3);

        assert_eq!(buffer.text, "abc");
    }

    #[test]
    fn test_delete() {
        let mut buffer = OneLineBuffer::new();
        buffer.insert('a');
        buffer.insert('b');
        buffer.insert('c');
        buffer.left();
        buffer.delete();
        assert_eq!(buffer.text, "ab");
        assert_eq!(buffer.cursor, 2);
    }

    #[test]
    fn test_delete_at_end() {
        let mut buffer = OneLineBuffer::new();
        buffer.insert('a');
        buffer.insert('b');
        buffer.insert('c');
        buffer.delete();  // Nothing to delete under the cursor
        assert_eq!(buffer.text, "abc");
        assert_eq!(buffer.cursor, 3);
    }

    #[test]
    fn test_backspace() {
        let mut buffer = OneLineBuffer::new();
        buffer.insert('a');
        buffer.insert('b');
        buffer.insert('c');
        buffer.backspace();
        assert_eq!(buffer.text, "ab");
        assert_eq!(buffer.cursor, 2);
    }

    #[test]
    fn test_home() {
        let mut buffer = OneLineBuffer::new();
        buffer.insert('a');
        buffer.insert('b');
        buffer.insert('c');
        buffer.home();
        assert_eq!(buffer.text, "abc");
        assert_eq!(buffer.cursor, 0);
    }

    #[test]
    fn test_end() {
        let mut buffer = OneLineBuffer::new();
        buffer.insert('a');
        buffer.insert('b');
        buffer.insert('c');
        buffer.end();
        assert_eq!(buffer.text, "abc");
        assert_eq!(buffer.cursor, 3);
    }

    #[test]
    fn test_left() {
        let mut buffer = OneLineBuffer::new();
        buffer.insert('a');
        buffer.insert('b');
        buffer.insert('c');
        buffer.left();
        assert_eq!(buffer.text, "abc");
        assert_eq!(buffer.cursor, 2);
    }

    #[test]
    fn test_right() {
        let mut buffer = OneLineBuffer::new();
        buffer.insert('a');
        buffer.insert('b');
        buffer.insert('c');
        buffer.right();
        assert_eq!(buffer.text, "abc");
        assert_eq!(buffer.cursor, 3);
    }
}

