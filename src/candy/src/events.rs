
pub enum CandyEvent {
    Submit(String),
    Confirm(Option<String>),
    Select(Vec<String>),
    Cancel,
}