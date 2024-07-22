use anyhow::Context as AnyContext;
use bot::{
    music::{beginloop, endloop, pause, play, queue, resume, skip, stop},
    Data,
};
use poise::{Framework, FrameworkOptions, PrefixFrameworkOptions};
use reqwest::Client as HttpClient;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use songbird::SerenityInit;
use tracing::info;

mod bot;
mod sources;

struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, _ctx: Context, _msg: Message) {}

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // rustls::crypto::ring::default_provider()
    //     .install_default()
    //     .expect("Failed to install rustls crypto provider"); 
    
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
        .event_handler(Bot)
        .framework(framework)
        .register_songbird()
        .type_map_insert::<HttpKey>(HttpClient::new())
        .await
        .expect("Err creating client");

    Ok(client.into())
}
