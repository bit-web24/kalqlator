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

impl Error {
    pub fn error(self) -> String {
        format!("Error:\n    at: {}\n    type: {:?}", self.at_char, self.typ)
        /*      let err_msg: String = String::new();
        err_msg.push_str(&format!(
            "error[{}:{}]: {:#?}\n",
            self.at_term, self.at_char, self.typ
        ));
        err_msg.push_str(&format!("          {}\n          ", self.exp));
        for i in 0..self.at_char - 2 {
            err_msg.push(' ');
        }
        for i in 0..self.term_len + 1 {
            err_msg.push('^');
        }

        err_msg*/
    }
}
