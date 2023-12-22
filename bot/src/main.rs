use poise::{
    builtins,
    serenity_prelude::{self as serenity, Command},
};
use tracing::{debug, error, info};

use crate::commands::{
    actions::{send_anonymous_message::*, send_message::*},
    settings::register_channel::*,
};

mod commands;
mod storage;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, AppState, Error>;

#[derive(Clone)]
pub struct AppState {}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set");

    storage::create_settings().expect("Failed to create settings");

    let options = poise::FrameworkOptions {
        commands: vec![
            help(),
            register_channel(),
            send_message(),
            send_anonymous_message(),
        ],
        on_error: |error| Box::pin(on_error(error)),
        pre_command: |ctx| {
            Box::pin(async move {
                debug!("Executing command {}...", ctx.command().qualified_name);
            })
        },

        post_command: |ctx| {
            Box::pin(async move {
                debug!("Executed command {}!", ctx.command().qualified_name);
            })
        },
        ..Default::default()
    };

    let state = AppState {};

    let s = state.clone();

    let framework = poise::Framework::builder()
        .token(token)
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                let commands = builtins::create_application_commands(&framework.options().commands);
                Command::set_global_application_commands(ctx, |c| {
                    *c = commands;
                    c
                })
                .await
                .expect("Failed to set global application commands");
                Ok(s)
            })
        })
        .options(options)
        .intents(serenity::GatewayIntents::non_privileged())
        .build()
        .await
        .expect("Failed to create framework");

    info!("Starting bot");

    framework.start().await.expect("Failed to start framework");
}

/// Show this help menu
#[poise::command(slash_command)]
async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "\
You can limit availability of commands to specific users or roles trough the server settings.",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

async fn on_error(e: poise::FrameworkError<'_, AppState, Error>) {
    match e {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {error}"),
        poise::FrameworkError::Command { error, ctx } => {
            error!("Error in command `{}`: {error:?}", ctx.command().name,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                error!("Error while handling error: {}", e);
            }
        }
    }
}
