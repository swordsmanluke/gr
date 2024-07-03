use std::error::Error;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{KeyCode, KeyEventKind};
use ratatui::text::{Line};
use ratatui::widgets::Paragraph;
use crate::string_helpers::Colorize;
use crate::symbols::{CHECK, RIGHT_TRIANGLE};
use crate::Tui;

struct SelectionState {
    allow_multiple: bool,
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
    }

    pub fn down(&mut self) {
        if self.caret_at < self.num_options - 1 {
            self.caret_at += 1;
        }
    }

    pub fn toggle(&mut self, index: usize) {
        if self.selected_options.contains(&index) {
            self.selected_options.retain(|i| *i != index);
        } else {
            if !self.allow_multiple {
                self.selected_options.clear();
            }
            self.selected_options.push(index);
        }
    }

    pub fn selected(&self, index: usize) -> bool {
        self.selected_options.contains(&index)
    }
}

impl Tui {

    pub fn select(&mut self, options: Vec<String>, prompt: Option<String>, multiple: bool) -> Result<Option<Vec<String>>, Box<dyn Error>> {
        if options.is_empty() {
            return Err("No options provided".into());
        }
        match prompt {
            Some(prompt) => println!("{}", prompt),
            None => (),
        }

        let mut selection_state = SelectionState {
            allow_multiple: multiple,
            caret_at: 0,
            selected_options: Vec::new(),
            num_options: options.len(),
        };

        loop {
            let options = Self::format_options(&options, &mut selection_state);

            // Draw UI
            self.terminal.draw(|frame| {
                let area = frame.size();
                frame.render_widget(
                    Paragraph::new(options),
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
                let line = caret.red() + " " + checkmark.green() + " " + option.to_owned().gold();
                line
            }).into_iter()
            .map(|s| { s.line })
            .collect::<Vec<Line>>();
        options
    }
}