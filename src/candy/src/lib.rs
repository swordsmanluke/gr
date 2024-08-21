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
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

pub fn choose(prompt: &str, options: Vec<String>, default: Option<String>, multiple: bool) -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}