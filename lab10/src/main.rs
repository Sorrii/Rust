use std::cell::RefCell;
use std::io;

struct Cache {
    data: RefCell<Vec<(i32, bool)>>,
}

impl Cache {
    fn new() -> Cache {
        Cache {
            data: RefCell::new(Vec::new()),
        }
    }

    fn check_cache(&self, number: i32) -> Option<bool> {
        for (num, _is_prime) in self.data.borrow().iter() {
            if number == *num {
                return Some(true);
            }
        }

        None
    }

    fn insert_number(&mut self, number: i32, is_prime: bool) {
        let mut data = self.data.borrow_mut();

        if data.len() >= 10 {
            data.remove(0);
        }

        data.push((number, is_prime));
    }
}

fn is_prime(number: i32) -> bool {
    if number < 2 {
        return false;
    }

    for i in 2..=((number as f64).sqrt() as i32) {
        if number % i == 0 {
            return false;
        }
    }

    true
}
fn main() {
    let mut cache = Cache::new();

    loop {
        let mut input = String::new();

        println!("Please insert a number: ");
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<i32>() {
            Ok(number) => match cache.check_cache(number) {
                Some(is_prime) => {
                    println!(
                        "Found in cache: {} is {}",
                        number,
                        if is_prime { "prime" } else { "not prime" }
                    );
                }
                None => {
                    let is_prime = is_prime(number);
                    println!(
                        "{} is {}",
                        number,
                        if is_prime { "prime" } else { "not prime" }
                    );
                    cache.insert_number(number, is_prime);
                }
            },
            Err(_) => {
                println!("Please insert a valid number!");
            }
        }

        println!();
    }
}
