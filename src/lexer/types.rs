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
pub enum ValueKind {
    Hex { value: String, is_reference: bool },
    Dec { value: String, is_reference: bool },
    Bin { value: String, is_reference: bool },
}

pub const COMMAND_CHARACTERS: [char; 9] = ['!', '+', '-', '<', '>', '?', '|', ':', ','];

pub const POSSIBLE_VALUES: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];
