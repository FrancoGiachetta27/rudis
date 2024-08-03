use serenity::futures::SinkExt;
use serenity::prelude::Mutex;
use songbird::input::Input;
use songbird::typemap::DebuggableStorage;
use songbird::Call;
use std::sync::Arc;
use tracing::{debug, info};

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

        info!("Added a new track to the queue: {:#?}", metadata);

        ctx.data().enqueue_track(metadata);

        handler.enqueue_input(input).await;
    } else {
        ctx.reply("Could not get that song from youtube, check if it exist or is available")
            .await?;
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
                    ctx.reply("The queue is currently shorter than the index provided".to_string())
                        .await?;
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
        ctx.reply("The track queue is currently empty").await?;
    }

    Ok(())
}

pub async fn show_queue(ctx: &Context<'_>) -> Result<(), Error> {
    let queue = ctx.data().get_queue();

    if queue.is_empty() {
        ctx.reply("The queue is empty!").await?;

        return Ok(())
    }
    
    for t in queue.iter() {
        let track = t.title.as_ref().unwrap();

        info!("TRACK {}", track);
    }

    Ok(())
}
