#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    NewTack,
    NewItem,
    DelItem,
    Increase,
    Decrease,
    Output,
    Input,
    BeginIf(u32),
    Else(u32),
    EndIf(u32),
    StartLoop(u32),
    BreakLoop(Option<u32>),
}

#[derive(Debug)]
pub struct Value {
    pub value_kind: ValueKind,
    pub is_reference: bool,
    pub value: String,
}

#[derive(Debug)]
pub enum ValueKind {
    Hex,
    Dec,
    Bin,
}

impl Value {
    pub fn new_value(value_kind: ValueKind, is_reference: bool, value: String) -> Self {
        Self {
            value_kind,
            is_reference,
            value,
        }
    }
}

pub const COMMAND_CHARACTERS: [char; 9] = ['!', '+', '-', '<', '>', '?', '|', ':', ','];
