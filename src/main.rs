use std::error::Error;
use gr_tui::Tui;

fn main() -> Result<(), Box<dyn Error>> {
    let mut tui = Tui::new();
    Tui::start();

    let res = run_selection(&mut tui);

    tui.stop();


    res
}

fn run_selection(tui: &mut Tui) -> Result<(), Box<dyn Error>> {
    let options = vec!["Option 1", "Option 2", "Option 3", "Who Knows"].iter().map(|s| s.to_string()).collect();
    let selected_options = tui.select(options, Some("Select an option".to_string()), true)?;
    match selected_options {
        Some(options) => println!("Selected option: {:?}", options.join(",")),
        None => println!("No option selected"),
    }
    Ok(())
}
