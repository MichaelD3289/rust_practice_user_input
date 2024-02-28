#[derive(Debug)]
pub enum Actions {
    Add,
    Remove,
    Unknown,
}

impl Actions {
    pub fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "add" => Actions::Add,
            "remove" => Actions::Remove,
            _ => Actions::Unknown,
        }
    }
}
