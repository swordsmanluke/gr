use std::error::Error;
use std::io::stdout;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::ExecutableCommand;
use ratatui::crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::Terminal;

pub struct TuiWidget {
    pub(crate) terminal: Terminal<CrosstermBackend<std::io::Stdout>>
}

impl TuiWidget {
    pub fn new() -> TuiWidget {
        let terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
        TuiWidget { terminal }
    }

    pub fn enter(&mut self) -> Result<(), Box<dyn Error>>{
        self.enter_alt_screen()?;
        self.enter_raw_mode()?;
        self.terminal.clear()?;
        self.terminal.hide_cursor()?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<(), Box<dyn Error>>{
        self.exit_alt_screen()?;
        self.exit_raw_mode()?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    pub fn enter_raw_mode(&mut self) -> Result<(), Box<dyn Error>> {
        self.terminal.hide_cursor()?;
        enable_raw_mode()?;
        Ok(())
    }

    pub fn exit_raw_mode(&mut self) -> Result<(), Box<dyn Error>> {
        self.terminal.show_cursor()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn enter_alt_screen(&mut self) -> Result<(), Box<dyn Error>> {
        stdout().execute(EnterAlternateScreen)?;
        Ok(())
    }

    pub fn exit_alt_screen(&mut self) -> Result<(), Box<dyn Error>> {
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }
}
