use poise::command;

use super::{Context, Error};

#[command(prefix_command, aliases("p"), slash_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Song to play"] args: String,
) -> Result<(), Error> {
    ctx.reply("play!").await?;
    Ok(())
}

#[command(prefix_command, slash_command)]
pub async fn pause(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("pause!").await?;
    Ok(())
}

#[command(prefix_command, slash_command)]
pub async fn resume(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("resume!").await?;
    Ok(())
}

#[command(prefix_command, slash_command)]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("stop!").await?;
    Ok(())
}

#[command(prefix_command, slash_command)]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error>{
    ctx.reply("skip!").await?;
    Ok(())
}

#[command(prefix_command, slash_command)]
pub async fn skipto(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("skipto!").await?;
    Ok(())
}

#[command(prefix_command, slash_command)]
pub async fn queue(
    ctx: Context<'_>,
    #[description = "Song to enqueue"] args: String,
) -> Result<(), Error> {
    ctx.reply("queue!").await?;
    Ok(())
}

#[command(prefix_command, aliases("loop"), slash_command)]
pub async fn beginloop(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("beginlooop!").await?;
    Ok(())
}

#[command(prefix_command, slash_command)]
pub async fn endloop(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("endloop!").await?;
    Ok(())
}
