#[derive(Debug, Clone, Copy)]
pub enum ErrorkindOnLexer {
    InvalidToken,  // Comment or '\' forgotten?
    InvalidSyntax, // forgot to put value?
}
#[derive(Debug, Clone, Copy)]
pub struct ErrorOnLexer {
    kind: ErrorkindOnLexer,
    position: u32,
}

impl ErrorOnLexer {
    pub fn new_error(kind: ErrorkindOnLexer, position: u32) -> Self {
        Self { kind, position }
    }
}
