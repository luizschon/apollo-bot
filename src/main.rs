use std::env;

use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use poise::{Framework, FrameworkOptions};
use serenity::{all::ClientBuilder, prelude::*};

mod commands;

type Context<'a> = poise::Context<'a, Data, anyhow::Error>;

struct Data;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let options = FrameworkOptions {
        commands: vec![commands::ping()],
        ..Default::default()
    };

    let framework = Framework::builder()
        .setup(move |_ctx, ready, _framework| {
            Box::pin(async move {
                tracing::debug!("Logged in as {}", ready.user.name);
                Ok(Data)
            })
        })
        .options(options)
        .build();

    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in the environment");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .unwrap();

    if let Err(why) = client.start().await {
        tracing::error!("Client error: {why:?}");
    }
}
