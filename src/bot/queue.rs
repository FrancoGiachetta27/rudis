use serenity::prelude::Mutex;
use songbird::Call;
use std::sync::Arc;

use super::sources;

use super::{Context, Error};

pub async fn enqueue_track(
    ctx: &Context<'_>,
    handler_lock: &Arc<Mutex<Call>>,
    track: String,
) -> Result<(), Error> {
    let mut handler = handler_lock.lock().await;

    if let Some(source) = sources::get_from_yt(&ctx, track).await {
        handler.enqueue_input(source.into()).await;
    } else {
        ctx.reply("Could not get that song from youtube, check if it exist or is available")
            .await?;
    }

    Ok(())
}
