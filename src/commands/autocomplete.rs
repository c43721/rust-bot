use crate::{Context, Error};
use serenity::futures::{Stream, StreamExt};
use std::fmt::Write as _;

async fn autocomplete_name<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    serenity::futures::stream::iter(&["Amanda", "Bob", "Christian", "Danny", "Ester", "Falk"])
        .filter(move |name| serenity::futures::future::ready(name.starts_with(&partial)))
        .map(|name| name.to_string())
}

async fn autocomplete_number(
    _ctx: Context<'_>,
    _partial: &str,
) -> impl Iterator<Item = poise::AutocompleteChoice<u32>> {
    // Dummy choices
    [1_u32, 2, 3, 4, 5]
        .iter()
        .map(|&n| poise::AutocompleteChoice {
            name: format!(
                "{} (why did discord even give autocomplete choices separate labels)",
                n
            ),
            value: n,
        })
}

/// Greet a user. Showcasing autocomplete!
#[poise::command(slash_command)]
pub async fn greet(
    ctx: Context<'_>,
    #[description = "Who to greet"]
    #[autocomplete = "autocomplete_name"]
    name: String,
    #[description = "A number... idk I wanted to test number autocomplete"]
    #[autocomplete = "autocomplete_number"]
    number: Option<u32>,
) -> Result<(), Error> {
    let mut response = format!("Hello {}", name);
    if let Some(number) = number {
        let _ = write!(response, "#{}", number);
    }
    response += "!";

    ctx.say(response).await?;
    Ok(())
}