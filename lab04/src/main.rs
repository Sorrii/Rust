use std::time::Instant;
use std::{fs, io};

enum MyError {
    NonAsciiCharacter(char),
    NoAbbreviations,
}

fn p1() -> Result<(), io::Error> {
    let content = fs::read_to_string("src/input_for_p1")?;

    let mut max_bytes: i32 = -1;
    let mut max_chars: i32 = -1;
    let mut max_bytes_string = String::new();
    let mut max_chars_string = String::new();

    for line in content.lines() {
        if line.len() as i32 > max_bytes {
            max_bytes = line.len() as i32;
            max_bytes_string = String::from(line);
        }

        if line.chars().count() as i32 > max_chars {
            max_chars = line.chars().count() as i32;
            max_chars_string = String::from(line);
        }
    }

    println!(
        "Bytes -> The maximum sized string is {} and has {} bytes!",
        max_bytes_string, max_bytes
    );
    println!(
        "Characters -> The maximum sized string is {} and has {} characters!",
        max_chars_string, max_chars
    );

    Ok(())
}

fn p2(mut string_to_cipher: &mut String) -> Result<String, MyError> {
    println!("String before chiphering: {} ", string_to_cipher);
    for c in string_to_cipher.chars() {
        if !c.is_ascii_alphabetic() && c != ' ' {
            return Err(MyError::NonAsciiCharacter(c));
        }
    }
    let ciphered_string = rot_13(&mut string_to_cipher);
    Ok(ciphered_string)
}

fn p3() -> Result<(), MyError> {
    let abb: [(&str, &str); 4] = [
        ("dl", "domnul"),
        ("dna", "doamna"),
        ("pt", "pentru"),
        ("ptr", "pentru"),
    ];

    let content =
        fs::read_to_string("src/input_for_p3").expect("Couldn't open input file for p3!\n");
    println!("String before modifying abbreviations: {}", content);
    let words: Vec<&str> = content.split_whitespace().collect();
    let mut result = String::new();
    let mut replaced = false;

    for word in words {
        let mut found = false;
        for &(short, long) in &abb {
            if short == word {
                result.push_str(&long);
                result.push(' ');

                found = true;
                replaced = true;

                break;
            }
        }

        if !found {
            result.push_str(&word);
            result.push(' ');
        }
    }

    if replaced {
        match fs::write("src/input_for_p3", &result) {
            Ok(_) => println!("Abbreviations were changed!"),
            Err(_) => println!("Error rewriting in file!\n"),
        }

        Ok(())
    } else {
        Err(MyError::NoAbbreviations)
    }
}

fn p4() -> Result<(), io::Error> {
    let content = fs::read_to_string("src/input_for_p4")?; //am folosit un fisier facut de mine pentru 
    //println!("{}", content);                                             ca toate liniile erau comentate
    for line in content.lines() {
        if !line.trim().is_empty() && !line.trim().starts_with('#') {
            let host_and_ip: Vec<&str> = line.split_whitespace().collect();

            if host_and_ip.len() >= 2 {
                let host = host_and_ip[0];
                let ip = host_and_ip[1];

                println!("{} => {}", host, ip);
            }
        }
    }

    Ok(())
}

fn do_p2() -> Result<String, MyError> {
    println!("The more efficient method: ");
    let string_to_cipher =
        fs::read_to_string("src/input_for_bonus").expect("Couldn't open input file for p3!\n");

    for c in string_to_cipher.chars() {
        if !c.is_ascii_alphabetic() && c != ' ' {
            return Err(MyError::NonAsciiCharacter(c));
        }
    }

    let mut ciphered_string = String::new();

    for c in string_to_cipher.chars() {
        let rot13_char = match c {
            'A'..='Z' => ((c as u8 - b'A' + 13) % 26 + b'A') as char,
            'a'..='z' => ((c as u8 - b'a' + 13) % 26 + b'a') as char,
            ' ' => ' ',
            _ => c,
        };

        ciphered_string.push(rot13_char);
    }

    Ok(ciphered_string)
}

fn do_p2_unefficiently() -> Result<String, MyError> {
    println!("The less efficient method: ");

    let lowercase_letters: Vec<char> = ('a'..'z').collect();
    let uppercase_letters: Vec<char> = ('A'..'Z').collect();

    let string_to_cipher =
        fs::read_to_string("src/input_for_bonus").expect("Couldn't open input file for p3!\n");

    for c in string_to_cipher.chars() {
        if !c.is_ascii_alphabetic() && c != ' ' {
            return Err(MyError::NonAsciiCharacter(c));
        }
    }

    let mut ciphered_string = String::new();
    for c in string_to_cipher.chars() {
        let rot13_char = match c {
            'a'..='z' => lowercase_letters[(c as usize - 'a' as usize + 13) % 26],
            'A'..='Z' => uppercase_letters[(c as usize - 'A' as usize + 13) % 26],
            ' ' => ' ',
            _ => c,
        };
        ciphered_string.push(rot13_char);
    }

    Ok(ciphered_string)
}

fn bonus() -> () {
    let start1 = Instant::now();
    match do_p2_unefficiently() {
        Ok(_result) => (),
        Err(MyError::NonAsciiCharacter(_c)) => {
            println!("Not all characters were ascii alphabetic!")
        }
        _ => (),
    }

    println!("Time elapsed: {:?}", start1.elapsed());

    let start2 = Instant::now();
    match do_p2() { 
        Ok(_result) => (),
        Err(MyError::NonAsciiCharacter(_c)) => {
            println!("Not all characters were ascii alphabetic!")
        }
        _ => (),
    }

    println!("Time elapsed: {:?}", start2.elapsed());
}

fn main() {
    println!("Exercise 1: ");
    let _ = p1();

    println!("\nExercise 2: ");
    let mut string_to_test = String::from("Hello World");
    match p2(&mut string_to_test) {
        Err(MyError::NonAsciiCharacter(c)) => {
            println!(
                "The string has other characters besides ascii alphabetic -> '{}'",
                c
            )
        }
        Ok(ciphered_string) => {
            println!("String after rot_13 cipher {}", ciphered_string);
        }
        _ => (),
    }

    println!("\nExercise 3: ");
    match p3() {
        Err(MyError::NoAbbreviations) => println!("There was nothing to change!"),
        Ok(()) => {
            let content =
                fs::read_to_string("src/input_for_p3").expect("Couldn't open input file for p3!\n");
            println!("String after modifying abbreviations: {}", content)
        }
        _ => (),
    }

    println!("\nExercise 4: ");
    let _ = p4();

    println!("\nBonus exercise: ");
    bonus();
}

fn rot_13(s: &mut String) -> String {
    let mut result = String::new();

    for c in s.chars() {
        let rot13_char = match c {
            'A'..='Z' => ((c as u8 - b'A' + 13) % 26 + b'A') as char,
            'a'..='z' => ((c as u8 - b'a' + 13) % 26 + b'a') as char,
            ' ' => ' ',
            _ => c,
        };

        result.push(rot13_char);
    }

    result
}
