#![allow(dead_code)] //TEMP while building
use std::io::{self, Write};
use std::fmt::{Display, Formatter, Error as FmtError};
use std::error::Error;
use std::num::ParseIntError;

/* Custom errors */
#[derive(Debug, Clone)]
pub enum ExtractOperationError {
    InvalidOperator,
    InvalidInteger(ParseIntError),
}

impl From<ParseIntError> for ExtractOperationError {
    fn from(e: ParseIntError) -> Self {
        Self::InvalidInteger(e)
    }
}

impl Error for ExtractOperationError {}

impl Display for ExtractOperationError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::InvalidOperator => write!(f, "Invalid operator found."),
            Self::InvalidInteger(e) => write!(f, "Invalid integer: {}", e)
        }
    }
}

/**
 * RUST Practise Calculator
 * Written by: AVB
 * 
 * TODO: Figure out how to handle order of operations
 *          Find a data struct to do it
 *       For debug: parse comand line inputs
 *       Do the math on the inputs
 *          Write some tests for the above
 * 
 * Later (don't think about now):
 *      Undo button
 */

#[derive(Debug, PartialEq)]
enum Operation {
    Op(char),
    Number(i64)
}

fn main() {
    //get user input TODO later change to UI
    print!("> ");
    std::io::stdout().flush().unwrap();
    let user_in = get_user_in();
    println!("{:?}", user_in);
    //parse user input
    //print result

}

//Read and pass user input TODO only temp until UI is built
fn get_user_in() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    match stdin.read_line(&mut buffer) {
        Ok(_text_in_buffer) => {},
        Err(_empty_buffer) => {eprintln!("Error: {}", _empty_buffer)}
    }
    buffer.trim().to_string()
}

/**
 * Just reading a single operation for now
 * TODO:
 * Read and parse the ints of the user input
 * user_in -> user input TODO: later change to how the UI handles it
 * Returns:
 *  Vector of ints
 *  Error: ParseIntError
 */
fn parse_ints(user_in:&String) -> Result<[Operation;3], ExtractOperationError> {
    let mut equation:[Operation;3] = [Operation::Number(0), Operation::Op('+'), Operation::Number(0)];

    //Split on whitespace (don't allow unspaced things for now)
    let split_line = user_in.split_whitespace().collect::<Vec<&str>>();

    equation[0] = Operation::Number(split_line[0].parse::<i64>()?);
    equation[2] = Operation::Number(split_line[2].parse::<i64>()?);
    if is_valid_op(split_line[1]) {
        equation[1] = Operation::Op(split_line[1].chars().nth(0).unwrap());
    }else {
        return Err(ExtractOperationError::InvalidOperator)
    }
    /* Save for later use, currently not used
    for (i, int_or_op) in user_in.split_whitespace().enumerate() {
        match int_or_op.parse::<i64>() {
            Ok(value) => equation[i] = Operation::Number(value),
            Err(e) => { //TODO figure out what to do here (handle operator validation)
                if is_valid_op(int_or_op){
                    equation[i] = Operation::Op(int_or_op.chars().nth(0).unwrap())
                }else {
                    Err(e)?
                }
            }
        }
    } */

    //do the math, then return that result?
    return Ok(equation);
}

/**
 * Checks if given string is in a list of valid operators:
 *  - Valid operators are +,-,/,*, // These later (, or )
 * 
 * returns true if matches above, else false
 */
fn is_valid_op(check_this: &str) -> bool {
    if check_this.len() != 1 { return false }
    let operators = ['+','-','/','*'];//,'(',')'];

    if operators.iter().any(|&o| check_this.starts_with(o)) {
        return true;
    }
    false
}

fn do_math(_values: Vec<i64>, _operations: Vec<char>) {
    //TODO
}

fn add(a:i64, b:i64) -> i64 {
    a + b
}

fn sub(a:i64, b:i64) -> i64 {
    a - b
}

fn multi(a:i64, b:i64) -> i64 {
    a * b
}

fn div(a:i64, b:i64) -> Result<i64, &'static str> {
    if b == 0 {
        return Err("Division by zero is undefined")
    }
    Ok(a / b)
}

