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

pub enum ExpressionType {
    Infix,
    Prefix,
    Postfix,
}

pub struct MetaData {
    pub exp: String,
    pub exp_type: ExpressionType,
    pub terms: u32,
    //pub terms_level1: u32,
    //pub terms_level2: u32,
    //pub terms_level3: u32,
    pub result: i32,
}

#[derive(Debug)]
pub struct Error {
    //exp: String,
    //term_len: u32,
    pub at_char: u32, // with respect to the expression
    pub typ: ErrorType,
    //at_term: u32,
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
