use anyhow::Context as AnyContext;
use poise::{Framework, FrameworkOptions, PrefixFrameworkOptions};
use reqwest::Client as HttpClient;
use rudis::bot::{
    music::{beginloop, endloop, pause, play, queue, resume, skip, stop},
    Data, HttpKey,
};
use serenity::all::GatewayIntents;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use songbird::SerenityInit;

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = secrets
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![
                play(),
                pause(),
                resume(),
                stop(),
                skip(),
                queue(),
                beginloop(),
                endloop(),
            ],
            prefix_options: PrefixFrameworkOptions {
                prefix: Some(secrets.get("PREFIX").context("'PREFIX', was not found")?),
                case_insensitive_commands: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                Ok(Data {})
            })
        })
        .build();

    let client = Client::builder(&token, intents)
        .framework(framework)
        .register_songbird()
        .type_map_insert::<HttpKey>(HttpClient::new())
        .await
        .expect("Err creating client");

    Ok(client.into())
}
