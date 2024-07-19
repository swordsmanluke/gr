use std::fmt::Display;

pub trait Indentable<T>
where T: Display {
    fn indent(&self, spaces: usize) -> String;
    fn indent_with(&self, indent_str: &str, depth: usize) -> String;
}

impl <T> Indentable<T> for T
where T: Display
{
    fn indent(&self, spaces: usize) -> String {
        self.indent_with(" ", spaces)
    }

    fn indent_with(&self, indent_str: &str, depth: usize) -> String {
        self.to_string().split("\n")
            .map(|s| format!("{}{}", indent_str.repeat(depth), s))
            .collect::<Vec<String>>().join("\n")
    }
}