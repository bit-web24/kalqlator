use crate::kalqlator;

pub fn syn_check(exp: &String) -> Result<String, kalqlator::Error> {
    // checking for valid numbers & arithmatic operators

    let mut at_char: u32 = 1;

    for i in exp.trim().chars() {
        let utoi = i as u32;

        if !(utoi >= 48 && utoi <= 57) {
            if !([37, 42, 43, 45, 47].iter().find(|&&x| x == utoi) == Some(&utoi)) {
                return Err(kalqlator::Error {
                    typ: kalqlator::ErrorType::CharNotDefined,
                    at_char,
                });
            }
        }

        at_char += 1;
    }

    // checking for first and last char in expression
    let f_chars = [37, 42, 43, 47];
    let e_chars = [37, 42, 43, 45, 47];
    let f_ctoi = exp.trim().chars().next().unwrap() as u32;
    let l_ctoi = exp.trim().chars().last().unwrap() as u32;

    if f_chars.iter().find(|&&x| x == f_ctoi) == Some(&f_ctoi) {
        return Err(kalqlator::Error {
            typ: kalqlator::ErrorType::UnusedOperator,
            at_char: 1,
        });
    } else {
        if e_chars.iter().find(|&&x| x == l_ctoi) == Some(&l_ctoi) {
            return Err(kalqlator::Error {
                typ: kalqlator::ErrorType::UnusedOperator,
                at_char: exp.trim().len() as u32,
            });
        }
    }
    // checking for multiple operators in a row
    let char_vec: Vec<char> = exp.trim().chars().collect();
    for x in 0..char_vec.len() as u32 {
        if [37, 42, 43, 45, 47]
            .iter()
            .find(|&&y| y == char_vec[x as usize] as u32)
            == Some(&(char_vec[x as usize] as u32))
        {
            if [37, 42, 43, 45, 47]
                .iter()
                .find(|&&y| y == *char_vec.get((x + 1) as usize).unwrap() as u32)
                == Some(&(char_vec[x as usize] as u32))
            {
                return Err(kalqlator::Error {
                    typ: kalqlator::ErrorType::MultipleOperators,
                    at_char: x,
                });
            }
        }
    }
    Ok(exp.to_string())
}

//pub fn parse(exp: String) -> Result<(Vec<i32>, Vec<char>), kalqlator::Error> {}

//pub fn eval(prsd_struct: (Vec<i32>, Vec<char>)) -> Result<i32, kalqlator::Error> {}
