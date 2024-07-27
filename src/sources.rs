use songbird::input::YoutubeDl;
use tracing::{error, info};

use crate::bot::{Context, HttpKey};

pub async fn get_from_yt(ctx: &Context<'_>, song: String) -> Option<YoutubeDl> {
    let client = {
        let data = ctx.serenity_context().data.read().await;

        match data.get::<HttpKey>().cloned() {
            Some(c) => c,
            None => {
                error!("Guaranteed to exist in the typemap.");
                return None
            }
        }
    };

    let source = if song.starts_with("https") || song.starts_with("http") {
        YoutubeDl::new(client, song)
    } else {
        YoutubeDl::new_search(client, song)
    };

    Some(source)
}
