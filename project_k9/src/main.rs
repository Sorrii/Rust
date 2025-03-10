mod helper;
mod pictures;
use csv::ReaderBuilder;
use helper::{close_call, create_database, get_top_users, get_user_points, update_score};
use serenity::{
    all::ChannelId,
    async_trait,
    builder::{CreateAttachment, CreateMessage},
    framework::standard::{
        macros::{command, group},
        Args, {CommandResult, Configuration, StandardFramework},
    },
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use std::{env, sync::Arc};
use tokio::sync::Mutex;
use tokio::time::sleep;

#[group]
#[commands(ping, quote, help, doctor, answer, points, top, episode, who)]
struct General;

struct Handler;

#[derive(Debug)]
struct Episode {
    title: String,
    season: String,
    episode_number: String,
    air_date: String,
    runtime: String,
    rating: String,
    appreciation_index: String,
}

impl Episode {
    fn receive_data(data: csv::StringRecord) -> Self {
        Episode {
            title: data[0].to_string(),
            season: data[1].to_string(),
            episode_number: data[2].to_string(),
            air_date: data[3].to_string(),
            runtime: data[4].to_string(),
            rating: data[5].to_string(),
            appreciation_index: data[6].to_string(),
        }
    }
}

struct Quiz {
    questions: Vec<(String, String)>,
    current_question_index: Option<usize>,
    index: usize,
    current_question: String,
    answered: bool,
}

impl Quiz {
    fn new() -> Self {
        let questions = vec![
            (
                String::from("Who is the first Doctor? -> use >answer to answer the question @everyone"),
                String::from("William Hartnell"),
            ),
            (
                String::from("What is the Doctor's home planet? -> use >answer to answer the question @everyone"),
                String::from("Gallifrey"),
            ),
            (
                String::from("Who are the Doctor's main enemies, known for their catchphrase 'Exterminate' -> use >answer to answer the question? @everyone"),
                String::from("Daleks"),
            ),
            (
                String::from("What is the name of the Doctor's time machine? -> use >answer to answer the question @everyone"),
                String::from("TARDIS"),
            ),
            (
                String::from("Who is the creator of the Daleks? -> use >answer to answer the question @everyone"),
                String::from("Davros"),
            ),
            (
                String::from("Which actor played the Tenth Doctor? -> use >answer to answer the question @everyone"),
                String::from("David Tennant"),
            ),
            (
                String::from("What species is the Master? -> use >answer to answer the question @everyone"),
                String::from("Time Lord"),
            ),
            (
                String::from("What is the name of the Doctor's robotic dog? -> use >answer to answer the question @everyone"),
                String::from("K-9"),
            ),
            (
                String::from("Who played the Fourth Doctor? -> use >answer to answer the question @everyone"),
                String::from("Tom Baker"),
            ),
            (
                String::from("What does TARDIS stand for? -> use >answer to answer the question @everyone"),
                String::from("Time And Relative Dimension In Space"),
            ),
            (
                String::from("Which Doctor Who story was the first to be broadcast in color? -> use >answer to answer the question @everyone"),
                String::from("Spearhead from Space"),
            ),
            (
                String::from("Which companion traveled with the Ninth and Tenth Doctors? -> use >answer to answer the question @everyone"),
                String::from("Rose Tyler"),
            ),
            (
                String::from("Who was the first female Doctor? -> use >answer to answer the question @everyone"),
                String::from("Jodie Whittaker"),
            ),
            (
                String::from("What is the signature tool used by the Doctor? -> use >answer to answer the question @everyone"),
                String::from("Sonic Screwdriver"),
            ),
            (
                String::from("What alien race looks like humanoid rhinos? -> use >answer to answer the question @everyone"),
                String::from("Judoon"),
            ),
            (
                String::from("What is the name of the Doctor's granddaughter? -> use >answer to answer the question @everyone"),
                String::from("Susan Foreman"),
            ),
            (
                String::from("Which Doctor had a distinctive long scarf? -> use >answer to answer the question @everyone"),
                String::from("The Fourth Doctor (Tom Baker)"),
            ),
            (
                String::from("Which actor played the Ninth Doctor? -> use >answer to answer the question @everyone"),
                String::from("Christopher Eccleston"),
            ),
            (
                String::from("Who is River Song to the Doctor? -> use >answer to answer the question @everyone"),
                String::from("His wife"),
            ),
            (
                String::from("What planet are the Sontarans from? -> use >answer to answer the question @everyone"),
                String::from("Sontar"),
            ),
            (
                String::from("Which race created the Reality Bomb? -> use >answer to answer the question @everyone"),
                String::from("Daleks"),
            ),
            (
                String::from("What is the name of the Doctor Who spin-off featuring a team of alien hunters? -> use >answer to answer the question @everyone"),
                String::from("Torchwood"),
            ),
            (
                String::from("Who is the 'impossible girl'? -> use >answer to answer the question @everyone"),
                String::from("Clara Oswald"),
            ),
            (
                String::from("What species is Madame Vastra? -> use >answer to answer the question @everyone"),
                String::from("Silurian"),
            ),
            (
                String::from("What does UNIT stand for in Doctor Who? -> use >answer to answer the question @everyone"),
                String::from("Unified Intelligence Taskforce"),
            ),
        ];

        Quiz {
            questions,
            current_question_index: None,
            index: 0,
            current_question: String::new(),
            answered: true,
        }
    }

    fn generate_question(&mut self) {
        if let Some((question, _answer)) = self.questions.get(self.index % self.questions.len()) {
            self.current_question = question.to_string();
            self.answered = false;
            self.current_question_index = Some(self.index);
            self.index += 1;
        }
    }

    fn check_answer(&mut self, user_answer: &str) -> bool {
        if let Some(index) = self.current_question_index {
            if let Some((_question, expected_answer)) = self.questions.get(index) {
                if user_answer
                    .to_lowercase()
                    .contains(&expected_answer.to_lowercase())
                {
                    return true;
                }
            }
        }
        false
    }
}

lazy_static::lazy_static! {
    static ref QUIZ: Arc<Mutex<Quiz>> = Arc::new(Mutex::new(Quiz::new()));
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let available_commands: String =
            String::from("Available commands: \n>quote\n>ping\n>doctor <number>\n>answer (to answer the quiz question)\n>points\n>top\n>episode\n>who");
        println!("{}", available_commands);

        let _ = create_database();

        tokio::spawn(post_question(ctx.clone(), 1186232393984643072.into()));
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new().group(&GENERAL_GROUP);
    framework.configure(Configuration::new().prefix(">"));

    let token = env::var("DISCORD_TOKEN").expect("Expected a token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client!");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let available_commands: String =
        String::from("Available commands: \n>quote\n>ping\n>doctor <number>\n>answer (to answer the quiz question)\n>points\n>top\n>episode <keyword>\n>who");

    msg.reply(ctx, available_commands).await?;

    Ok(())
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn quote(ctx: &Context, msg: &Message) -> CommandResult {
    use helper::gen_random_string;

    let strings = vec![
        String::from("Whole worlds pivot on acts of imagination.\n \t - 13th Doctor"),
        String::from("Letting it get to you. You know what that’s called? Being alive. Best thing there is. Being alive right now is all that counts.\n \t - 11th Doctor"),
        String::from("There’s no point in being grown up if you can’t be childish sometimes.\n \t - 4th Doctor"),
        String::from("When did you last have the pleasure of smelling a flower, watching a sunset, eating a well-prepared meal?\n \t - 5th Doctor"),
        String::from("Time travel is like visiting Paris. You can't just read the guidebook. You've got to throw yourself in! Eat the food, use the wrong verbs, get charged double and end up kissing complete strangers! Or is that just me?\n \t - 9th Doctor"),
        String::from("Lives change worlds. People can save planets or wreck them. That’s the choice. Be the best of humanity.\n \t - 13th Doctor"),
        String::from("Life depends on change, and renewal.\n \t - 2nd Doctor"),
        String::from("Some people live more in 20 years than others do in 80. It’s not the time that matters, it’s the person.\n \t - 10th Doctor"),
        String::from("Nothing’s sad until it’s over, and then everything is.\n \t - 12th Doctor"),
        String::from("Everything’s got to end sometime. Otherwise, nothing would ever get started.\n \t - 11th Doctor"),
        String::from("You want weapons? We’re in a library! The best weapons in the world!\n \t - 10th Doctor"),
        String::from("We’re all stories, in the end. Just make it a good one, eh?\n \t - 11th Doctor"),
        String::from("You can always judge a man by the quality of his enemies.\n \t - 7th Doctor"),
        String::from("There must be no regrets, no tears, no anxieties. Just go forward in all your beliefs, and prove to me that I am not mistaken in mine.\n \t - 1st Doctor"),
        String::from("Bowties are cool!\n \t - 11th Doctor")
    ];

    if let Some(random_string) = gen_random_string(strings) {
        msg.reply(ctx, random_string).await?;
    } else {
        msg.reply(ctx, "No quotes available!").await?;
    }

    Ok(())
}

#[command]
async fn doctor(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        msg.reply(ctx, "Usage: >doctor 1 <= number <= 13").await?;
        return Ok(());
    }

    match args.single::<usize>() {
        Ok(index) if (1..=13).contains(&index) => {
            let image_path = match pictures::get_image_path(index) {
                Some(path) => path,
                None => {
                    msg.reply(ctx, "Incorret path to image").await?;
                    return Ok(());
                }
            };
        
            if !std::path::Path::new(&image_path).exists() {
                msg.reply(ctx, "File not found").await?;
                return Ok(());
            }
        
            let builder = CreateMessage::new()
                .content("")
                .add_file(CreateAttachment::path(image_path).await?);
        
            let msg = msg.channel_id.send_message(&ctx.http, builder).await;
            if let Err(why) = msg {
                println!("Error sending message: {:?}", why);
            };
        },
        Ok(_) => {
            msg.reply(ctx, "Invalid index! Try a number between 1 and 13").await?;
        }
        Err(_) => {
            msg.reply(ctx, "Invalid input! This command only works with numbers").await?;
        }
    }


    Ok(())
}

#[command]
async fn answer(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        msg.reply(ctx, "Usage: >answer <your answer>").await?;
        return Ok(());
    }
    let user_answer = args.rest();

    let mut quiz = QUIZ.lock().await;
    if quiz.check_answer(user_answer) {
        drop(quiz);

        let _ = update_score(msg.author.clone().into(), 1);
        msg.reply(ctx, "Correct answer! You got a point").await?;

        let mut quiz = QUIZ.lock().await;
        quiz.answered = true;
    } else if let Some(index) = quiz.current_question_index {
        if let Some((_question, actual_answer)) = quiz.questions.get(index) {
            if close_call(user_answer, actual_answer) {
                msg.reply(ctx, "Incorrect answer! You are close though! Try again!")
                    .await?;
            } else {
                msg.reply(ctx, "Incorrect answer! Try again!").await?;
            }
        }
    }

    Ok(())
}

