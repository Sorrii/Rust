use rand::prelude::*;
use rusqlite::{params, Connection, Result};
use serenity::model::id::UserId;
use strsim::levenshtein;

pub fn gen_random_string(quotes: Vec<String>) -> Option<String> {
    let mut rng = rand::thread_rng();
    let pos: usize = rng.gen_range(0..quotes.len());

    quotes.get(pos).cloned()
}

pub fn close_call(user_answer: &str, actual_answer: &str) -> bool {
    let threshold = 2;
    let distance = levenshtein(user_answer, actual_answer);

    distance > 0 && distance <= threshold
}

pub fn create_database() -> Result<()> {
    let conn = Connection::open("scoreboard.db")?;
    let create = r"
    create table if not exists scoreboard (
        discord_id INTEGER PRIMARY KEY,
        points INTEGER
);
";
    conn.execute(create, ())?;

    Ok(())
}

pub fn update_score(user_id: UserId, points: u8) -> Result<()> {
    let conn = Connection::open("scoreboard.db")?;
    conn.execute(
        "INSERT INTO scoreboard (discord_id, points) VALUES (?1, ?2)
         ON CONFLICT(discord_id) DO UPDATE SET points = points + ?2;",
        (user_id.get() as i64, points),
    )?;
    Ok(())
}

pub fn get_user_points(user_id: UserId) -> Result<i32> {
    let conn = Connection::open("scoreboard.db")?;

    let points: i32 = conn
        .query_row(
            "SELECT points FROM scoreboard WHERE discord_id = ?1",
            params![user_id.get() as i64],
            |row| row.get(0),
        )
        .unwrap_or(0);

    Ok(points)
}

pub fn get_top_users() -> Result<Vec<(i64, i32)>> {
    let conn = Connection::open("scoreboard.db")?;
    let mut stmt =
        conn.prepare("SELECT discord_id, points FROM scoreboard ORDER BY points DESC LIMIT 3")?;
    let user_iter = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;

    let mut users = Vec::new();
    for user in user_iter {
        users.push(user?);
    }

    Ok(users)
}
