mod reddit_methods;

use std::fs::read_to_string;

use json::parse;

use serenity::{
    async_trait,
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework,
    },
    model::channel::Message,
    prelude::*,
};

#[group]
#[commands(reddit)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(
        parse(&read_to_string("./resources/auth.json").expect("Failed to read in auth file"))
            .expect("Failed to parse auth file")["discord_bot_secret"]
            .as_str()
            .unwrap(),
        intents,
    )
    .event_handler(Handler)
    .framework(framework)
    .await
    .expect("Failed to create client");

    if let Err(why) = client.start().await {
        println!(
            "An unrecoverable error has occurred in the application: {:?}",
            why
        );
    }
}

#[command]
async fn reddit(ctx: &Context, msg: &Message) -> CommandResult {
    match msg.content.split(" ").nth(1) {
        Some(sub_name) => match reddit_methods::get_random_post_url(sub_name).await {
            Some(response_url) => {
                msg.reply(ctx, response_url).await?;

                Ok(())
            }
            None => Ok(()),
        },
        None => match reddit_methods::get_random_post_url("all").await {
            Some(response_url) => {
                msg.reply(ctx, response_url).await?;

                Ok(())
            }
            None => Ok(()),
        },
    }
}
