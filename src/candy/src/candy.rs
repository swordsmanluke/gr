use crossterm::{
    event,
    event::{Event, KeyCode},
    // ExecutableCommand, QueueableCommand,
    // terminal, cursor, style::{self, Stylize}
};
use crate::events::CandyEvent;
use crate::line_editor::OneLineBuffer;

// Candy is a simple TUI crate for rust. It provides a simple way to create
// interactive TUIs in a simple and straightforward way.
pub struct Candy {
    input: OneLineBuffer,
}

impl Candy {
    pub fn new() -> Candy {
        Candy {
            input: OneLineBuffer::new()
        }
    }

    pub fn edit_line(&mut self, prompt: &str) -> CandyEvent {
        self.input.clear();
        loop {
            // Clear the current line then print the prompt
            println!("{}: {}", prompt, self.input.display());
            // Read input in raw mode from Crossterm

            if let Event::Key(event) = event::read().expect("Failed to read crossterm event") {
                match event.code {
                    // Add character
                    KeyCode::Char(c) => self.input.insert(c),

                    // Cursor Movements
                    KeyCode::Home  => self.input.home(),
                    KeyCode::Left  => self.input.left(),
                    KeyCode::Right => self.input.right(),
                    KeyCode::End   => self.input.end(),

                    // Deletion
                    KeyCode::Backspace => self.input.backspace(),
                    KeyCode::Delete    => self.input.delete(),

                    // Submit/Cancel
                    KeyCode::Enter => return CandyEvent::Submit(self.input.text().to_string()),
                    KeyCode::Esc   => return CandyEvent::Cancel,

                    // Unhandled
                    _ => {}
                }
            }
        }
    }

    pub fn choose_option(&mut self, prompt: &str, options: Vec<String>, default: Option<String>, multiple: bool) -> String {
        loop {
            // Clear the current block, then print the prompt
            println!("{}:", prompt);
            for (i, option) in options.iter().enumerate() {

            }
            // Read input in raw mode from Crossterm
        }
    }
}

