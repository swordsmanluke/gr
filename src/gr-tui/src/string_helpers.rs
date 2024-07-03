use std::ops::Add;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

// Used so we can implement addition for spans into lines
pub struct WrapSpan<'a> {
    span: Span<'a>,
}

impl<'a> Into<Span<'a>> for WrapSpan<'a> {
    fn into(self) -> Span<'a> {
        self.span
    }
}

impl<'a> Into<WrapSpan<'a>> for Span<'a> {
    fn into(self) -> WrapSpan<'a> {
        WrapSpan { span: self }
    }
}

pub struct WrapLine<'a> {
    pub(crate) line: Line<'a>,
}

impl<'a> Into<Line<'a>> for WrapLine<'a> {
    fn into(self) -> Line<'a> {
        self.line
    }
}

impl<'a> Into<WrapLine<'a>> for Line<'a> {
    fn into(self) -> WrapLine<'a> {
        WrapLine { line: self }
    }
}

/*
 Helper methods to set colors on strings, converting them into Spans for Ratatui
 */
pub trait Colorize {
    fn fg<'a>(self, color: Color) -> WrapSpan<'a>;
    fn bg<'a>(self, color: Color) -> WrapSpan<'a>;

    fn red<'a>(self) -> WrapSpan<'a>;
    fn green<'a>(self) -> WrapSpan<'a>;
    fn blue<'a>(self) -> WrapSpan<'a>;
    fn white<'a>(self) -> WrapSpan<'a>;
    fn gold<'a>(self) -> WrapSpan<'a>;
    fn default<'a>(self) -> WrapSpan<'a>;
}

impl Colorize for &str {
    fn fg<'a>(self, color: Color) -> WrapSpan<'a> {
        self.to_string().fg(color)
    }

    fn bg<'a>(self, color: Color) -> WrapSpan<'a> {
        self.to_string().bg(color)
    }

    fn red<'a>(self) -> WrapSpan<'a> {
        self.fg(Color::Red)
    }
    fn green<'a>(self) -> WrapSpan<'a> {
        self.fg(Color::Green)
    }
    fn blue<'a>(self) -> WrapSpan<'a> {
        self.fg(Color::Blue)
    }
    fn white<'a>(self) -> WrapSpan<'a> {
        self.fg(Color::White)
    }

    fn gold<'a>(self) -> WrapSpan<'a> {
        self.fg(Color::Yellow)
    }

    fn default<'a>(self) -> WrapSpan<'a> {
        self.fg(Color::Reset)
    }
}

impl Colorize for String {
    fn fg<'a>(self, color: Color) -> WrapSpan<'a> {
        Span::styled(self, Style::default().fg(color)).into()
    }

    fn bg<'a>(self, color: Color) -> WrapSpan<'a> {
        Span::styled(self, Style::default().bg(color)).into()
    }

    fn red<'a>(self) -> WrapSpan<'a> {
        self.fg(Color::Red)
    }
    fn green<'a>(self) -> WrapSpan<'a> {
        self.fg(Color::Green)
    }
    fn blue<'a>(self) -> WrapSpan<'a> {
        self.fg(Color::Blue)
    }
    fn white<'a>(self) -> WrapSpan<'a> {
        self.fg(Color::White)
    }

    fn gold<'a>(self) -> WrapSpan<'a> {
        self.fg(Color::Yellow)
    }

    fn default<'a>(self) -> WrapSpan<'a> {
        self.fg(Color::Reset)
    }
}

/*
Addition support, converting Spans to Lines
 */

impl<'a> Add<WrapSpan<'a>> for WrapSpan<'a> {
    type Output = WrapLine<'a>;

    fn add(self, rhs: Self) -> Self::Output {
        Line::from(vec![self.span, rhs.span]).into()
    }
}

impl<'a> Add<Span<'a>> for WrapSpan<'a> {
    type Output = WrapLine<'a>;

    fn add(self, rhs: Span<'a>) -> Self::Output {
        // Convert, then add
        self + <Span<'a> as Into<WrapSpan<'a>>>::into(rhs)
    }
}

impl<'a> Add<WrapLine<'a>> for WrapLine<'a> {
    type Output = WrapLine<'a>;

    fn add(self, rhs: Self) -> Self::Output {
        WrapLine { line: Line::from([self.line.spans, rhs.line.spans].concat()) }
    }
}

impl<'a> Add<WrapSpan<'a>> for WrapLine<'a> {
    type Output = WrapLine<'a>;

    fn add(self, rhs: WrapSpan<'a>) -> Self::Output {
        WrapLine { line: Line::from([self.line.spans, vec![rhs.span]].concat()) }
    }
}

impl<'a> Add<&str> for WrapSpan<'a> {
    type Output = WrapLine<'a>;

    fn add(self, rhs: &str) -> Self::Output {
        self + rhs.to_string()
    }
}

impl<'a> Add<String> for WrapSpan<'a> {
    type Output = WrapLine<'a>;

    fn add(self, rhs: String) -> Self::Output {
        self + WrapSpan { span: Span::from(rhs) }
    }
}

impl<'a> Add<&str> for WrapLine<'a> {
    type Output = WrapLine<'a>;

    fn add(self, rhs: &str) -> Self::Output {
        self + rhs.to_string()
    }
}

impl<'a> Add<String> for WrapLine<'a> {
    type Output = WrapLine<'a>;

    fn add(self, rhs: String) -> Self::Output {
        self + WrapSpan { span: Span::from(rhs) }
    }
}

impl<'a> Add<&String> for WrapLine<'a> {
    type Output = WrapLine<'a>;

    fn add(self, rhs: &String) -> Self::Output {
        self + WrapSpan { span: Span::from(rhs.to_owned()) }
    }
}