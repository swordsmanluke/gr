use itertools::Itertools;
use crate::asni_mods::AnsiWrapper;

#[derive(Clone)]
pub(crate) struct Selector {
    options: Vec<String>,
    selections: Vec<bool>,
    multiselect: bool,
    cursor: usize,
    page: usize,
    page_size: usize,
}

impl Selector {
    pub fn new(options: Vec<impl Into<String> + Clone>,
               multiselect: bool,
               page_size: usize) -> Selector {
        let cursor = 0;
        let selections = vec![false; options.len()];

        Selector {
            options: options.iter().map(|o| (*o).clone().into()).collect_vec(),
            selections,
            multiselect,
            cursor,
            page: 0,
            page_size,
        }
    }

    pub fn move_to(&mut self, i: usize) {
        if i >= self.options.len() && self.options.len() > 0 {
            return self.move_to_end();
        }

        self.cursor = i;

        if !self.on_current_page(self.cursor) {
            self.page_to_cursor();
        }
    }

    pub fn move_to_beginning(&mut self) {
        self.move_to(0)
    }

    pub fn move_to_end(&mut self) {
        let i = match self.options.len() {
            0 => 0,
            _ => self.options.len() - 1
        };
        self.move_to(i)
    }

    pub fn up(&mut self) {
        if self.cursor > 0 {
            self.move_to(self.cursor - 1);
        }
    }

    pub fn down(&mut self) {
        if self.cursor < self.options.len() - 1 {
            self.move_to(self.cursor + 1);
        }
    }

    pub fn page_up(&mut self) {
        if self.cursor > self.page_size {
            self.move_to(self.cursor - self.page_size);
        } else {
            self.move_to_beginning();
        }
    }

    pub fn page_down(&mut self) {
        if self.cursor + self.page_size < self.options.len() {
            self.move_to(self.cursor + self.page_size);
        } else {
            self.move_to_end();
        }
    }

    pub fn toggle(&mut self) {
        let toggling_on = !self.selections[self.cursor];

        if toggling_on && self.selected_count() >= 1 {
            if !self.multiselect {
                self.delesect_all();
            }
        }

        self.selections[self.cursor] = !self.selections[self.cursor];
    }

    fn delesect_all(&mut self) {
        self.selections = vec![false; self.options.len()];
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

            let option = if selected { option.bold() } else { option.to_owned() };
            let prefix = if cursor_at { ">" } else { " " };

            parts.push(format!("{} {}", prefix, option));
        }
        parts.join("\n")
    }

    fn selected_count(&self) -> usize {
        self.selections.iter().filter(|s| **s).count()
    }

    fn on_current_page(&self, i: usize) -> bool {
        i >= self.page_range().0 && i < self.page_range().1
    }

    fn page_range(&self) -> (usize, usize) {
        (self.page * self.page_size, (self.page + 1) * self.page_size)
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
        let mut selector = Selector::new(vec!["a", "b", "c", "d"], false, 2);
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
        let mut selector = Selector::new(vec!["a", "b", "c", "d"], false, 2);
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

    #[test]
    fn test_selection() {
        let mut selector = Selector::new(vec!["a", "b", "c", "d"], false, 2);
        selector.down(); // move to 'b'
        selector.toggle(); // select 'b'

        assert_eq!(selector.selections(), vec!["b"]);
        assert_eq!(selector.selected_count(), 1);
    }

    #[test]
    fn test_deselection() {
        let mut selector = Selector::new(vec!["a", "b", "c", "d"], false, 2);
        selector.down(); // move to 'b'
        selector.toggle(); // select 'b'
        selector.toggle(); // deselect 'b'

        assert!(selector.selections().is_empty());
        assert_eq!(selector.selected_count(), 0);
    }
}