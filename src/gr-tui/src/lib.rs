mod select;
mod symbols;
pub mod string_helpers;

use std::error::Error;
use std::io::stdout;

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    widgets::Paragraph,
    Terminal,
};
use ratatui::prelude::Line;

pub struct Tui {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl Tui {
    pub fn println(&mut self, s: String) -> () {
        self.terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new(Line::from(s + "\n")),
                area,
            );
        }).unwrap();
    }
}

impl Tui {
    pub fn new() -> Tui {
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
        terminal.clear().unwrap();

        Tui { terminal }
    }

    pub fn enter_raw_mode(&mut self) {
        self.terminal.hide_cursor().unwrap();
        enable_raw_mode().unwrap();
    }

    pub fn exit_raw_mode(&mut self) {
        self.terminal.show_cursor().unwrap();
        disable_raw_mode().unwrap();
    }

    pub fn enter_alt_screen(&mut self) {
        stdout().execute(EnterAlternateScreen).unwrap();
    }

    pub fn exit_alt_screen(&mut self) {
        stdout().execute(LeaveAlternateScreen).unwrap();
    }

    pub fn in_raw_mode(&mut self, f: fn() -> Result<(), Box<dyn Error>>) -> Result<(), Box<dyn Error>> {
        self.enter_raw_mode();
        let r = f();
        self.exit_raw_mode();
        r
    }

    pub fn in_alt_screen(&mut self, f: fn() -> Result<(), Box<dyn Error>>) -> Result<(), Box<dyn Error>> {
        self.enter_alt_screen();
        let r = f();
        self.exit_alt_screen();
        r
    }
}
