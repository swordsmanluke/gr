mod select;
mod symbols;
mod string_helpers;

use std::io::{stdout};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    widgets::Paragraph,
    Terminal,
};

pub struct Tui {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl Tui {
    pub fn new() -> Tui {
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
        terminal.clear().unwrap();

        Tui { terminal }
    }

    pub fn start() {
        stdout().execute(EnterAlternateScreen).unwrap();
        enable_raw_mode().unwrap();
    }

    pub fn stop(&mut self) {
        disable_raw_mode().unwrap();
        self.terminal.show_cursor().unwrap();
        stdout().execute(LeaveAlternateScreen).unwrap();
    }

}
