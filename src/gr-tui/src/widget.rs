use std::io::stdout;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::ExecutableCommand;
use ratatui::crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::prelude::Line;
use ratatui::Terminal;

pub struct TuiWidget<'a> {
    pub(crate) terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    scrollback: Vec<Line<'a>>,
}

impl TuiWidget<'_> {
    pub fn new() -> TuiWidget<'static> {
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
        TuiWidget { terminal, scrollback: Vec::new() }
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
}
