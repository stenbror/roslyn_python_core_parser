
pub struct SyntaxError {
    pub position: usize,
    pub message: String,
}

impl SyntaxError {
    pub fn new(position: usize, message: String) -> Self {
        SyntaxError { position, message }
    }
}