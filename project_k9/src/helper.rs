use rand::prelude::*;

pub fn get_random_quote(quotes: Vec<String>) -> Option<String> {
    let mut rng = rand::thread_rng();
    let pos: usize = rng.gen_range(0..quotes.len());

    quotes.get(pos).cloned()
}
