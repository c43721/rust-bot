mod commands;
use commands::*;

mod model;

use tracing::{error, instrument};

use poise::serenity_prelude as serenity;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// User data, which is stored and accessible in all command invocations
pub struct Data {
    http_client: reqwest::Client,
}

#[instrument]
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                register::register(),
                ping::ping(),
                urbandict::urban_dictionary(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                edit_tracker: Some(poise::EditTracker::for_timespan(
                    std::time::Duration::from_secs(3600),
                )),
                additional_prefixes: vec![poise::Prefix::Literal("hey bot,")],
                ..Default::default()
            },
            on_error: |err| {
                Box::pin(async move {
                    match err {
                        poise::FrameworkError::Command { ctx, .. } => {
                            error!(
                                "In on_error: {:?}",
                                ctx.invocation_data::<&str>().await.as_deref()
                            );
                        }
                        err => {
                            if let Err(e) = poise::builtins::on_error(err).await {
                                error!("Fatal error while sending error message: {}", e);
                            }
                        }
                    }
                })
            },
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("expected DISCORD_TOKEN"))
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .user_data_setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(Data {
                    http_client: reqwest::Client::new(),
                })
            })
        });

    framework.run().await.unwrap();
}
