use crossterm::{
    event,
    event::{Event, KeyCode},
    // ExecutableCommand, QueueableCommand,
    // terminal, cursor, style::{self, Stylize}
};
use crate::events::CandyEvent;
use crate::line_editor::OneLineBuffer;
use crate::selector::Selector;

// Candy is a simple TUI crate for rust. It provides a simple way to create
// interactive TUIs in a simple and straightforward way.
pub struct Candy {}

impl Candy {
    pub fn new() -> Candy {
        Candy {}
    }

    fn goto(&mut self, x: u16, y: u16) {
        println!("\x1b[{};{}H", x, y)
    }

    fn current_cursor_loc(&mut self) -> (u16, u16) {
        let (x, y) = crossterm::cursor::position().unwrap();
        (x, y)
    }

    fn clear_line(&mut self) {
        println!("\x1b[2K")
    }

    fn reset_cur_line(&mut self) {
        let (_x, y) = crossterm::cursor::position().unwrap();
        self.goto(0, y);
        self.clear_line();
    }

    pub fn edit_line(&mut self, prompt: &str) -> CandyEvent {
        let mut input = OneLineBuffer::new();
        loop {
            // Clear the current line then print the prompt
            self.reset_cur_line();
            println!("{}: {}", prompt, input.display());
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

    pub fn choose_option(&mut self, prompt: &str, options: Vec<String>, multiple: bool) -> CandyEvent {
        let block_size: u16 = 7;
        let mut selector = Selector::new(options, multiple, (block_size - 2) as usize);

        // Get us some room
        println!("{}", "\n".repeat(block_size as usize));



        loop {
            // Clear the current block, then print the prompt
            let (_x, y) = self.current_cursor_loc();
            for i in 0..block_size {
                self.goto(0, y - i);
                self.clear_line();
            }

            println!("{}:", prompt);
            println!("{}", selector.display());

            if let Event::Key(event) = event::read().expect("Failed to read crossterm event") {
                match event.code {
                    // Cursor Movements
                    KeyCode::Home => selector.move_to_beginning(),
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

