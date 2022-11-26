use crate::kalqlator::{self, ErrorType};
use std::num::IntErrorKind;

pub fn syn_check(exp: &String) -> Result<String, kalqlator::Error> {
    // checking for valid numbers & arithmatic operators

    let mut at_char: u32 = 1;

    for i in exp.trim().chars() {
        let utoi = i as u32;

        if !(utoi >= 48 && utoi <= 57) {
            if !([32, 37, 42, 43, 45, 47].iter().find(|&&x| x == utoi) == Some(&utoi)) {
                return Err(kalqlator::Error {
                    typ: kalqlator::ErrorType::CharNotDefined,
                    at_char,
                });
            }
        }

        at_char += 1;
    }

    // checking for first and last char in expression
    let f_chars: [u32; 4] = [37, 42, 43, 47];
    let e_chars: [u32; 5] = [37, 42, 43, 45, 47];
    let f_ctoi: u32 = exp.trim().chars().next().unwrap() as u32;
    let l_ctoi: u32 = exp.trim().chars().last().unwrap() as u32;

    if f_chars.iter().find(|&&x| x == f_ctoi) == Some(&f_ctoi) {
        return Err(kalqlator::Error {
            typ: kalqlator::ErrorType::OperandMissing,
            at_char: 0,
        });
    } else {
        if e_chars.iter().find(|&&x| x == l_ctoi) == Some(&l_ctoi) {
            return Err(kalqlator::Error {
                typ: kalqlator::ErrorType::OperandMissing,
                at_char: (exp.trim().len() as u32) + 1,
            });
        }
    }
    // checking for missing operands
    let char_vec: Vec<char> = exp.trim().chars().collect();
    let tmp = vec![37, 42, 43, 45, 47];

    for x in 0..(char_vec.len() - 1) {
        if tmp.iter().find(|&&y| y == char_vec[x] as u32) == Some(&(char_vec[x] as u32))
            && tmp.iter().find(|&&y| y == char_vec[x + 1] as u32) == Some(&(char_vec[x + 1] as u32))
        {
            return Err(kalqlator::Error {
                typ: kalqlator::ErrorType::OperandMissing,
                at_char: (x as u32) + 1,
            });
        }
    }

    Ok(exp.to_string())
}

pub fn parse(
    exp: &String,
) -> Result<(/*operators*/ Vec<u32>, /*operands*/ Vec<u32>), kalqlator::Error> {
    // separating operators and operands
    let mut operands: Vec<u32> = Vec::new();
    let mut operators: Vec<u32> = Vec::new();
    const OPERATORS: [u32; 5] = [37, 42, 43, 45, 47];

    let mut xoperand: String = String::new();
    for x in exp.trim().chars() {
        if OPERATORS.iter().find(|&&i| i == x as u32) == Some(&(x as u32)) {
            if !xoperand.is_empty() {
                chk_psh_trm_vlu(&xoperand, &mut operands)?;
            }
            xoperand.clear();
            operators.push(x as u32);
        } else {
            xoperand.push(x);
        }
    }

    chk_psh_trm_vlu(&xoperand, &mut operands)?;
    Ok((operators, operands))
}

fn chk_psh_trm_vlu(xoperand: &String, operands: &mut Vec<u32>) -> Result<(), kalqlator::Error> {
    match xoperand.parse::<u32>() {
        Err(xx) => {
            return Err(kalqlator::Error {
                typ: {
                    match *xx.kind() {
                        IntErrorKind::Empty => ErrorType::Empty,
                        IntErrorKind::InvalidDigit => ErrorType::InvalidDigit,
                        IntErrorKind::PosOverflow => ErrorType::PosOverflow,
                        IntErrorKind::NegOverflow => ErrorType::NegOverflow,
                        IntErrorKind::Zero => ErrorType::Zero,
                        _ => ErrorType::UnknownError,
                    }
                },
                at_char: 609,
            })
        }
        Ok(xx) => operands.push(xx),
    }
    Ok(())
}

pub fn eval(prsd_stru: (Vec<u32>, Vec<u32>)) -> Result<(i32, u32), kalqlator::Error> {
    let (mut operators, operands) = prsd_stru;
    let mut operands: Vec<i32> = operands.iter().map(|&x| x as i32).collect();

    if operators.len() == operands.len() {
        operands[0] = 0 - operands[0];
        operators.remove(0);
    }

    let terms:u32 = operands.len() as u32;

    #[allow(unused)]
    for o in 0..operators.iter().filter(|&n| *n == '/' as u32).count() {
        for x in 0..operators.len() {
            if operators[x] == '/' as u32 {
                operands[x] = operands[x] / operands[x + 1];
                operators.remove(x);
                operands.remove(x + 1);
                break;
            }
        }
    }

    #[allow(unused)]
    for o in 0..operators.iter().filter(|&n| *n == '*' as u32).count() {
        for x in 0..operators.len() {
            if operators[x] == '*' as u32 {
                operands[x] = operands[x] * operands[x + 1];
                operators.remove(x);
                operands.remove(x + 1);
                break;
            }
        }
    }

    #[allow(unused)]
    for o in 0..operators.iter().filter(|&n| *n == '+' as u32).count() {
        for x in 0..operators.len() {
            if operators[x] == '+' as u32 {
                operands[x] = operands[x] + operands[x + 1];
                operators.remove(x);
                operands.remove(x + 1);
                break;
            }
        }
    }

    #[allow(unused)]
     for o in 0..operators.iter().filter(|&n| *n == '-' as u32).count() {
        for x in 0..operators.len() {
            if operators[x] == '-' as u32 {
                operands[x] = operands[x] - operands[x + 1];
                operators.remove(x);
                operands.remove(x + 1);
                break;
            }
        }
    }

    if operands.is_empty() {
        Err(kalqlator::Error{
            at_char: 0,
            typ: ErrorType::Empty,
        })
    } else {
        Ok((operands[0], terms))
    }
}
