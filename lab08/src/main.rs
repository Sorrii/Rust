use std::collections::HashMap;
use std::fs;

fn problem() {
    let content = fs::read_to_string("input.txt").expect("Couldn't open the input file!\n");

    let mut word_count: HashMap<String, u32> = HashMap::new();

    for line in content.lines() {
        for word in line.to_lowercase().split_whitespace() {
            let cleaned: String = word.chars().filter(|c| c.is_alphanumeric()).collect();
            
            word_count
                .entry(cleaned)
                .and_modify(|count| *count = *count + 1).or_insert(1);
        }
    }
    
    let mut word_count_vec: Vec<(&String, &u32)> = word_count.iter().collect();
    word_count_vec.sort_by(|a, b| b.1.cmp(a.1));

    for (word, count) in word_count_vec {
        println!("{:<8} => {}", word, count);
    }
}

fn main() {
    problem();
}
