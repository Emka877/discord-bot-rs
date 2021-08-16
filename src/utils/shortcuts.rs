#![allow(dead_code)]

use serenity::model::channel::Embed;
use serenity::{model::id::ChannelId, prelude::*, utils::MessageBuilder};
use serenity::http::CacheHttp;

use crate::datastructs::CEmbedData;

pub async fn send(ctx: &Context, target_channel: ChannelId, reply: &mut MessageBuilder) -> () {
    send_or_forward_err(ctx, target_channel, reply).await;
}

pub async fn send_or_console_err(ctx: &Context, target_channel: ChannelId, reply: &mut MessageBuilder) -> () {
    if let Some(err) = send_or_forward_err(ctx, target_channel, reply).await {
        println!("Error: {}", err);
    }
}

pub async fn send_or_discord_err(ctx: &Context, target_channel: ChannelId, error_target_channel: ChannelId, reply: &mut MessageBuilder) -> () {
    if let Some(err) = send_or_forward_err(ctx, target_channel, reply).await {
        let error_message: String = format!("Erreur: {}", err);
        if let Err(err2) = error_target_channel.say(ctx.http(), error_message).await {
            println!("{}", err2);
        }
    }
}

pub async fn send_or_forward_err(
    ctx: &Context,
    target_channel: ChannelId,
    reply: &mut MessageBuilder
) -> Option<SerenityError> {
    if let Err(err) = target_channel.say(ctx.http(), reply.build()).await {
        return Some(err);
    }

    None
}

pub async fn send_embed_ignore_error(
    ctx: &Context,
    target_channel: ChannelId,
    data: CEmbedData,
) -> () {
    let _ = target_channel.send_message(&ctx.http(), |m| {
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
    });
}
