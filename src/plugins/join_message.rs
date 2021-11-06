use crate::constants::channels::LANDING_CHANNEL;
use serenity::model::id::ChannelId;
use serenity::utils::MessageBuilder;
use serenity::{client::Context, model::guild::Member};
use std::sync::Arc;
use crate::utils::shortcuts::send_or_console_err;

pub async fn send_join_message(ctx: Arc<Context>, member: Member) {
    let mut builder = MessageBuilder::new();
    builder.push_line("Welcome, ");
    builder.user(member.clone());
    builder.push_line("! Please write or private message me `!notabot` to confirm joining this server.");
    let landing_channel: ChannelId = ChannelId::from(LANDING_CHANNEL);
    send_or_console_err(&ctx, landing_channel, &mut builder);
}
