use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use clap::{App, Arg};

#[cfg(target_os = "windows")]
const OS: &str = "Windows";
#[cfg(target_os = "linux")]
const OS: &str = "Linux";
#[cfg(target_os = "macos")]
const OS: &str = "Mac";

fn encode_file(input_path: &str, output_path: &str) -> Result<(), io::Error> {
    let mut input_file = File::open(input_path)?;
    let mut input_data = Vec::new();
    input_file.read_to_end(&mut input_data)?;

    let encoded_data = base64::encode(&input_data);

    let mut output_file = File::create(output_path)?;
    output_file.write_all(encoded_data.as_bytes())?;

    println!("Encoding completed successfully.");

    Ok(())
}

fn main() {
    println!("encoder, version {}, built for {}", env!("CARGO_PKG_VERSION"), OS);

    let matches = App::new("encoder")
    .version(env!("CARGO_PKG_VERSION"))
    .author("Your Name")
    .about("Encodes data in Base64")
    .arg(Arg::new("input")
        .short('i')
        .long("input")
        .value_name("FILE")
        .about("Sets the input file for encoding")
        .takes_value(true))
    .arg(Arg::new("output")
        .short('o')
        .long("output")
        .value_name("FILE")
        .about("Sets the output file for encoding")
        .takes_value(true))
    .get_matches();

    if let (Some(input_path), Some(output_path)) = (matches.value_of("input"), matches.value_of("output")) {
        if let Err(err) = encode_file(input_path, output_path) {
            eprintln!("Error: {}", err);
        }
    } else {
        let mut input = String::new();
        if let Err(err) = io::stdin().read_line(&mut input) {
            eprintln!("Error reading from stdin: {}", err);
        } else {
            let encoded = base64::encode(input.trim().as_bytes());
            println!("{}", encoded);
        }
    }
}