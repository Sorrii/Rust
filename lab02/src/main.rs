fn add_chars_n(mut s: String, c: char, num: u32) -> String {
    for _ in 0..num {
        s.push(c);
    }

    s
}

fn add_chars_n_ref(s: &mut String, c: char, num: u32) -> () {
    for _ in 0..num {
        s.push(c);
    }
}

fn add_space(s: &mut String, num: u32) -> () {
    for _ in 0..num {
        s.push(' ');
    }
}

fn add_str(s: &mut String, str_to_add: &str) -> () {
    s.push_str(str_to_add);
}

fn add_integer(s: &mut String, num: i64) -> () {
    if num == 0 {
        s.push('0');
        return;
    }

    let mut num = num;
    if num < 0 {
        s.push('-');
        num = -num;
    }

    let mut div = 1;

    while num / div >= 10 {
        div *= 10;
    }

    let mut counter = 0;
    while div != 0 {
        if counter != 0 && counter % 3 == 0 {
            s.push('_');
        }

        let digit = (num / div) % 10;
        s.push((digit as u8 + b'0') as char);

        counter += 1;
        div /= 10;
    }
}

fn add_float(s: &mut String, num: f64) -> () {
    if num == 0.0 {
        s.push_str("0.0");
        return;
    }

    let mut num = num;
    if num < 0.0 {
        s.push('-');
        num = -num;
    }

    let integer_part = num.floor() as i64;
    add_integer(s, integer_part);
    s.push('.');

    let mut number_of_leading_zeros = 0;
    if num.fract() != 0.0 {
        let mut x: i32;
        let mut power: f64 = 10.0;
        loop {
            x = (num * power).floor() as i32;
            power *= 10.0;

            if x % 10 != 0 {
                break;
            }
            number_of_leading_zeros += 1;
        }
    }

    for _ in 0..number_of_leading_zeros {
        s.push('0');
    }

    let decimal_part = num.fract() * 1_000 as f64;
    add_integer(s, decimal_part as i64);
}

fn main() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        s = add_chars_n(s, c, 26 - i);

        i += 1;
    }
    println!("1. String modified with fn 'add_chars_n'\n");
    println!("{}\n", s);
    //p1 ^
    let mut s2 = String::new();
    i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        add_chars_n_ref(&mut s2, c, 26 - i);
        i += 1;
    }
    println!("2. String modified with fn 'add_chars_n_ref'\n");
    println!("{}", s2);
    //p2 ^
    println!("\n3. P3 -> ");
    print_desired_string();
    //p3 ^
}

fn print_desired_string() -> () {
    let green_heart = String::from("\x1b[32m\u{1F49A}\x1b[0m");
    let mut _string = String::new();

    let mut i_green_heart = String::from("I");
    i_green_heart += &green_heart;

    add_space(&mut _string, 43);
    add_str(&mut _string, &i_green_heart);
    add_str(&mut _string, "\n");
    add_space(&mut _string, 43);
    add_str(&mut _string, "RUST.");
    add_str(&mut _string, "\n");
    add_str(&mut _string, "\n");
    add_str(&mut _string, "\t");

    add_str(&mut _string, "Most");
    add_space(&mut _string, 12);
    add_str(&mut _string, "crate");
    add_space(&mut _string, 6);
    add_integer(&mut _string, 306437968);
    add_space(&mut _string, 12);
    add_str(&mut _string, "and");
    add_space(&mut _string, 6);
    add_str(&mut _string, "lastest");
    add_space(&mut _string, 10);
    add_str(&mut _string, "is");
    add_space(&mut _string, 7);
    add_str(&mut _string, "\n");
    add_str(&mut _string, "\t");
    
    add_space(&mut _string, 5);
    add_str(&mut _string, "downloaded");
    add_space(&mut _string, 8);
    add_str(&mut _string, "has");
    add_space(&mut _string, 13);
    add_str(&mut _string, "downloads");
    add_space(&mut _string, 6);
    add_str(&mut _string, "the");
    add_space(&mut _string, 11);
    add_str(&mut _string, "version");
    add_space(&mut _string, 4);
    add_float(&mut _string, 2.0381); // am folosit 2.0381 pentru ca daca puneam 2.038 afisa 2.037(9999999)
    add_str(&mut _string, ".");

    println!("{}", _string);
}
