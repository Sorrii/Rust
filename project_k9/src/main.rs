mod helper;
use serenity::{
    async_trait,
    framework::standard::{
        macros::{command, group},
        {CommandResult, Configuration, StandardFramework},
    },
    model::channel::Message,
    prelude::*,
};
use std::env;

#[group]
#[commands(ping, quote)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new().group(&GENERAL_GROUP);
    framework.configure(Configuration::new().prefix(">"));

    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client!");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn quote(ctx: &Context, msg: &Message) -> CommandResult {
    use helper::get_random_quote;

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

    if let Some(random_string) = get_random_quote(strings) {
        msg.reply(ctx, random_string).await?;
    } else {
        msg.reply(ctx, "No quotes available!").await?;
    }

    Ok(())
}
