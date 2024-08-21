use std::cmp::min;
use itertools::Itertools;
use crate::asni_mods::AnsiWrapper;

pub(crate) struct Selector {
    options: Vec<String>,
    selections: Vec<bool>,
    multiselect: bool,
    allow_none: bool,
    cursor: usize,
    page: usize,
    page_size: usize,
}

impl Selector {
    pub fn new(options: Vec<impl Into<String> + Clone>,
               multiselect: bool,
               allow_none: bool,
               page_size: usize) -> Selector {
        let cursor = 0;
        let mut selections = Vec::new();
        for _ in options.iter() {
            selections.push(false);
        }
        Selector {
            options: options.iter().map(|o| (*o).clone().into()).collect_vec(),
            selections,
            multiselect,
            allow_none,
            cursor,
            page: 0,
            page_size,
        }
    }

    pub fn up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }

        if !self.on_current_page(self.cursor) {
            self.page_to_cursor();
        }
    }

    pub fn down(&mut self) {
        if self.cursor < self.options.len() - 1 {
            self.cursor += 1;
        }

        if !self.on_current_page(self.cursor) {
            self.page_to_cursor();
        }
    }

    pub fn page_up(&mut self) {
        if self.cursor > self.page_size {
            self.cursor -= self.page_size;
        } else {
            self.cursor = 0;
        }

        self.page_to_cursor();
    }

    pub fn page_down(&mut self) {
        if self.cursor + self.page_size < self.options.len() {
            self.cursor += self.page_size;
        } else {
            if self.options.is_empty() {
                self.cursor = 0;
            } else {
                self.cursor = self.options.len();
            }
        }

        self.page_to_cursor();
    }

    pub fn toggle(&mut self) {
        self.selections[self.cursor] = !self.selections[self.cursor];
    }

    pub fn selections(&self) -> Vec<String> {
        let mut selections = Vec::new();
        for (i, s) in self.selections.iter().enumerate() {
            if *s {
                selections.push(self.options[i].clone());
            }
        }
        selections
    }

    pub fn display(&self) -> String {
        let mut parts = Vec::new();

        for (i, option) in self.options.iter().enumerate() {
            if !self.on_current_page(i) { continue; }

            let cursor_at = i == self.cursor;
            let selected = self.selections[i];

            let option = if selected { option.bold() }
                        else { option.to_owned() };
            let prefix = if cursor_at { ">" } else { " " };

            parts.push(format!("{} {}", prefix, option));
        }
        parts.join("\n")
    }

    fn on_current_page(&self, i: usize) -> bool {
        i >= self.page_range().0 && i < self.page_range().1
    }

    fn page_range(&self) -> (usize, usize) {
        (self.page * self.page_size, (self.page + 1) * self.page_size)
    }

    fn page(&self, i: usize, size: usize) -> Vec<String> {
        let page_start = i * size;
        let page_end = min((i + 1) * size, self.options.len());

        if page_start >= self.options.len() { return Vec::new(); }

        self.options[page_start..page_end].to_vec()
    }

    fn page_to_cursor(&mut self) {
        self.page = self.cursor / self.page_size;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_moves_cursor() {
        let mut selector = Selector::new(vec!["a", "b", "c", "d"], false, false, 2);
        assert_eq!(selector.cursor, 0);
        assert_eq!(selector.page, 0);

        selector.page_down();
        assert_eq!(selector.cursor, 2);
        assert_eq!(selector.page, 1);

        selector.page_up();
        assert_eq!(selector.cursor, 0);
        assert_eq!(selector.page, 0);
    }

    #[test]
    fn test_page_follows_cursor() {
        let mut selector = Selector::new(vec!["a", "b", "c", "d"], false, false, 2);
        assert_eq!(selector.page, 0);
        assert_eq!(selector.cursor, 0);

        selector.down();
        assert_eq!(selector.page, 0);
        assert_eq!(selector.cursor, 1);

        selector.down();
        selector.down();
        assert_eq!(selector.cursor, 3);
        assert_eq!(selector.page, 1);

        selector.up();
        selector.up();
        assert_eq!(selector.cursor, 1);
        assert_eq!(selector.page, 0);
    }
}