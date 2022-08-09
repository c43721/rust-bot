mod commands;
use commands::*;

#[macro_use]
extern crate log;

use poise::serenity_prelude as serenity;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// User data, which is stored and accessible in all command invocations
pub struct Data {}

#[tokio::main]
async fn main() {
    env_logger::init();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![register::register(), ping::ping(), autocomplete::greet()],
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
                        err => poise::builtins::on_error(err).await.unwrap(),
                    }
                })
            },
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("expected DISCORD_TOKEN"))
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));

    framework.run().await.unwrap();
}
