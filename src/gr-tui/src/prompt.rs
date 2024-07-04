use std::error::Error;
use crate::string_helpers::WrapLine;
use crate::Tui;

impl<'a> Tui<'a> {

    pub fn prompt(&mut self, prompt: WrapLine<'a>) -> Result<String, Box<dyn Error>> {
        self.exit_raw_mode();
        self.print(prompt.into());
        let cursor = self.terminal.get_cursor()?;
        self.terminal.set_cursor(cursor.0 + 1, cursor.1)?;
        self.terminal.show_cursor()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let res = input.trim().to_string();
        Ok(res)
    }
}