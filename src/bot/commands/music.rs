use crate::bot::{get_voice_manage_info, queue, Context, Error};
use poise::command;
use tracing::{error, info};

/// play: finds a song on youtube and plays it (receives the song's name or a youtube's link)
#[command(prefix_command, aliases("p"), slash_command, guild_only)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Song to play"] args: Vec<String>,
) -> Result<(), Error> {
    if let Some((manager, guild_id, channel_id)) = get_voice_manage_info(&ctx).await {
        let track = args.join(" ");
        let handler_lock = manager.join(guild_id, channel_id).await?;

        queue::enqueue_track(&ctx, &handler_lock, track).await?;

        let handler = handler_lock.lock().await;
        let queue = handler.queue();

        queue.current().unwrap().play()?;
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

        if let Err(_) = queue.pause() {
            ctx.reply("There's no song being played right now").await?;
        };
    } else {
        ctx.reply("There's no song being played right now").await?;
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

        if queue.resume().is_err() {
            ctx.reply("There's no song in the queue right now or it is already being played")
                .await?;
        };
    }

    Ok(())
}

/// stop: stop the bot and cleans the queue
#[command(prefix_command, slash_command)]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    if let Some((manager, guild_id, _)) = get_voice_manage_info(&ctx).await {
        let handler_lock = manager.get(guild_id).unwrap();
        let handler = handler_lock.lock().await;
        let queue = handler.queue();

        queue.stop();
    }

    Ok(())
}

/// skip: drops the current playing track and plays the next one
#[command(prefix_command, slash_command)]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("skip!").await?;
    Ok(())
}

/// skipto: skips to the given queue position and plays the asociated track, dropping the others
#[command(prefix_command, slash_command)]
pub async fn skipto(ctx: Context<'_>, args: String) -> Result<(), Error> {
    ctx.reply("skipto!").await?;
    Ok(())
}

/// queue: enqueues the given song
#[command(prefix_command, slash_command)]
pub async fn queue(
    ctx: Context<'_>,
    #[description = "Song to enqueue"] args: String,
) -> Result<(), Error> {
    ctx.reply("queue!").await?;
    Ok(())
}

/// beginloop: starts a loop on the current playing track
#[command(prefix_command, aliases("loop"), slash_command)]
pub async fn beginloop(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("beginlooop!").await?;
    Ok(())
}

/// endloop: ends the loop, if there's one
#[command(prefix_command, slash_command)]
pub async fn endloop(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("endloop!").await?;
    Ok(())
}
