use std::error::Error;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{KeyCode, KeyEventKind};
use ratatui::style::Stylize;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::Paragraph;
use crate::symbols::{CHECK, RIGHT_TRIANGLE};
use crate::widget::TuiWidget;

struct SelectionState {
    allow_multiple: bool,
    auto_select: bool,
    caret_at: usize,
    selected_options: Vec<usize>,
    num_options: usize,
}

enum SelectionEvent {
    Up,
    Down,
    Toggle(usize),
    Accept,
    None
}

impl SelectionState {
    pub fn up(&mut self) {
        if self.caret_at > 0 {
            self.caret_at -= 1;
        }

        if self.auto_select {
            self.select(self.caret_at);
        }
    }

    pub fn down(&mut self) {
        if self.caret_at < self.num_options - 1 {
            self.caret_at += 1;
        }

        if self.auto_select {
            self.select(self.caret_at);
        }
    }

    pub fn select(&mut self, index: usize) {
        if !self.allow_multiple {
            self.selected_options.clear();
        }

        if !self.selected_options.contains(&index) {
            self.selected_options.push(index);
        }
    }

    pub fn unselect(&mut self, index: usize) {
        self.selected_options.retain(|i| *i != index);
    }

    pub fn toggle(&mut self, index: usize) {
        if self.selected_options.contains(&index) {
            self.unselect(index);
        } else {
            self.select(index);
        }
    }

    pub fn selected(&self, index: usize) -> bool {
        self.selected_options.contains(&index)
    }
}

impl TuiWidget {

    pub fn yn(&mut self, prompt: &str) -> Result<bool, Box<dyn Error>> {
        let selection = self.select_one(prompt,vec!["Yes".into(), "No".into()])?;
        match selection {
            Some(choice) => Ok(choice == "Yes"),
            None => Ok(false),
        }
    }

    pub fn select_one(&mut self, prompt: &str, options: Vec<String>) -> Result<Option<String>, Box<dyn Error>> {
        let selections = self.select(options, Some(prompt), false, true)?;
        match selections {
            Some(mut selections) => Ok(selections.pop()),
            None => Ok(None),
        }
    }

    pub fn select_many(&mut self, prompt: &str, options: Vec<String>) -> Result<Option<Vec<String>>, Box<dyn Error>> {
        self.select(options, Some(prompt), true, false)
    }

    pub fn select(&mut self, options: Vec<String>, prompt: Option<&str>, multiple: bool, auto_select: bool) -> Result<Option<Vec<String>>, Box<dyn Error>> {
        if options.is_empty() {
            return Err("No options provided".into());
        }

        self.enter()?;
        let res = self.perform_selection(options, prompt, multiple, auto_select);
        self.exit()?;

        res
    }

    fn perform_selection(&mut self, options: Vec<String>, prompt: Option<&str>, multiple: bool, auto_select: bool) -> Result<Option<Vec<String>>, Box<dyn Error>> {
        let mut selected_options = Vec::new();
        if auto_select {
            selected_options.push(0);
        }

        let mut selection_state = SelectionState {
            allow_multiple: multiple,
            auto_select: auto_select,
            caret_at: 0,
            selected_options: selected_options,
            num_options: options.len(),
        };

        loop {
            let options = Self::format_options(&options, &mut selection_state);
            let prompt = Text::from(prompt.clone().unwrap_or(""));
            let opt_text = Text::from(options.clone());
            let text = Text::from(vec![prompt.lines, opt_text.lines].concat());
            // Draw UI
            self.terminal.draw(|frame| {
                let area = frame.size();
                frame.render_widget(
                    Paragraph::new(text),
                    area,
                );
            })?;

            // Handle Input
            match Self::handle_selection_input(&mut selection_state)? {
                SelectionEvent::Up => selection_state.up(),
                SelectionEvent::Down => selection_state.down(),
                SelectionEvent::Toggle(index) => selection_state.toggle(index),
                SelectionEvent::Accept => break,
                SelectionEvent::None => (),
            }
        }

        if selection_state.selected_options.is_empty() {
            return Ok(None);
        }

        Ok(Some(selection_state.selected_options.iter().map(|i| options[*i].clone()).collect()))
    }

    fn handle_selection_input(selection_state: &SelectionState) -> Result<SelectionEvent, Box<dyn Error>> {
        // Handle Input
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    return match key.code {
                        KeyCode::Down => {
                            Ok(SelectionEvent::Down)
                        }
                        KeyCode::Up => {
                            Ok(SelectionEvent::Up)
                        }
                        KeyCode::Char(' ') => {
                            Ok(SelectionEvent::Toggle(selection_state.caret_at))
                        }
                        KeyCode::Enter => {
                            Ok(SelectionEvent::Accept)
                        }
                        _ => {Ok(SelectionEvent::None)}
                    }
                }
            }
        }

        Ok(SelectionEvent::None)
    }

    fn format_options<'a>(options: &'a Vec<String>, selection_state: &SelectionState) -> Vec<Line<'a>> {
        let options = options
            .iter()
            .enumerate()
            .map(|(index, option)| {
                let careted = index == selection_state.caret_at;
                let caret = if careted { RIGHT_TRIANGLE} else { " " };

                let selected = selection_state.selected(index);
                let checkmark = if selected { CHECK } else { " " };
                let line = Line::from(vec![
                    Span::from(caret).red(),
                    Span::from(" "),
                    Span::from(checkmark).green(),
                    Span::from(" "),
                    Span::from(option)]);
                line
            }).into_iter()
            .map(|s| { s.into() })
            .collect::<Vec<Line>>();
        options
    }
}