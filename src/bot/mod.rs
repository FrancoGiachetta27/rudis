use anyhow::Context as AnyContext;
use reqwest::Client as HttpClient;
use serenity::all::{ChannelId, GuildId};
use songbird::{typemap::TypeMapKey, Songbird};
use std::sync::Arc;

pub mod general;
pub mod music;

pub struct Bot;
pub struct Data {}
pub struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

/// returns the necessary info to manage bot's voice state
/// if the channel id cannot be obtained, then it returns nothing
pub async fn get_voice_manage_info(
    ctx: &Context<'_>,
) -> Option<(Arc<Songbird>, GuildId, ChannelId)> {
    let (guild_id, channel_id) = {
        let guild = ctx.guild().unwrap();

        let channel_id: Option<ChannelId> = guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|state| state.channel_id);

        (guild.id, channel_id)
    };
    let connect_to = match channel_id {
        Some(c) => c,
        None => {
            ctx.reply("You should be in a channel!").await.unwrap();

            return None;
        }
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .context("Songbird Voice client placed in at initialization")
        .unwrap()
        .clone();

    Some((manager, guild_id, connect_to))
}
