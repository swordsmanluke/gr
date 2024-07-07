use std::error::Error;
use std::io::Write;
use crate::TuiWidget;

impl<'a> TuiWidget<'a> {

    pub fn prompt(&mut self, prompt: &str) -> Result<String, Box<dyn Error>> {
        self.terminal.show_cursor()?;

        print!("{} ", prompt);
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let res = input.trim().to_string();
        Ok(res)
    }
}