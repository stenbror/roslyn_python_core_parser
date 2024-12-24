
pub struct SyntaxError {
    pub position: u32,
    pub message: String,
}

impl SyntaxError {
    pub fn new(position: u32, message: String) -> Self {
        SyntaxError { position, message }
    }
}