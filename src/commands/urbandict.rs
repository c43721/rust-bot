use crate::{
    model::urbandictionary::{Definition, SearchStringResponse},
    Context, Error,
};

use poise::serenity_prelude::Colour;
use serenity::futures::{Stream, StreamExt};

const RANDOM_URL: &str = "https://api.urbandictionary.com/v0/random";
const DEFINITION_URL: &str = "https://api.urbandictionary.com/v0/define?term=";
const DEFINITION_ID_URL: &str = "https://api.urbandictionary.com/v0/define?defid=";

async fn get_defaults(ctx: Context<'_>) -> Vec<Definition> {
    let response = ctx
        .data()
        .http_client
        .get(RANDOM_URL)
        .send()
        .await
        .unwrap()
        .json::<SearchStringResponse>()
        .await
        .unwrap();

    response.list
}

async fn get_definitions_for_term(ctx: Context<'_>, term: String) -> Vec<Definition> {
    let url = format!("{}{}", DEFINITION_URL, term);

    let response = ctx
        .data()
        .http_client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<SearchStringResponse>()
        .await
        .unwrap();

    response.list
}

async fn get_definition_by_id(ctx: Context<'_>, id: String) -> Vec<Definition> {
    let url = format!("{}{}", DEFINITION_ID_URL, id);

    let response = ctx
        .data()
        .http_client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<SearchStringResponse>()
        .await
        .unwrap();

    response.list
}

async fn autocomplete_urbandict<'a>(
    _ctx: Context<'_>,
    search_term: &'a str,
) -> impl Stream<Item = poise::AutocompleteChoice<String>> + 'a {
    let terms = if search_term.is_empty() {
        get_defaults(_ctx).await
    } else {
        get_definitions_for_term(_ctx, search_term.to_string()).await
    };

    serenity::futures::stream::iter(terms).map(|def| poise::AutocompleteChoice {
        name: format!("{} ({})", def.word, def.thumbs_up),
        value: def.defid.to_string(),
    })
}

/// Look for a term on Urban Dictionary!!
#[poise::command(
    slash_command,
    rename = "urbandict",
    description_localized("en-US", "Search for a term on Urban Dictionary")
)]
pub async fn urban_dictionary(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_urbandict"]
    #[description = "The term to search for"]
    #[min = 1]
    #[max = 100]
    term: String,
) -> Result<(), Error> {
    let definitions = get_definition_by_id(ctx, term).await;

    let definition_to_send = definitions.first().unwrap();

    ctx.send(|m| {
        m.embed(|e| {
            e.title(format!(
                "{} (written by {})",
                definition_to_send.word, definition_to_send.author
            ))
            .color(Colour::BLUE)
            .description(&definition_to_send.definition)
            .url(&definition_to_send.permalink)
            .field("Example", &definition_to_send.example, false)
            .field("Thumbs Up", definition_to_send.thumbs_up.to_string(), true)
            .field(
                "Thumbs Down",
                definition_to_send.thumbs_down.to_string(),
                true,
            )
            .field("Defid", definition_to_send.defid.to_string(), true)
        })
    })
    .await?;

    Ok(())
}
