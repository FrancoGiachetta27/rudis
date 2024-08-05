use anyhow::Context as AnyContext;
use reqwest::Client as HttpClient;
use serenity::all::{ChannelId, GuildId};
use songbird::{typemap::TypeMapKey, Songbird};
use std::sync::Arc;

pub mod commands;
pub mod data;
pub mod queue;
pub mod sources;

use data::Data;

use crate::utils::{create_embed_error, send_embed_message};

pub struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type Command = poise::Command<Data, Error>;

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
            let embed = create_embed_error("You should be in a channel");
            send_embed_message(&ctx, embed).await.unwrap();

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
