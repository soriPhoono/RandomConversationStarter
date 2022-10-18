mod reddit_methods;

use std::fs::read_to_string;

use serenity::{
    async_trait,
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework,
    },
    model::channel::Message,
    prelude::*,
};

use roux::Reddit;

#[group]
#[commands(reddit)]
struct General;

struct Handler;

impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("/"))
        .group(&GENERAL_GROUP);

    let token = read_to_string("./resources/auth.txt").expect("Failed to read in auth file");

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Failed to create client");

    if let Err(why) = client.start().await {
        println!("Failed to start client: {:?}", why);
    }
}

#[command]
async fn reddit(ctx: &Context, msg: &Message) -> CommandResult {
    let client = Reddit::new()

    Ok(())
}
