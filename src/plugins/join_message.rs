use crate::constants::channels::LANDING_CHANNEL;
use serenity::model::id::ChannelId;
use serenity::utils::MessageBuilder;
use serenity::{client::Context, model::guild::Member};
use std::sync::Arc;

pub async fn send_join_message(ctx: Arc<Context>, member: Member) {
    let message = MessageBuilder::new()
        .push_line("Welcome, ")
        .user(member.clone())
        .push_line(
            "! Please write or private message me `!notabot` to confirm joining this server.",
        )
        .build();
    let landing_channel: ChannelId = ChannelId::from(LANDING_CHANNEL);
    if let Err(welcome_error) = landing_channel
        .send_message(&ctx.http, |m| {
            m.content(message);
            m
        })
        .await
    {
        eprintln!(
            "Error sending welcome message: {}",
            welcome_error.to_string()
        );
    }
}
