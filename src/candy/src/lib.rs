use crate::candy::Candy;

mod candy;
mod asni_mods;
mod str_tools;
mod line_editor;
mod events;
mod selector;

pub fn puts(text: &str) {
    println!("{}", text);
}

pub fn gets() -> String {
    match Candy::new().edit_line("> ") {
        events::CandyEvent::Submit(v) => v,
        events::CandyEvent::Cancel => "".to_string(),
        _ => "".to_string(),
    }
}

pub fn choose(prompt: &str, options: Vec<String>, default: Option<String>, multiple: bool) -> String {
    match Candy::new().choose_option(prompt, options, multiple) {
        events::CandyEvent::Select(v) => v[0].clone(),
        events::CandyEvent::Cancel => default.unwrap_or("".to_string()),
        _ => "".to_string(),
    }
}