use colored::{Color, Colorize};

pub(crate) struct ColorCycle {
    cycle: Vec<Color>,
    index: usize
}

impl ColorCycle {
    pub fn new() -> Self {
        let cycle = vec![Color::Cyan,
                          Color::Magenta,
                          Color::BrightGreen,
                          Color::BrightRed,
                          Color::Blue,
                          Color::Yellow,
                          Color::BrightYellow];
        Self {
            cycle,
            index: 0
        }
    }

    pub fn colorize<T: Into<String>>(self, text: T) -> String {
        text.into().color(self.color()).to_string()
    }

    pub fn advance(&mut self) -> Color {
        self.index = (self.index + 1) % self.cycle.len();
        self.color()
    }

    pub fn color(&self) -> Color {
        self.cycle[self.index]
    }
}

