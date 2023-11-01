use std::fs;
//use std::time::Instant;
enum MyErrors {
    CharacterIsNotAscii,
}

fn rot_13_c(c: char) -> char {
    match c {
        'A'..='Z' => return ((c as u8 - b'A' + 13) % 26 + b'A') as char,
        'a'..='z' => return ((c as u8 - b'a' + 13) % 26 + b'a') as char,
        ' ' => ' ',
        _ => return c,
    }
}

fn cipher() -> Result<String, MyErrors> {
    let string_to_cipher = fs::read_to_string("input.txt").expect("Failed to open input file!\n");
    let mut ciphered_string = String::new();

    for c in string_to_cipher.chars() {
        if c.is_ascii() {
            let l = rot_13_c(c);
            ciphered_string.push(l);
        } else {
            return Err(MyErrors::CharacterIsNotAscii);
        }
    }

    Ok(ciphered_string)
}

fn main() {
    //let start = Instant::now();
    match cipher() {
        Err(MyErrors::CharacterIsNotAscii) => println!("Error -> Not all characters were ASCII!"),
        Ok(result) => match fs::write("output.txt", &result) {
            Ok(_) => (),
            Err(_) => println!("Error printing in output file!"),
        },
    }

    //println!("Time elapsed: {:?}", start.elapsed());
}
