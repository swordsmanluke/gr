mod select;
mod symbols;
pub mod string_helpers;
mod prompt;

use std::error::Error;
use std::io::stdout;
use ratatui::{backend::CrosstermBackend, crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
}, widgets::Paragraph, Terminal};
use ratatui::prelude::Line;
use crate::string_helpers::GrString;

pub struct Tui<'a> {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    scrollback: Vec<Line<'a>>,
}

impl<'a> Tui<'a> {
    pub fn println(&mut self, s: GrString<'a>) -> () {
        self.print(s + "\n");
    }

    pub fn print(&mut self, s: GrString<'a>) -> () {
        s.line.to_string().split("\n").for_each(|lstr| {
            self.scrollback.push(Line::from(lstr.to_owned()));
        });

        while self.scrollback.len() > 100 {
            self.scrollback.remove(0);
        }

        self.terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new(
                    self.scrollback.iter().cloned().collect::<Vec<Line>>()
                ),
                area,
            );
        }).unwrap();
    }
}

impl Tui<'_> {
    pub fn new() -> Tui<'static> {
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
        terminal.clear().unwrap();

        Tui { terminal, scrollback: Vec::new() }
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
