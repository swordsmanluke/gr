use std::fmt::Display;

pub trait Indentable<T>
where T: Display {
    fn indent(&self, spaces: usize) -> String;
}

impl <T> Indentable<T> for T
where T: Display
{
    fn indent(&self, spaces: usize) -> String {
        self.to_string().split("\n")
            .map(|s| format!("{}{}", " ".repeat(spaces), s))
            .collect::<Vec<String>>().join("\n")
    }
}