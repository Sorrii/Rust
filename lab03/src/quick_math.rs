use std::{io, num::ParseIntError};

#[derive(Debug)]
pub enum MyAppErrors {
    OverflowAdd,
    OverflowMul,
    InvalidOperator,
    InvalidEquation,
    DivByZero,
}

fn print_error(error: MyAppErrors) {
    match error {
        MyAppErrors::OverflowAdd => {
            println!("Adding the given numbers will result in an overflow!\n")
        }
        MyAppErrors::OverflowMul => {
            println!("Multiplying the given numbers will result in an overflow!\n")
        }
        MyAppErrors::InvalidOperator => {
            println!("Invalid operator was used -> try '+', '-', '*', '/', '%'\n")
        }
        MyAppErrors::InvalidEquation => println!("Invalid equation -> try a 'operator' b\n"),
        MyAppErrors::DivByZero => println!("You cannot divide by 0!\n"),
    }
}

fn check_eq(operands: &[&str]) -> Result<bool, MyAppErrors> {
    if operands.len() == 3 {
        Ok(true)
    } else {
        Err(MyAppErrors::InvalidEquation)
    }
}

fn is_number(num: &str) -> Result<i32, ParseIntError> {
    match num.parse::<i32>() {
        Ok(number) => Ok(number),
        Err(invalid_input) => Err(invalid_input),
    }
}

fn is_operator(op: &str) -> Result<bool, MyAppErrors> {
    if op == "+" || op == "-" || op == "*" || op == "/" || op == "%" {
        Ok(true)
    } else {
        Err(MyAppErrors::InvalidOperator)
    }
}

fn calculate(a: i32, b: i32, op: &str) -> Result<i32, MyAppErrors> {
    if op == "+" {
        match a.checked_add(b) {
            Some(result) => Ok(result),
            None => Err(MyAppErrors::OverflowAdd),
        }
    } else if op == "-" {
        match a.checked_add(b * (-1)) {
            Some(result) => Ok(result),
            None => Err(MyAppErrors::OverflowAdd),
        }
    } else if op == "/" {
        if b == 0 {
            Err(MyAppErrors::DivByZero)
        } else {
            Ok(a / b)
        }
    } else if op == "*" {
        match a.checked_mul(b) {
            Some(result) => Ok(result),
            None => Err(MyAppErrors::OverflowMul),
        }
    } else {
        if b == 0 {
            Err(MyAppErrors::DivByZero)
        } else {
            Ok(a % b)
        }
    }
}

pub fn quick_math() -> () {
    println!("Welcome to quick math!\n");
    println!("Accepted equations: a + b, a - b, a * b, a / b, a % b");
    println!("Write down a simple math equation: ");
    let mut number1: i32 = 0;
    let mut number2: i32 = 0;
    let mut op: &str = "";
    let mut flag = 0;
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let operands: Vec<&str> = input.split_whitespace().collect();

    match check_eq(&operands) {
        Ok(true) => flag += 1,
        Ok(false) => {}
        Err(error) => print_error(error),
    }

    match operands.get(0) {
        Some(&operand) => match is_number(operand) {
            Ok(number) => {
                number1 = number;
                flag += 1;
            }
            Err(_error) => println!("Invalid input numbers!\n"),
        },
        None => println!("No operand was provided!\n"),
    }

    match operands.get(2) {
        Some(&operand) => match is_number(operand) {
            Ok(number) => {
                number2 = number;
                flag += 1;
            }
            Err(_error) => println!("Invalid input numbers!\n"),
        },
        None => println!("No operand was provided!\n"),
    }

    match operands.get(1) {
        Some(&operand) => match is_operator(operand) {
            Ok(true) => {
                op = operand;
                flag += 1;
            }
            Ok(false) => {}
            Err(error) => print_error(error),
        },
        None => println!("No operator was provided!\n"),
    }
    if flag == 4 {
        match calculate(number1, number2, op) {
            Ok(result) => print!("= {}", result),
            Err(error) => print_error(error),
        }
    } else {
        println!("Failed to calculate!\n");
    }
}
