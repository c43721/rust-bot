use poise::serenity_prelude as serenity;
use crate::{Context, Error};

#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    let user = ctx.author();

    if user.id != serenity::UserId(176457969465163776) {
        ctx.say("Nope.").await?;
        return Ok(());
    }

    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
