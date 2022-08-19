use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum ErrorkindOnLexer {
    InvalidToken,                     // Comment or '\' forgotten?
    NoContent,
    NoValuePut,
    BackslashAfterLastCommand,
    NoOpeningLoop,
    NoEndOrElseIf,
    NoClosingLoop,
    NoOpeningIf,
}
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct ErrorOnLexer {
    pub kind: ErrorkindOnLexer,
    pub position: usize,
}

impl ErrorOnLexer {
    pub fn new_error(kind: ErrorkindOnLexer, position: usize) -> Self {
        Self { kind, position }
    }
}
