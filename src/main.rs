use process::syn_check; // eval, parse};
use std::io;
mod kalqlator;
mod process;

fn main() -> Result<(), io::Error> {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input)?;

    if !(input.trim().len() < 1) {
        match kalqlate(input) {
            Ok(metadata) => println!("{} = {}", metadata.exp.trim(), metadata.result),
            Err(error) => eprintln!("{}", error.error()),
        }
    } else {
        eprintln!("Error: no input");
    }

    Ok(())
}
fn kalqlate(exp: String) -> Result<kalqlator::MetaData, kalqlator::Error> {
    let chkd_exp: String = syn_check(&exp)?;
    //let prsd_struct: (Vec<i32>, Vec<char>) = parse(chkd_exp)?;
    let result: i32 = 20; //eval(prsd_struct)?;
    Ok(kalqlator::MetaData {
        exp,
        exp_type: kalqlator::ExpressionType::Infix,
        terms: 5,
        result,
    })
}
