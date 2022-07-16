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
    Else,
    EndIf,
    StartLoop,
    EndLoop,
}
pub fn break_items() -> [char; 9] {
    ['\\', '+', '-', '>', '<', ',', '?', ':', '|']
}
