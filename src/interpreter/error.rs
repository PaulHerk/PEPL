#[derive(Debug, Clone, Copy)]
pub enum ErrorkindOnInterpreter {
    TackNotFound,
    NoItemInTack,
    InvalidNumber,
}

#[derive(Debug, Clone, Copy)]
pub struct ErrorOnInterpreter {
    pub kind: ErrorkindOnInterpreter,
    pub token_index: u32,
}

impl ErrorOnInterpreter {
    pub fn new_error(kind: ErrorkindOnInterpreter, token_index: u32) -> Self {
        Self { kind, token_index }
    }
}
