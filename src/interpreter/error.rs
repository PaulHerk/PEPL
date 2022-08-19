use wasm_bindgen::prelude::wasm_bindgen;
#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub enum ErrorkindOnInterpreter {
    TackNotFound,
    NoItemInTack,
    InvalidNumber,
    NoSuchCharacter,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct ErrorOnInterpreter {
    pub kind: ErrorkindOnInterpreter,
    pub position: u32,
}

impl ErrorOnInterpreter {
    pub fn new_error(kind: ErrorkindOnInterpreter, position: u32) -> Self {
        Self { kind, position }
    }
}
