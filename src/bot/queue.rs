use serenity::prelude::Mutex;
use songbird::input::Input;
use songbird::Call;
use std::sync::Arc;
use tracing::{debug, info};

use crate::utils::{
    create_embed_error, create_multi_embed, create_simple_embed, create_track_embed,
    send_embed_message,
};

use super::sources;

use super::{Context, Error};

pub enum SkipQuery {
    Front,
    Index(u32),
}

pub async fn enqueue_track(
    ctx: &Context<'_>,
    handler_lock: &Arc<Mutex<Call>>,
    track: String,
) -> Result<(), Error> {
    let mut handler = handler_lock.lock().await;

    if let Some(source) = sources::get_from_yt(&ctx, track).await {
        let mut input: Input = source.into();
        let metadata = input.aux_metadata().await?;

        ctx.data().enqueue_track(metadata.clone());

        let title = metadata.title.unwrap();
        let description = "```css\n🎙️  New track added\n```";
        let author = ctx.author().name.clone();
        let artist = metadata.artist.unwrap();
        let thumbnail = metadata.thumbnail.unwrap();
        let url = metadata.source_url.clone().unwrap().clone();
        let duration = metadata.duration.unwrap();

        let embed =
            create_track_embed(title, description, author, artist, url, thumbnail, duration);

        send_embed_message(&ctx, embed).await?;

        handler.enqueue_input(input).await;
    } else {
        let embed = create_embed_error(
            "Could not get that song from youtube, check if it exist or is available",
        );
        send_embed_message(&ctx, embed).await?;
    }

    Ok(())
}

pub async fn skip_song(
    ctx: &Context<'_>,
    handler_lock: &Arc<Mutex<Call>>,
    query: SkipQuery,
) -> Result<(), Error> {
    let handler = handler_lock.lock().await;
    let queue = handler.queue();

    if !queue.is_empty() {
        let data = ctx.data();

        match query {
            SkipQuery::Front => {
                queue.skip()?;
                data.pop_track();
            }
            SkipQuery::Index(index) => {
                if (queue.len() as u32) < index {
                    let embed = create_embed_error(
                        "The queue is currently shorter than the index provided",
                    );
                    send_embed_message(&ctx, embed).await?;
                } else {
                    queue.modify_queue(|q| {
                        for _ in 0..index {
                            q.pop_front();
                        }
                    });
                }
            }
        }
    } else {
        let embed = create_embed_error("The track queue is currently empty");
        send_embed_message(&ctx, embed).await?;
    }

    Ok(())
}

pub async fn show_queue(ctx: &Context<'_>) -> Result<(), Error> {
    let queue = ctx.data().get_queue();

    if queue.is_empty() {
        let embed = create_embed_error("The queue is empty");
        send_embed_message(&ctx, embed).await?;

        return Ok(());
    }

    let fields = queue
        .iter()
        .enumerate()
        .map(|(i, track)| (i.to_string(), track.title.clone().unwrap(), true))
        .collect::<Vec<(String, String, bool)>>();

    let embed = create_multi_embed(fields);

    send_embed_message(&ctx, embed).await?;

    Ok(())
}
