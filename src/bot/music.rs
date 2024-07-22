use super::{get_voice_manage_info, Context, Error};
use poise::command;
use tracing::info;
use crate::sources;


/// play: finds a song on youtube and plays it (receives the song's name or a youtube's link)
#[command(prefix_command, aliases("p"), slash_command, guild_only)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Song to play"] args: Vec<String>,
) -> Result<(), Error> {
    if let Some((manager, guild_id, channel_id)) = get_voice_manage_info(&ctx).await {
        let song = args.join(" ");

        let handler = manager.join(guild_id, channel_id).await?;

        if let Some(source) = sources::get_from_yt(&ctx, song).await {
            info!("Source: {:?}", source);
            let mut handler_lock = handler.lock().await;
            let _ = handler_lock.play_input(source.clone().into());
        } else {
            ctx.reply("Could not get that song from youtube, check if it exist or is available")
                .await?;
        }
    } 

    Ok(())
}

/// puase: pauses the current playing track
#[command(prefix_command, slash_command)]
pub async fn pause(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("pause!").await?;
    Ok(())
}

/// resume: resumes the puased track, does nothing if there is noone
#[command(prefix_command, slash_command)]
pub async fn resume(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("resume!").await?;
    Ok(())
}

/// stop: stop the bot and cleans the queue
#[command(prefix_command, slash_command)]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("stop!").await?;
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
