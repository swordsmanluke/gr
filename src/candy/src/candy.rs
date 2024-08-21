use std::io::{stdout, Write};
use crossterm::{event, event::{Event, KeyCode}, execute};
use crossterm::cursor::{MoveDown, MoveTo};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, ScrollDown, Clear, ClearType};
use crate::events::CandyEvent;
use crate::line_editor::OneLineBuffer;
use crate::selector::Selector;

// Candy is a simple TUI crate for rust. It provides a simple way to create
// interactive TUIs in a simple and straightforward way.
pub struct Candy {}

struct EnterRawMode {}

impl Drop for EnterRawMode {
    fn drop(&mut self) {
        disable_raw_mode().expect("Failed to disable raw mode");
    }
}

impl EnterRawMode {
    fn new() -> Self {
        enable_raw_mode().expect("Failed to enable raw mode");
        Self {}
    }
}

impl Candy {
    pub fn new() -> Candy {
        Candy {}
    }

    fn goto(&self, x: u16, y: u16) {
        let mut stdout = stdout();
        execute!(stdout, MoveTo(x, y)).unwrap();
    }

    fn current_cursor_loc(&self) -> (u16, u16) {
        let (x, y) = crossterm::cursor::position().unwrap();
        (x, y)
    }

    fn clear_line(&self) {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::CurrentLine)).unwrap();
    }

    fn scrolldown(&self, n:u16) {
        let mut stdout = stdout();
        execute!(stdout, ScrollDown(n), MoveDown(n)).unwrap();
    }

    fn reset_cur_line(&self) {
        let (_x, y) = crossterm::cursor::position().unwrap();
        self.goto(0, y);
        self.clear_line();
    }

    fn puts(&self, text: impl Into<String>) {
        let mut stdout = stdout();
        write!(stdout, "{}", text.into()).unwrap();
    }

    pub fn edit_line(&self, prompt: &str, default: Option<&str>) -> CandyEvent {
        let text = if default.is_some() { default.unwrap().into() } else { String::new() };
        let mut input = OneLineBuffer::new(text);
        let enter_raw = EnterRawMode::new();
        loop {
            // Clear the current line then print the prompt
            self.reset_cur_line();
            self.puts(format!("{}: {}", prompt, input.display()));
            // Read input in raw mode from Crossterm

            if let Event::Key(event) = event::read().expect("Failed to read crossterm event") {
                match event.code {
                    // Add character
                    KeyCode::Char(c) => input.insert(c),

                    // Cursor Movements
                    KeyCode::Home  => input.home(),
                    KeyCode::Left  => input.left(),
                    KeyCode::Right => input.right(),
                    KeyCode::End   => input.end(),

                    // Deletion
                    KeyCode::Backspace => input.backspace(),
                    KeyCode::Delete    => input.delete(),

                    // Submit/Cancel
                    KeyCode::Enter => return CandyEvent::Submit(input.text().to_string()),
                    KeyCode::Esc   => return CandyEvent::Cancel,

                    // Unhandled
                    _ => {}
                }
            }
        }
    }

    pub fn yn(&self, prompt: impl Into<String>) -> bool {
        let event = self.select_one(prompt, vec!["y", "n"]);
        match event {
            CandyEvent::Submit(selected) => selected == "y",
            _ => false,
        }
    }

    pub fn select_one(&self, prompt: impl Into<String>, options: Vec<impl Into<String> + Clone>) -> CandyEvent {
        let event = self.choose_option(prompt, options, false);
        match event {
            CandyEvent::Select(selections) => CandyEvent::Submit(selections.first().unwrap().to_string()),
            e=> e,
        }
    }

    pub fn choose_option(&self, prompt: impl Into<String>, options: Vec<impl Into<String> + Clone>, multiple: bool) -> CandyEvent {
        let mut block_size: u16 = 7;
        let mut selector = Selector::new(options, multiple, (block_size - 2) as usize);
        let prompt = prompt.into();

        // Get us some room
        println!("{}", "\n\r".repeat(block_size as usize));
        // self.scrolldown(block_size);

        let enter_raw = EnterRawMode::new();

        loop {
            // Clear the current block, then print the prompt
            let (_x, y) = self.current_cursor_loc();
            for i in 0..block_size {
                self.goto(0, y - i);
                self.clear_line();
            }

            self.puts(format!("{}:\n\r", prompt));
            self.puts(selector.display());

            block_size = (selector.display().lines().count() + 1) as u16;

            if let Event::Key(event) = event::read().expect("Failed to read crossterm event") {
                match event.code {
                    // Cursor Movements
                    KeyCode::Home => selector.move_to_beginning(),
                    KeyCode::Up   => selector.up(),
                    KeyCode::Down => selector.down(),
                    KeyCode::Left  | KeyCode::PageUp   => selector.page_up(),
                    KeyCode::Right | KeyCode::PageDown => selector.page_down(),
                    KeyCode::End => selector.move_to_end(),

                    // Select
                    KeyCode::Char(' ') => selector.toggle(),

                    // Submit/Cancel
                    KeyCode::Enter => return CandyEvent::Select(selector.selections()),
                    KeyCode::Esc => return CandyEvent::Cancel,

                    // Unhandled
                    _ => {}
                }
            }
        }
    }
}

