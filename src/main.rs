use std::env;

use dotenv::dotenv;
use lavalink_rs::{client::LavalinkClient, node::NodeBuilder, prelude::*};
use poise::serenity_prelude as serenity;
use poise::{Framework, FrameworkOptions};
use serenity::{all::ClientBuilder, prelude::*};
use songbird::SerenityInit;

mod commands;

type Context<'a> = poise::Context<'a, Data, anyhow::Error>;

struct Data {
    pub lavalink: LavalinkClient,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let options = FrameworkOptions {
        commands: vec![commands::ping(), commands::play()],
        ..Default::default()
    };

    let framework = Framework::builder()
        .options(options)
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(&ctx.http, &framework.options().commands)
                    .await?;

                let node_local = NodeBuilder {
                    hostname: format!(
                        "localhost:{}",
                        env::var("LAVALINK_PORT")
                            .expect("Expected LAVALINK_PORT in the environment")
                    ),
                    is_ssl: false,
                    events: Default::default(),
                    password: env::var("LAVALINK_PASS")
                        .expect("Expected LAVALINK_PASS in the environment"),
                    user_id: ctx.cache.current_user().id.into(),
                    session_id: None,
                };

                Ok(Data {
                    lavalink: LavalinkClient::new(
                        Default::default(),
                        vec![node_local],
                        NodeDistributionStrategy::round_robin(),
                    )
                    .await,
                })
            })
        })
        .build();

    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in the environment");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = ClientBuilder::new(token, intents)
        .register_songbird()
        .framework(framework)
        .await?;

    client.start().await?;
    Ok(())
}
