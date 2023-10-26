#[derive(Debug)]
pub enum MyError {
    Overflow,
}
#[derive(Debug)]
pub enum CharacterErrors {
    NotAsciiChar(char),
    CharIsNotADigit(char),
    CharIsNotABase16Digit(char),
    CharIsNotALetter(char),
    CharIsNotPrintable(char),
}
