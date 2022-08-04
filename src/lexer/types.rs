#[derive(Debug)]
pub enum TokenKind {
    NewTack,
    NewItem,
    DelItem,
    Increase,
    Decrease,
    Output,
    Input,
    BeginIf,
    ReverseIf,
    EndIf,
    StartLoop,
    EndLoop,
}
#[derive(Debug)]
pub struct Value {
    value_kind: ValueKind,
    is_reference: bool,
    value: String,
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

pub const POSSIBLE_VALUES: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];
