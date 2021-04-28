use serenity::{client::Context, framework::{standard::{CommandResult, macros::command}}, model::channel::Message};

#[command]
#[owners_only]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}

#[command]
pub async fn links(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(
        ctx,
        format!(
            "
        Twitch:
        - Star and Grey: https://www.twitch.tv/star_and_grey
        
        Youtube:
        - Grey Monster: https://www.youtube.com/channel/UCFsWs9C4oDm_JMtmpLFX7eQ
        - Emka: https://www.youtube.com/channel/UChUWneEkjNMqLNpp-vQ2DRQ"
        ),
    )
    .await?;
    Ok(())
}