#[command]
async fn points(ctx: &Context, msg: &Message) -> CommandResult {
    match get_user_points(msg.author.clone().into()) {
        Ok(points) => {
            let reply_message = format!("You have {} points.", points);
            msg.reply(ctx, &reply_message).await?;
        }
        Err(why) => {
            println!("Error retrieving data: {:?}", why);
        }
    }
    Ok(())
}

#[command]
async fn top(ctx: &Context, msg: &Message) -> CommandResult {
    match get_top_users() {
        Ok(users) => {
            let mut response = String::new();
            for (index, (user_id, points)) in users.iter().enumerate() {
                response.push_str(&format!(
                    "{}. <@{}> - {} points\n",
                    index + 1,
                    user_id,
                    points
                ));
            }
            msg.reply(ctx, response).await?;
        }
        Err(why) => {
            println!("Error showing the top users: {:?}", why);
        }
    }
    Ok(())
}

#[command]
async fn episode(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        msg.reply(ctx, "Usage: >episode <text to search>").await?;
        return Ok(());
    }
    let arg = args.rest();

    let csv_path = "src/Doctor_Who_CSV.csv";

    if let Ok(mut reader) = ReaderBuilder::new().has_headers(true).from_path(csv_path) {
        let mut matching_episodes: Vec<Episode> = Vec::new();

        for result in reader.records() {
            match result {
                Ok(record) => {
                    let episode = Episode::receive_data(record);

                    if episode.title.to_lowercase().contains(&arg.to_lowercase()) {
                        matching_episodes.push(episode);
                    }
                }
                Err(why) => {
                    println!("Error reading CSV file: {}", why);
                }
            }
        }
        if !matching_episodes.is_empty() {
            let mut response = String::new();

            for episode in &matching_episodes {
                let info = format!(
                    "\nTitle: {}\nSeason: {}\nEpisode: {}\nRuntime: {}\nAir Date: {}\nRating: {}\nAI: {}\n",
                    episode.title,
                    episode.season,
                    episode.episode_number,
                    episode.runtime,
                    episode.air_date,
                    episode.rating,
                    episode.appreciation_index
                );
                response.push_str(&info);
            }

            let path = "matching_episodes.txt";
            let mut temp_file = File::create(path)?;

            temp_file.write_all(response.as_bytes())?;

            let builder = CreateMessage::new()
                .content("")
                .add_file(CreateAttachment::path(path).await?);

            let msg = msg.channel_id.send_message(&ctx.http, builder).await;
            if let Err(why) = msg {
                println!("Error sending message: {:?}", why);
            };

            std::fs::remove_file(path)?;
        } else {
            msg.reply(ctx, "No matching episodes were found!").await?;
        }
    }

    Ok(())
}

#[command]
async fn who(ctx: &Context, msg: &Message) -> CommandResult {
    let username = &msg.author.name;
    let response = format!("Usually Doctor Who, but you are {}!", username);
    msg.reply(ctx, response).await?;
    Ok(())
}

async fn post_question(ctx: Context, channel_id: ChannelId) {
    loop {
        sleep(Duration::from_secs(10)).await;

        let mut quiz = QUIZ.lock().await;

        if quiz.answered {
            quiz.generate_question();

            let question = quiz.current_question.clone();
            let builder = CreateMessage::new().content(question);
            let msg = channel_id.send_message(&ctx.http, builder).await;
            if let Err(why) = msg {
                println!("Error sending question: {:?}", why);
            }
        }
    }
}
