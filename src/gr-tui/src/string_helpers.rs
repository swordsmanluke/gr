use std::fmt::Display;
use std::ops::Add;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

#[derive(Debug, Clone)]
pub struct GrString<'a> {
    pub(crate) line: Line<'a>,
}

impl<'a> GrString<'a> {
    pub fn indent(&self, depth: usize) -> GrString<'a> {
        GrString::from(" ".repeat(depth)) + self.clone()
    }
}

impl<'a> From<String> for GrString<'a> {
    fn from(s: String) -> Self {
        GrString { line: Line::from(s) }
    }
}

impl<'a> From<&str> for GrString<'a> {
    fn from(s: &str) -> Self {
        s.to_owned().into()
    }
}

impl<'a> From<Span<'a>> for GrString<'a> {
    fn from(s: Span<'a>) -> Self {
        GrString { line: Line::from(s) }
    }
}

impl<'a> Into<Line<'a>> for GrString<'a> {
    fn into(self) -> Line<'a> {
        self.line
    }
}

impl<'a> Into<GrString<'a>> for Line<'a> {
    fn into(self) -> GrString<'a> {
        GrString { line: self }
    }
}

/*
 Helper methods to set colors on strings, converting them into Spans for Ratatui
 */
pub trait Colorize {
    fn fg<'a>(self, color: Color) -> GrString<'a>;
    fn bg<'a>(self, color: Color) -> GrString<'a>;

    fn red<'a>(self) -> GrString<'a>;
    fn green<'a>(self) -> GrString<'a>;
    fn blue<'a>(self) -> GrString<'a>;
    fn cyan<'a>(self) -> GrString<'a>;
    fn white<'a>(self) -> GrString<'a>;
    fn gold<'a>(self) -> GrString<'a>;
    fn default<'a>(self) -> GrString<'a>;
}

impl Colorize for &str {
    fn fg<'a>(self, color: Color) -> GrString<'a> {
        self.to_string().fg(color)
    }
    fn bg<'a>(self, color: Color) -> GrString<'a> {
        self.to_string().bg(color)
    }

    fn red<'a>(self) -> GrString<'a> {
        self.fg(Color::Red)
    }
    fn green<'a>(self) -> GrString<'a> {
        self.fg(Color::Green)
    }
    fn blue<'a>(self) -> GrString<'a> {
        self.fg(Color::Blue)
    }
    fn cyan<'a>(self) -> GrString<'a> {
        self.fg(Color::Cyan)
    }
    fn white<'a>(self) -> GrString<'a> {
        self.fg(Color::White)
    }

    fn gold<'a>(self) -> GrString<'a> {
        self.fg(Color::Yellow)
    }

    fn default<'a>(self) -> GrString<'a> {
        self.fg(Color::Reset)
    }
}

impl Colorize for String {
    fn fg<'a>(self, color: Color) -> GrString<'a> {
        Span::styled(self, Style::default().fg(color)).into()
    }

    fn bg<'a>(self, color: Color) -> GrString<'a> {
        Span::styled(self, Style::default().bg(color)).into()
    }

    fn red<'a>(self) -> GrString<'a> {
        self.fg(Color::Red)
    }
    fn green<'a>(self) -> GrString<'a> {
        self.fg(Color::Green)
    }
    fn blue<'a>(self) -> GrString<'a> {
        self.fg(Color::Blue)
    }
    fn cyan<'a>(self) -> GrString<'a> {
        self.fg(Color::Cyan)
    }
    fn white<'a>(self) -> GrString<'a> {
        self.fg(Color::White)
    }
    fn gold<'a>(self) -> GrString<'a> {
        self.fg(Color::Yellow)
    }

    fn default<'a>(self) -> GrString<'a> {
        self.fg(Color::Reset)
    }
}

/*
Addition support, to make GrStrings viral
 */

impl<'a> Add<GrString<'a>> for GrString<'a> {
    type Output = GrString<'a>;

    fn add(self, rhs: Self) -> Self::Output {
        GrString { line: Line::from([self.line.clone().spans, rhs.line.clone().spans].concat()) }
    }
}

impl<'a> Add<&str> for GrString<'a> {
    type Output = GrString<'a>;

    fn add(self, rhs: &str) -> Self::Output {
        self + rhs.to_string()
    }
}

impl<'a> Add<String> for GrString<'a> {
    type Output = GrString<'a>;

    fn add(self, rhs: String) -> Self::Output {
        self + GrString { line: Line::from(rhs.clone()) }
    }
}

impl<'a> Add<&String> for GrString<'a> {
    type Output = GrString<'a>;

    fn add(self, rhs: &String) -> Self::Output {
        self + GrString { line: Line::from(rhs.to_owned()) }
    }
}

impl<'a> Add<&str> for &'a GrString<'a> {
    type Output = GrString<'a>;

    fn add(self, rhs: &str) -> Self::Output {
        GrString { line: Line::from([self.clone().line.spans, Line::from(rhs.to_string()).spans].concat()) }
    }
}