use crate::Context;

#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> anyhow::Result<()> {
    ctx.say("Pong!").await?;
    Ok(())
}
