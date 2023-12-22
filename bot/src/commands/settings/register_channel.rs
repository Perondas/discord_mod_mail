use crate::{
    commands::common::{get_guild, ok_or_respond},
    storage, Context, Error,
};

/// Set the channel where the bot will send mail to
#[poise::command(slash_command, rename = "register_channel")]
pub async fn register_channel(
    ctx: Context<'_>,
    #[description = "Channel Id for update broadcast"] channel_id: Option<u64>,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let guild = get_guild!(ctx);

    let channel_id = match channel_id {
        Some(id) => id,

        None => ctx.channel_id().0,
    };

    if !guild.channels.iter().any(|c| c.0 .0 == channel_id) {
        ctx.say("Please provide a valid channel id.").await?;
        return Ok(());
    }

    let mut settings = ok_or_respond!(
        ctx,
        storage::read_settings(),
        "An error occurred while reading the settings."
    );

    settings.message_channel = Some(channel_id);

    ok_or_respond!(
        ctx,
        storage::write_settings(settings),
        "An error occurred while updating the channel."
    );

    ctx.say("Update channel set.").await?;

    Ok(())
}
