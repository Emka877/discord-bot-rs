use chrono::offset::Utc;
use chrono::DateTime;
use serenity::model::id::{ChannelId, MessageId, RoleId};
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    utils::MessageBuilder,
};
use std::env::current_exe;
use std::fs;

use crate::constants::channels::{INFRARED, ZIGGURAT};
use crate::persistence;
use crate::persistence::mem::{self, get_sticky_id};
use crate::plugins::sticky_plugin::send_sticky_and_update_mem;
use crate::utils::apis::igdb::query_game_by_name;
use crate::utils::igdb::IGDBGameSearchResponseData;
use crate::utils::shortcuts::{delete_message, send_or_discord_err};
use crate::{constants::channels::ERRORS, datastructs::SanitizedMessage};

#[command]
pub async fn version(ctx: &Context, msg: &Message) -> CommandResult {
    let exe = current_exe().unwrap();
    let metas = fs::metadata(exe).unwrap();
    let build_date: DateTime<Utc> = metas.created().unwrap().into();
    let build_tz = build_date + chrono::Duration::hours(2);

    msg.reply(
        ctx,
        format!(
            "\nDiscord bot version {}\nBuilt on {}",
            env!("CARGO_PKG_VERSION"),
            build_tz
        ),
    )
    .await?;

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
    let message = ctx
        .http
        .get_message(src_channel_id.into(), message_id.into())
        .await
        .unwrap();
    let chan_id_parsed: u64 = san.arguments.get(1).unwrap().parse::<u64>().unwrap();
    let target_channel_id: ChannelId = ChannelId(chan_id_parsed);
    let original_poster_name: String = message.author.name.clone();

    // Check if source and target channels are diff
    if msg.channel_id == target_channel_id {
        return Ok(());
    }

    // Copy content
    let mut msg_builder: MessageBuilder = MessageBuilder::new();
    let content: String = message.content.clone();
    msg_builder.push_line(content);
    msg_builder.push_line(format!("(Original Poster: {})", original_poster_name));

    // Delete
    let del_result = message.delete(&ctx.http).await;

    if del_result.is_ok() {
        // Send to new channel
        let _ = send_or_discord_err(ctx, target_channel_id, ERRORS.into(), &mut msg_builder).await;
    }

    Ok(())
}

#[command]
#[aliases("notabot")]
pub async fn not_a_bot(ctx: &Context, msg: &Message) -> CommandResult {
    let infrared_role_id = RoleId::from(INFRARED);
    // let everyone_role_id = RoleId::from(EVERYONE);
    let guild_id = msg.guild_id.unwrap();
    let user = msg.author.clone();

    if let Ok(is_infrared) = user.has_role(&ctx.http, guild_id, infrared_role_id).await {
        if is_infrared {
            let _ = msg
                .reply_mention(
                    &ctx.http,
                    "You're already a confirmed member, congratulations.",
                )
                .await;
        } else {
            if let Err(role_error) = &ctx
                .http
                .add_member_role(
                    guild_id.into(),
                    user.id.into(),
                    infrared_role_id.into(),
                    Some("New member correctly input the code"),
                )
                .await
            {
                eprintln!("Cannot assign role to user: {}", role_error.to_string());
            }
            let _ = msg.reply_mention(&ctx.http, "You are now confirmed.").await;
        }
    }

    Ok(())
}

#[command]
pub async fn search(ctx: &Context, msg: &Message) -> CommandResult {
    let sani: SanitizedMessage = msg.into();
    let game_name: String = sani.args_single_line;
    let response: Result<IGDBGameSearchResponseData, reqwest::Error> =
        query_game_by_name(game_name).await;

    if response.is_ok() {
        let res_data: IGDBGameSearchResponseData = response.unwrap();
        let _ = msg.reply_mention(&ctx.http, res_data.to_string()).await;
    } else {
        eprintln!(
            "There was an issue searching for an IGDB game: {}",
            response.unwrap_err()
        );
    }

    Ok(())
}

#[command]
#[aliases("sticky")]
pub async fn set_sticky(ctx: &Context, msg: &Message) -> CommandResult {
    let sani: SanitizedMessage = msg.into();

    let mut msg_builder = MessageBuilder::new();
    msg_builder.push_bold("STICKY MESSAGE: ");
    msg_builder.push(sani.args_single_line.clone());
    send_sticky_and_update_mem(&ctx, ZIGGURAT.into(), &mut msg_builder).await;

    // Set the STICKY_MESSAGE to the message content
    mem::set_sticky(sani.args_single_line.clone());

    Ok(())
}

#[command]
#[aliases("unsticky")]
pub async fn clear_sticky(ctx: &Context, _msg: &Message) -> CommandResult {
    delete_message(&ctx, ZIGGURAT.into(), get_sticky_id()).await;
    mem::clear_sticky();

    Ok(())
}

#[command]
#[aliases("errorlog")]
#[aliases("getLastErrors")]
pub async fn get_errors_log(ctx: &Context, msg: &Message) -> CommandResult {
    let san: SanitizedMessage = msg.into();
    let args = san.arguments;
    let mut limit: i32 = 10;
    
    if let Some(lim) = args.get(0) {
        let parsed_limit = lim.parse::<i32>();
        if parsed_limit.is_ok() {
            limit = parsed_limit.unwrap();
        }
    }

    match persistence::edge::requests::get_latest_error_logs(limit).await {
        Ok(logs_opt) => {
            if logs_opt.is_some() {
                let logs = logs_opt.unwrap();
                let mut msg_builder: MessageBuilder = MessageBuilder::new();
                
                for log in logs.iter() {
                    msg_builder.push(format!("({} - {} | {}) {}", log.created_local, log.level.clone().unwrap_or(String::from("Unknown")), log.channel_name.clone().unwrap_or(String::from("No channel")), log.log));
                    let _ = msg.reply_mention(&ctx, msg_builder.build()).await;
                }
            }
        },
        Err(error) => {
            println!("{}", error);
        }
    }

    Ok(())
}
