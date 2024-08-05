use crate::{
    bot::{
        get_voice_manage_info,
        queue::{self, SkipQuery},
        Context, Error,
    },
    utils::{create_embed_error, send_embed_message},
};
use poise::command;
use tracing::error;

/// play: finds a song on youtube and plays it (receives the song's name or a youtube's link)
#[command(prefix_command, aliases("p"), slash_command, guild_only)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Song to play"] args: Vec<String>,
) -> Result<(), Error> {
    if let Some((manager, guild_id, channel_id)) = get_voice_manage_info(&ctx).await {
        let track = args.join(" ");
        let handler_lock = manager.join(guild_id, channel_id).await?;

        if let Err(err) = queue::enqueue_track(&ctx, &handler_lock, track.clone()).await {
            error!("An error ocurred while enqueueing the track: {}", err);

            let embed = create_embed_error("An error acurred while adding the track to the queue");
            send_embed_message(&ctx, embed).await?;
        };

        let handler = handler_lock.lock().await;
        let queue = handler.queue();

        if let Err(e) = queue.current().unwrap().play() {
            error!("An error ocurred while playing the track {}", e);

            let embed = create_embed_error(&format!(
                "There was an error while trying to play the track {}",
                track.clone()
            ));
            send_embed_message(&ctx, embed).await?;
        };
    }

    Ok(())
}

/// puase: pauses the current playing track
#[command(prefix_command, guild_only, slash_command)]
pub async fn pause(ctx: Context<'_>) -> Result<(), Error> {
    if let Some((manager, guild_id, _)) = get_voice_manage_info(&ctx).await {
        let handler_lock = manager.get(guild_id).unwrap();
        let handler = handler_lock.lock().await;
        let queue = handler.queue();

        if let Err(e) = queue.pause() {
            error!("An error ocurred while pausing the track {}", e);

            let embed = create_embed_error("There's no song being played right now");
            send_embed_message(&ctx, embed).await?;
        }
    }

    Ok(())
}

/// resume: resumes the puased track, does nothing if there is noone
#[command(prefix_command, slash_command)]
pub async fn resume(ctx: Context<'_>) -> Result<(), Error> {
    if let Some((manager, guild_id, _)) = get_voice_manage_info(&ctx).await {
        let handler_lock = manager.get(guild_id).unwrap();
        let handler = handler_lock.lock().await;
        let queue = handler.queue();

        if let Err(e) = queue.resume() {
            error!("An error ocurred while resuming the track {}", e);

            let embed = create_embed_error(
                "There's no song in the queue right now or it is already being played",
            );
            send_embed_message(&ctx, embed).await?;
        }
    }

    Ok(())
}

/// stop: stop the bot and cleans the queue
#[command(prefix_command, slash_command)]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    if let Some((manager, guild_id, _)) = get_voice_manage_info(&ctx).await {
        let handler_lock = manager.get(guild_id).unwrap();
        let mut handler = handler_lock.lock().await;

        handler.stop();

        ctx.data().clean();
    }

    Ok(())
}

/// skip: drops the current playing track and plays the next one
#[command(prefix_command, slash_command)]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    if let Some((manager, guild_id, _)) = get_voice_manage_info(&ctx).await {
        let handler_lock = manager.get(guild_id).unwrap();

        if let Err(err) = queue::skip_song(&ctx, &handler_lock, SkipQuery::Front).await {
            error!("An error ocurred while skiping front: {}", err);

            let embed = create_embed_error("An error ocurred while skiping");
            send_embed_message(&ctx, embed).await?;
        };
    }

    Ok(())
}

/// skipto: skips to the given queue position and plays the asociated track, dropping the others
#[command(prefix_command, slash_command)]
pub async fn skipto(ctx: Context<'_>, args: u32) -> Result<(), Error> {
    if let Some((manager, guild_id, _)) = get_voice_manage_info(&ctx).await {
        let handler_lock = manager.get(guild_id).unwrap();

        {
            let handler = handler_lock.lock().await;

            handler.queue().pause()?;
        }

        if let Err(err) = queue::skip_song(&ctx, &handler_lock, SkipQuery::Index(args)).await {
            error!("An error ocurred while skiping to index {}: {}", args, err);

            let embed = create_embed_error("An error ocurred while skiping");
            send_embed_message(&ctx, embed).await?;
        }

        let handler = handler_lock.lock().await;
        let queue = handler.queue();

        if let Err(e) = queue.current().unwrap().play() {
            let current_track = ctx.data().get_track(0).unwrap();

            error!("An error ocurred while playing the track {}", e);

            let embed = create_embed_error(&format!(
                "There was an err while trying to play the track {}",
                current_track.title.unwrap()
            ));
            send_embed_message(&ctx, embed).await?;
        };

        ctx.data().pop_range(args);
    }

    Ok(())
}

/// queue: shows the current track queue
#[command(prefix_command, slash_command)]
pub async fn queue(ctx: Context<'_>) -> Result<(), Error> {
    if let Err(err) = queue::show_queue(&ctx).await {
        error!("An error ocurred while showing the queue: {}", err);

        let embed = create_embed_error("An error ocurred while trying to show the queue");
        send_embed_message(&ctx, embed).await?;
    }

    Ok(())
}

/// beginloop: starts a loop on the current playing track
#[command(prefix_command, aliases("loop"), slash_command)]
pub async fn beginloop(ctx: Context<'_>) -> Result<(), Error> {
    let embed = create_embed_error("loop!");
    send_embed_message(&ctx, embed).await?;

    Ok(())
}

/// endloop: ends the loop, if there's one
#[command(prefix_command, slash_command)]
pub async fn endloop(ctx: Context<'_>) -> Result<(), Error> {
    let embed = create_embed_error("endloop!");
    send_embed_message(&ctx, embed).await?;

    Ok(())
}
