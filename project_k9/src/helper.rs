use rand::prelude::*;
use serenity::model::id::UserId;
use strsim::levenshtein;
use rusqlite::{Connection, Result};

pub fn gen_random_string(quotes: Vec<String>) -> Option<String> {
    let mut rng = rand::thread_rng();
    let pos: usize = rng.gen_range(0..quotes.len());

    quotes.get(pos).cloned()
}

pub fn close_call(user_answer: &str, actual_answer: &String) -> bool {
    let threshold = 2; 
    let distance = levenshtein(user_answer, actual_answer.as_str());

    distance > 0 && distance <= threshold

}

pub fn create_database() -> Result<()> {
    let conn = Connection::open("points.db")?;
    let create = r"
    create table if not exists points (
        discord_id INTEGER PRIMARY KEY,
        points INTEGER
);
";
    conn.execute(create, ())?;

    Ok(())
}

pub fn update_score(user_id: UserId, points: u8) -> Result<()> {
    let conn = Connection::open("points.db")?;
    conn.execute(
        "INSERT INTO users (discord_id, points) VALUES (?1, ?2)
         ON CONFLICT(discord_id) DO UPDATE SET points = points + ?2;",
        (user_id.get() as i64, points),
    )?;
    Ok(())
}