#[derive(Debug, Clone, Copy)]
pub enum ErrorkindOnLexer {
    InvalidToken,                     // Comment or '\' forgotten?
    InvalidSyntax(InvalidSyntaxKind), // forgot to put value?
    NoContent,
}

#[derive(Debug, Clone, Copy)]
pub enum InvalidSyntaxKind {
    NoValuePut,
    BackslashAfterLastCommand,
    NoOpeningLoop,
    NoEndOrElseIf,
    NoClosingLoop,
    NoOpeningIf,
}

#[derive(Debug, Clone, Copy)]
pub struct ErrorOnLexer {
    kind: ErrorkindOnLexer,
    position: usize,
}

impl ErrorOnLexer {
    pub fn new_error(kind: ErrorkindOnLexer, position: usize) -> Self {
        Self { kind, position }
    }
    pub fn new_invalid_syntax_error(kind: InvalidSyntaxKind, position: usize) -> Self {
        Self {
            kind: ErrorkindOnLexer::InvalidSyntax(kind),
            position,
        }
    }
}