//TESTS
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basic_addition_input() {
        let expected: [Operation;3] = [Operation::Number(123), Operation::Op('+'), Operation::Number(456)];
        let input = "123 + 456".to_string();
        let actual = parse_ints(&input).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn basic_sub_input() {
        let expected: [Operation;3] = [Operation::Number(123), Operation::Op('-'), Operation::Number(456)];
        let input = "123 - 456".to_string();
        let actual = parse_ints(&input).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn basic_multi_input() {
        let expected: [Operation;3] = [Operation::Number(123), Operation::Op('*'), Operation::Number(456)];
        let input = "123 * 456".to_string();
        let actual = parse_ints(&input).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn basic_div_input() {
        let expected: [Operation;3] = [Operation::Number(123), Operation::Op('/'), Operation::Number(456)];
        let input = "123 / 456".to_string();
        let actual = parse_ints(&input).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn invalid_inputs_return_err() {
        let mut input = "x / 456".to_string();
        assert!(parse_ints(&input).is_err());
        input = "123 x 456".to_string();
        assert!(parse_ints(&input).is_err());
        input = "123 / x".to_string();
        assert!(parse_ints(&input).is_err());
    }

    #[test]
    fn basic_add(){
        assert_eq!(2, add(1,1));
        assert_eq!(4, add(2,2));
    }

    #[test]
    fn add_negative(){
        assert_eq!(-2, add(-1,-1));
        assert_eq!(-3, add(-5, 2));
    }

    #[test]
    fn add_limits() {
        let limit = std::i64::MAX;
        let min = std::i64::MIN;

        assert_eq!(limit, add(limit-1, 1));
        assert_eq!(min+1, add(min, 1));
        assert_eq!(-1, add(min, limit));
    }

    #[test]
    #[should_panic]
    fn add_overflow_check() {
        add(i64::MAX, 1);
    }

    #[test]
    #[should_panic]
    fn add_underflow_check() {
        add(i64::MIN, -1);
    }
    //END add tests
    //START sub tests
    #[test]
    fn basic_sub() {
        assert_eq!(0, sub(1, 1));
        assert_eq!(10, sub(20, 10));
    }

    #[test]
    fn sub_negative() {
        assert_eq!(0, sub(-1, -1));
        assert_eq!(-10, sub(-20, -10));
    }

    #[test]
    fn sub_limits() {
        assert_eq!(i64::MIN, sub(i64::MIN+1, 1));
        assert_eq!(i64::MAX, sub(i64::MAX-1, -1));
    }

    #[test]
    #[should_panic]
    fn sub_overflow() {
        sub(i64::MAX, -1);
    }

    #[test]
    #[should_panic]
    fn sub_underflow() {
        sub(i64::MIN, 1);
    }
    //END sub tests
    //START multi tests
    #[test]
    fn multi_by_one() {
        assert_eq!(1, multi(1, 1));
        assert_eq!(10, multi(10, 1));
        assert_eq!(100, multi(100, 1));
    }

    #[test]
    fn multi_by_zero() {
        assert_eq!(0, multi(1, 0));
        assert_eq!(0, multi(100, 0));
        assert_eq!(0, multi(2342342, 0));
    }

    #[test]
    fn basic_multi() {
        assert_eq!(4, multi(2, 2));
        assert_eq!(25, multi(5, 5));
        assert_eq!(32, multi(8, 4));
    }

    #[test]
    fn multi_negative() {
        assert_eq!(-1, multi(1, -1));
        assert_eq!(-1, multi(-1, 1));
        assert_eq!(-6, multi(3, -2));
        assert_eq!(-6, multi(-3, 2));
        assert_eq!(-100, multi(-50, 2));
        assert_eq!(-100, multi(50, -2));
    }

    #[test]
    fn multi_double_negative() {
        assert_eq!(1, multi(-1, -1));
        assert_eq!(9, multi(-3, -3));
        assert_eq!(100, multi(-50, -2));
    }

    #[test]
    fn multi_limit() {
        assert_eq!(i64::MAX, multi(i64::MAX/7, 7));
        assert_eq!(i64::MIN, multi(i64::MIN/2, 2));
    }

    #[test]
    #[should_panic]
    fn multi_overflow() {
        multi(i64::MAX/7, 8);
    }

    #[test]
    #[should_panic]
    fn multi_underflow() {
        multi(i64::MIN/2, 3);
    }
    //END multi tests
    //START div tests
    #[test]
    fn div_by_zero() {
        assert!(div(1, 0).is_err(), "Division by zero is undefined");
    }

    #[test]
    fn div_basic() {
        assert_eq!(0, div(0, 1).unwrap());
        assert_eq!(1, div(1, 1).unwrap());
        assert_eq!(2, div(4, 2).unwrap());
        assert_eq!(10, div(100, 10).unwrap());
    }

    fn div_negative() {
        assert_eq!(-1, div(1, -1).unwrap());
        assert_eq!(-1, div(-1, 1).unwrap());
        assert_eq!(-2, div(-4, 2).unwrap());
        assert_eq!(-2, div(4, -2).unwrap());
        assert_eq!(-10, div(-100, 10).unwrap());
        assert_eq!(-10, div(100, -10).unwrap());
    }

    fn div_double_negative() {
        assert_eq!(1, div(-1, -1).unwrap());
        assert_eq!(2, div(-4, -2).unwrap());
        assert_eq!(10, div(-100, -10).unwrap());
    }
    //END div tests
}
