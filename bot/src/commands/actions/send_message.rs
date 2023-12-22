use crate::{
    commands::common::{get_guild, get_guild_channel, ok_or_respond},
    storage, Context, Error,
};

#[poise::command(slash_command, rename = "message")]
/// Send a message to the moderation team (No one can see that you used this command)
pub async fn send_message(ctx: Context<'_>, message: String) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let user = ctx.author().id;

    let settings = ok_or_respond!(
        ctx,
        storage::read_settings(),
        "An error occurred while reading the settings."
    );

    let channel_id = match settings.message_channel {
        Some(id) => id,

        None => {
            ctx.say("Nowhere to send").await?;
            return Ok(());
        }
    };

    let guild = get_guild!(ctx);

    let c = get_guild_channel!(ctx, guild, channel_id);

    c.send_message(ctx, |m| {
        m.content(format!("Message from <@{}>:\n{}", user.0, message))
    })
    .await?;

    ctx.say("Message sent").await?;

    Ok(())
}
