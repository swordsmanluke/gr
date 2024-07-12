use std::error::Error;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{KeyCode, KeyEventKind};
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Widget, Paragraph, Clear};
use crate::TuiWidget;

pub(crate) enum EditorEvent {
    Add(char),
    Left,
    Right,
    Delete,
    Backspace,
    Accept,
    Cancel,
    None,
}

pub(crate) enum PromptResult {
    Accept(String),
    Cancel,
    None, // Nothing happened... uet
}

pub struct Prompt {
    state: PromptState,
}

#[derive(Clone, Debug)]
pub struct PromptState {
    prompt: String,
    input: String,
    cursor_pos: usize,
}

impl Prompt {
    pub fn new(state: PromptState) -> Prompt {
        Prompt { state }
    }
}

impl Widget for Prompt {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let prompt_line = Line::from(vec![
            Span::from(format!("{}: ", self.state.prompt).bold()),
            Span::from(self.state.input).gray()
        ]);
        let txt = Text::from(prompt_line);
        Clear.render(area, buf);
        Paragraph::new(txt).render(area, buf);
    }
}

impl PromptState {
    fn new(prompt: &str, input: &str) -> PromptState {
        PromptState {
            prompt: prompt.to_string(),
            input: input.to_string(),
            cursor_pos: input.len(),
        }
    }

    pub fn handle_input(&mut self, event: EditorEvent) -> PromptResult {
        match event {
            EditorEvent::Add(c) => self.add(c),
            EditorEvent::Left => self.left(),
            EditorEvent::Right => self.right(),
            EditorEvent::Delete => self.delete(),
            EditorEvent::Backspace => self.backspace(),
            EditorEvent::Accept => return PromptResult::Accept(self.input.clone()),
            EditorEvent::Cancel => return PromptResult::Cancel,
            EditorEvent::None => {}
        }
        PromptResult::None
    }

    pub fn add(&mut self, c: char) -> () {
        self.input.insert(self.cursor_pos, c);
        self.cursor_pos += 1;
    }

    pub fn left(&mut self) -> () {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }

    pub fn right(&mut self) -> () {
        if self.cursor_pos < self.input.len() {
            self.cursor_pos += 1;
        }
    }

    pub fn delete(&mut self) -> () {
        if self.cursor_pos < self.input.len() && self.input.len() > 0 {
            self.input.remove(self.cursor_pos);
        }
    }

    pub fn backspace(&mut self) -> () {
        if self.cursor_pos > 0 {
            self.input.remove(self.cursor_pos - 1);
            self.cursor_pos -= 1;
        }
    }
}

impl TuiWidget {
    pub fn one_liner(&mut self, prompt: &str, default: Option<&str>) -> Result<String, Box<dyn Error>> {
        self.enter()?;

        let mut input = match default {
            Some(s) => s.to_owned(),
            None => String::new(),
        };

        let mut prompt = PromptState::new(&prompt, &input);

        loop {
            let pw = Prompt::new(prompt.clone());
            self.terminal.draw(|frame| {
                frame.render_widget(pw, frame.size());
            })?;

            let input_event = Self::capture_prompt_input()?;

            match prompt.handle_input(input_event) {
                PromptResult::Accept(s) => {
                    input = s;
                    break;
                }
                PromptResult::Cancel => {
                    input = String::new();
                    break;
                }
                PromptResult::None => {}
            }
        }

        self.exit()?;

        Ok(input.clone())
    }

    pub fn prompt(&mut self, prompt: &str) -> Result<String, Box<dyn Error>> {
        self.one_liner(prompt, None)
    }

    fn capture_prompt_input() -> Result<EditorEvent, Box<dyn Error>> {
        // Handle Input
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    return match key.code {
                        KeyCode::Char(c) => {
                            Ok(EditorEvent::Add(c))
                        }
                        KeyCode::Left => {
                            Ok(EditorEvent::Left)
                        }
                        KeyCode::Right => {
                            Ok(EditorEvent::Right)
                        }
                        KeyCode::Enter => {
                            Ok(EditorEvent::Accept)
                        }
                        KeyCode::Esc => {
                            Ok(EditorEvent::Cancel)
                        }
                        KeyCode::Backspace => {
                            Ok(EditorEvent::Backspace)
                        }
                        KeyCode::Delete => {
                            Ok(EditorEvent::Delete)
                        }
                        _ => { Ok(EditorEvent::None) }
                    };
                }
            }
        }

        Ok(EditorEvent::None)
    }
}