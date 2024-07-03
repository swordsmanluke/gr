mod select;
mod symbols;
pub mod string_helpers;

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
        let terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
        // terminal.clear().unwrap();

        Tui { terminal }
    }

    pub fn start() {
        // stdout().execute(EnterAlternateScreen).unwrap();
        enable_raw_mode().unwrap();
    }

    pub fn stop(&mut self) {
        disable_raw_mode().unwrap();
        self.terminal.show_cursor().unwrap();
        // stdout().execute(LeaveAlternateScreen).unwrap();
    }

}
