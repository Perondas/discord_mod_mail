use crate::{
    commands::common::{get_guild, get_guild_channel, ok_or_respond},
    storage, Context, Error,
};

#[poise::command(slash_command, rename = "anonymous_message")]
/// Send a anonymous message to the moderation team (No one can see that you used this command)
pub async fn send_anonymous_message(ctx: Context<'_>, message: String) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

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
        m.content(format!("Anonymous message:\n{}", message))
    })
    .await?;

    ctx.say("Message sent").await?;

    Ok(())
}
