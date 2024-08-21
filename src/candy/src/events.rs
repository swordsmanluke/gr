
pub enum CandyEvent {
    Submit(String),
    Confirm(Option<String>),
    Cancel,
}