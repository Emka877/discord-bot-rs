use serenity::{client::Context, framework::standard::{CommandResult, macros::command}, model::channel::Message, utils::MessageBuilder};
use std::fs;
use std::env::current_exe;
use chrono::offset::Utc;
use chrono::DateTime;
use serenity::model::id::{MessageId, ChannelId};

use crate::{constants::channels::ERRORS, datastructs::SanitizedMessage};
use crate::utils::shortcuts::send_or_discord_err;


#[command]
pub async fn version(ctx: &Context, msg: &Message) -> CommandResult {
    let exe = current_exe().unwrap();
    let metas = fs::metadata(exe).unwrap();
    let build_date: DateTime<Utc> = metas.created().unwrap().into();
    let build_tz = build_date + chrono::Duration::hours(2);

    msg.reply(
        ctx, 
        format!("\nAnna version {}\nBuilt on {}", 
            env!("CARGO_PKG_VERSION"),
            build_tz
        )
    ).await?;
    
    Ok(())
}

#[command]
#[aliases("move", movemsg)]
#[owners_only]
#[min_args(2)]
#[max_args(2)]
pub async fn move_message_manually(ctx: &Context, msg: &Message) -> CommandResult {
    // Need: Message id, target channel id
    let san: SanitizedMessage = msg.into();
    let src_channel_id: ChannelId = msg.channel_id;
    let msg_id_parsed: u64 = san.arguments.get(0).unwrap().parse::<u64>().unwrap();
    let message_id: MessageId = MessageId(msg_id_parsed);
    let message = ctx.cache.message(src_channel_id, message_id).await.unwrap();
    let chan_id_parsed: u64 = san.arguments.get(1).unwrap().parse::<u64>().unwrap();
    let target_channel_id: ChannelId = ChannelId(chan_id_parsed);

    // Check if source and target channels are diff
    if msg.channel_id == target_channel_id {
        return Ok(());
    }

    // Copy content
    let mut msg_builder: MessageBuilder = MessageBuilder::new();
    let content: String = message.content;
    msg_builder.push_line(content);

    // Delete
    let del_result = msg.delete(&ctx.http).await;

    if del_result.is_ok() {
        // Send to new channel
        let _ = send_or_discord_err(
            ctx, 
            target_channel_id, 
            ERRORS.into(), 
            &mut msg_builder
        ).await;
    }

    Ok(())
}