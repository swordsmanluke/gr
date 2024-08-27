use std::io::{stdout, Write};
use crossterm::{event, event::{Event, KeyCode}, execute};
use crossterm::cursor::{Hide, MoveDown, MoveTo, MoveToColumn, Show};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, ScrollDown, Clear, ClearType, ScrollUp};
use crate::asni_mods::AnsiWrapper;
use crate::events::CandyEvent;
use crate::line_editor::OneLineBuffer;
use crate::selector::Selector;

// Candy is a simple TUI crate for rust. It provides a simple way to create
// interactive TUIs in a simple and straightforward way.
pub struct Candy {}

pub trait CandyOption {
    fn id(&self) -> String;
    fn render(&self, index: usize, cursor_at: usize, selections: &Vec<bool>) -> String;
}

impl CandyOption for &str {
    fn id(&self) -> String {
        self.to_string()
    }

    fn render(&self, index: usize, cursor_at: usize, selections: &Vec<bool>) -> String {
        self.to_string().render(index, cursor_at, selections)
    }
}

impl CandyOption for String {
    fn id(&self) -> String {
        self.clone()
    }

    fn render(&self, index: usize, cursor_at: usize, selections: &Vec<bool>) -> String {
        let mut out = self.to_string();
        if index == cursor_at { out = out.bold(); }
        if selections[index] { out = out.green(); }

        out.to_string()
    }
}


struct EnterRawMode {}

impl EnterRawMode {
    fn new() -> Self {
        let mut stdout = stdout();
        enable_raw_mode().expect("Failed to enable raw mode");
        execute!(stdout, Hide).unwrap();
        Self {}
    }
}

impl Drop for EnterRawMode {
    fn drop(&mut self) {
        disable_raw_mode().expect("Failed to disable raw mode");
        let mut stdout = stdout();
        execute!(stdout, Show, ScrollUp(1), MoveDown(1), MoveToColumn(0)).unwrap();
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
        execute!(stdout, ScrollUp(n), MoveDown(n), MoveToColumn(0)).unwrap();
    }

    fn reset_cur_line(&self) {
        let (_x, y) = crossterm::cursor::position().unwrap();
        self.goto(0, y);
        self.clear_line();
    }

    fn puts(&self, text: impl Into<String>) {
        let mut stdout = stdout();
        write!(stdout, "{}", text.into()).unwrap();
        stdout.flush().unwrap();
    }

    pub fn edit_line(&self, prompt: &str, default: Option<&str>) -> CandyEvent {
        let mut stdout = stdout();
        let text = if default.is_some() { default.unwrap().into() } else { String::new() };
        let mut input = OneLineBuffer::new(text);
        let enter_raw = EnterRawMode::new();
        // We want to show the cursor, so do that first, then set up our cursor on the next line.
        execute!(stdout, Show, ScrollUp(1), MoveDown(1), MoveToColumn(0)).unwrap();
        loop {
            // Clear the current line then print the prompt
            self.reset_cur_line();
            self.puts(format!("{} {}", prompt, input.display()));

            // Put the cursor where it should be
            execute!(stdout, MoveTo((prompt.len() + input.cursor + 1) as u16, self.current_cursor_loc().1)).unwrap();

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
                    KeyCode::Enter => {
                        self.scrolldown(1);
                        return CandyEvent::Submit(input.text().to_string());
                    },
                    KeyCode::Esc   => {
                        self.scrolldown(1);
                        return CandyEvent::Cancel
                    },

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

    pub fn select_one(&self, prompt: impl Into<String>, options: Vec<impl CandyOption + 'static>) -> CandyEvent {
        let event = self.choose_option(prompt, options, false);
        match event {
            CandyEvent::Select(selections) => CandyEvent::Submit(selections.first().unwrap().to_string()),
            e=> e,
        }
    }

    pub fn choose_option(&self, prompt: impl Into<String>, options: Vec<impl CandyOption + 'static>, multiple: bool) -> CandyEvent {
        let mut block_size: u16 = 7;
        let mut selector = Selector::new(options, multiple, (block_size - 2) as usize);
        let prompt = prompt.into();

        // Get us some room
        println!("{}", "\n\r".repeat(block_size as usize));

        let enter_raw = EnterRawMode::new();

        loop {
            // Clear the current block, then print the prompt
            let (_x, y) = self.current_cursor_loc();
            for i in 0..block_size {
                self.goto(0, y - i);
                self.clear_line();
            }

            self.puts(format!("{}\n\r", prompt));
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

