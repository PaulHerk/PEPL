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
    StartFor,
    EndFor,
}
pub fn items() -> [char; 9] {
    ['\\', '+', '-', '>', '<', ',', '?', ':', '|']
}
