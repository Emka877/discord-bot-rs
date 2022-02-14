#![allow(dead_code)]

use serenity::http::CacheHttp;
use serenity::model::channel::Message;
use serenity::{model::id::ChannelId, prelude::*, utils::MessageBuilder};
use serenity::model::id::{UserId, MessageId};

use crate::datastructs::CEmbedData;

pub async fn send_raw(ctx: &Context, target_channel: ChannelId, reply: &mut MessageBuilder) -> Result<Message, serenity::Error> {
    target_channel.say(&ctx.http, reply.build()).await
}

pub async fn send(ctx: &Context, target_channel: ChannelId, reply: &mut MessageBuilder) -> () {
    send_or_forward_err(ctx, target_channel, reply).await;
}

pub async fn send_or_console_err(
    ctx: &Context,
    target_channel: ChannelId,
    reply: &mut MessageBuilder,
) -> () {
    if let Some(err) = send_or_forward_err(ctx, target_channel, reply).await {
        println!("Error: {}", err);
    }
}

pub async fn send_or_discord_err(
    ctx: &Context,
    target_channel: ChannelId,
    error_target_channel: ChannelId,
    reply: &mut MessageBuilder,
) -> () {
    if let Some(err) = send_or_forward_err(ctx, target_channel, reply).await {
        let error_message: String = format!("Error: {}", err);
        if let Err(err2) = error_target_channel.say(ctx.http(), error_message).await {
            println!("{}", err2);
        }
    }
}

pub async fn send_or_forward_err(
    ctx: &Context,
    target_channel: ChannelId,
    reply: &mut MessageBuilder,
) -> Option<SerenityError> {
    let built = format!("{}", reply.build());
    if let Err(err) = target_channel.say(ctx.http(), built).await {
        return Some(err);
    }

    None
}

pub async fn send_embed_ignore_error(
    ctx: &Context,
    target_channel: ChannelId,
    data: CEmbedData,
) -> () {
    let _err = send_embed_or_forward_error(ctx, target_channel, data).await;
}

pub async fn send_embed_or_discord_error(
    ctx: &Context,
    target_channel: ChannelId,
    error_channel: ChannelId,
    data: CEmbedData,
) -> () {
    if let Some(error) = send_embed_or_forward_error(ctx, target_channel, data).await {
        if let Err(err2) = error_channel
            .say(&ctx.http(), format!("Error: {}", error))
            .await
        {
            println!("{}", err2);
        }
    }
}

pub async fn send_embed_or_console_error(
    ctx: &Context,
    target_channel: ChannelId,
    data: CEmbedData,
) -> () {
    if let Some(err) = send_embed_or_forward_error(ctx, target_channel, data).await {
        println!("{}", err);
    }
}

pub async fn send_embed_or_forward_error(
    ctx: &Context,
    target_channel: ChannelId,
    data: CEmbedData,
) -> Option<SerenityError> {
    if let Err(err) = target_channel
        .send_message(&ctx.http(), |m| {
            m.content(data.content.clone());
            m.tts(data.tts);

            m.embed(|me| {
                me.title(data.title.clone());
                me.description(data.description.clone());
                if data.thumbnail.is_some() {
                    me.thumbnail(data.thumbnail.unwrap());
                }
                me
            });

            m
        })
        .await
    {
        return Some(err);
    }

    None
}

pub async fn send_private_message_or_console_error(context: &Context, user_id: UserId, message_content: &mut MessageBuilder) -> () {
    let private_channel = user_id.create_dm_channel(&context.http).await;

    if private_channel.is_err() {
        eprintln!("Cannot create private channel: {}", private_channel.unwrap().to_string());
    } else {
        // Shadowing
        let private_channel = private_channel.unwrap();
        if let Err(send_result) = private_channel.send_message(&context.http, |m| {
            m.content(message_content.build());
            m
        }).await {
            eprintln!("Cannot send a private message: {}", send_result.to_string());
        }
    }
}

pub async fn delete_message(context: &Context, channel_id: ChannelId, message_id: MessageId) -> () {
    let message_object = context
        .http
        .get_message(channel_id.into(), message_id.into())
        .await
        .unwrap();
    let _ = message_object.delete(context.http()).await;
}