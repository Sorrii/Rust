mod enums;
mod quick_math;

use enums::CharacterErrors;
use enums::MyError;

fn next_prime(x: u16) -> Option<u16> {
    let mut copy: u16 = match x.checked_add(1) {
        Some(value) => value,
        None => return None,
    };

    while !is_prime(copy) {
        copy = match copy.checked_add(1) {
            Some(value) => value,
            None => return None,
        }
    }

    Some(copy)
}

fn checked_addition_u32(a: u32, b: u32) -> u32 {
    match a.checked_add(b) {
        Some(result) => result,
        None => panic!("Adding {} to {} will result in an overflow!", a, b),
    }
}

fn checked_multiplication_u32(a: u32, b: u32) -> u32 {
    match a.checked_mul(b) {
        Some(result) => result,
        None => panic!("Multiplying {} by {} will result in an overflow!", a, b),
    }
}

fn checked_addition_u32_result(a: u32, b: u32) -> Result<u32, MyError> {
    match a.checked_add(b) {
        Some(result) => Ok(result),
        None => Err(MyError::Overflow),
    }
}

fn checked_multiplication_u32_result(a: u32, b: u32) -> Result<u32, MyError> {
    match a.checked_mul(b) {
        Some(result) => Ok(result),
        None => Err(MyError::Overflow),
    }
}

fn main() {
    println!("Exercise 1.");
    let mut number: u16 = 65_003; //started from 65_003 for readability.

    loop {
        match next_prime(number) {
            Some(prime) => {
                println!("Next prime number after {} is {}", number, prime);
                number = prime;
            }
            None => {
                println!("Error! Next prime number is out of u16 bounds (> 65535)!\n");
                break;
            }
        }
    }

    println!("Exercise 2.");
    println!("For numbers {} and {} the result is: ", 25, 174);
    print!(" -Checked addition -> {}\n", checked_addition_u32(25, 174));
    print!(
        " -Checked multiplication -> {}\n",
        checked_multiplication_u32(25, 174)
    );

    //println!("For numbers {} and {} the result is: ", 2_147_483_647, 2_147_483_647);
    //print!(" -Checked addition -> {}\n", checked_addition_u32(2_147_483_647, 2_147_483_647));
    //print!(" -Checked multiplication -> {}\n", checked_multiplication_u32(2_147_483_647, 2_147_483_647));
    println!("Don't want the program to panic - added comments to that part of the exercise\n");

    println!("Exercise 3.");
    println!("For numbers {} and {} the result is: ", 25, 174);
    print!(
        " -Checked addition -> {:?}\n",
        checked_addition_u32_result(25, 174)
    );
    print!(
        " -Checked multiplication -> {:?}\n",
        checked_multiplication_u32_result(25, 174)
    );

    println!(
        "For numbers {} and {} the result is: ",
        2_147_483_647, 2_147_483_647
    );
    print!(
        " -Checked addition -> {:?}\n",
        checked_addition_u32_result(2_147_483_647, 2_147_483_647)
    );
    print!(
        " -Checked multiplication -> {:?}\n",
        checked_multiplication_u32_result(2_147_483_647, 2_147_483_647)
    );
    println!("\nExercise 4.\n");
    test_ex4();

    println!("\nExercise 5.\n");

    quick_math::quick_math();
}

fn is_prime(num: u16) -> bool {
    if num <= 1 {
        return false;
    } else if num % 2 == 0 && num != 2 {
        return false;
    } else {
        for i in 3..(num / 2 + 1) {
            if num % i == 0 {
                return false;
            }
        }
    }

    true
}

fn to_uppercase(c: char) -> Result<char, CharacterErrors> {
    match c.is_ascii_alphabetic() {
        true => {
            let uppercase_c = c.to_ascii_uppercase();
            Ok(uppercase_c)
        }
        false => Err(CharacterErrors::CharIsNotALetter(c)),
    }
}

fn to_lowercase(c: char) -> Result<char, CharacterErrors> {
    match c.is_ascii_alphabetic() {
        true => {
            let lowercase_c = c.to_ascii_lowercase();
            Ok(lowercase_c)
        }
        false => Err(CharacterErrors::CharIsNotALetter(c)),
    }
}

fn print_char(c: char) -> Result<(), CharacterErrors> {
    match c.is_ascii_graphic() {
        true => {
            println!("The character is: '{}'\n", c);
            Ok(())
        }
        false => Err(CharacterErrors::CharIsNotPrintable(c)),
    }
}

fn char_to_number(c: char) -> Result<u8, CharacterErrors> {
    if c.is_ascii() && c.is_alphanumeric() {
        match c.to_digit(10) {
            Some(digit) => Ok(digit as u8),
            None => Err(CharacterErrors::CharIsNotADigit(c)),
        }
    } else if c.is_ascii() && !c.is_alphanumeric() {
        Err(CharacterErrors::CharIsNotADigit(c))
    } else {
        Err(CharacterErrors::NotAsciiChar(c))
    }
}

fn char_to_number_hex(c: char) -> Result<u8, CharacterErrors> {
    if c.is_ascii() {
        if let Some(digit) = c.to_digit(16) {
            if digit <= 0xF {
                Ok(digit as u8)
            } else if digit > 0xF {
                Err(CharacterErrors::CharIsNotADigit(c))
            } else {
                Err(CharacterErrors::CharIsNotABase16Digit(c))
            }
        } else {
            Err(CharacterErrors::CharIsNotABase16Digit(c))
        }
    } else {
        Err(CharacterErrors::NotAsciiChar(c))
    }
}

fn print_error(error: CharacterErrors) -> () {
    match error {
        CharacterErrors::NotAsciiChar(c) => println!("Error: '{}' is not an ASCII character", c),
        CharacterErrors::CharIsNotADigit(c) => println!("Error: '{}' is not a digit", c),
        CharacterErrors::CharIsNotABase16Digit(c) => {
            println!("Error: '{}' is not a base 16 number", c)
        }
        CharacterErrors::CharIsNotALetter(c) => println!("Error: '{}' is not a letter", c),
        CharacterErrors::CharIsNotPrintable(c) => {
            println!("Error: {:?} is not a printable character", c)
        }
    }
}

fn test_ex4() -> () {
    let mut c = 'A';

    for _ in 0..2 {
        match print_char(c) {
            Ok(_char) => (),
            Err(error) => print_error(error),
        }

        match to_lowercase(c) {
            Ok(lowercase_c) => {
                c = lowercase_c;
                println!("Letter {} is now lowercase!", c);
            }
            Err(error) => print_error(error),
        }
        
        match to_uppercase(c) {
            Ok(uppercase_c) => {
                c = uppercase_c;
                println!("Letter {} is now uppercase!", c);
            }
            Err(error) => print_error(error),
        }
        
        match char_to_number(c) {
            Ok(number) => println!("Number in base 10 is {}", number),
            Err(error) => print_error(error),
        }

        match char_to_number_hex(c) {
            Ok(number) => println!("Number from hex to base 10 is {}", number),
            Err(error) => print_error(error),
        }

        println!("------------------------------");

        c = '2';
    }

    let non_printable_char = '\n';
    println!("For newline character ->");
    match print_char(non_printable_char) {
        Ok(_char) => (),
        Err(error) => print_error(error),
    }

    println!("------------------------------");

    let non_hex_number = 'G';
    println!("For non hex number ->");
    match char_to_number_hex(non_hex_number) {
        Ok(number) => println!("Number from hex to base 10 is {}", number),
        Err(error) => print_error(error),
    }
}
