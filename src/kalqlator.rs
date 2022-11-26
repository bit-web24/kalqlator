#[derive(Debug)]
pub enum ErrorType {
    CharNotDefined,
    OperandMissing,
    Empty,
    InvalidDigit,
    PosOverflow,
    NegOverflow,
    Zero,
    UnknownError,
}
#[derive(Debug)]
pub struct MetaData {
    pub exp: String,
    pub terms: u32,
    pub result: i32,
}

#[derive(Debug)]
pub struct Error {
    pub at_char: u32, // with respect to the expression
    pub typ: ErrorType,
}
